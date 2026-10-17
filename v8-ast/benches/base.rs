//! v8-ast 核心评测操作 trait 与两侧实现 (genbench 步骤 2)
//!
//! 评测对象 (仅真实公开 API):
//! - 节点构造分配: `AstNodeFactory` 代表节点 (Literal/VariableProxy/BinaryOperation/
//!   NaryOperation/ExpressionStatement/Block) 的 Zone 驻留分配路径 (`alloc_node`)
//! - enum_dispatch 派发遍历: `AstVisitor` 全树访问固定形状合成表达式树
//!   (深度/分支参数化, 确定性生成)
//! - 节点尺寸/布局检查: `AstNodeTrait::node_type`/`base` + 类别谓词的分类扫描路径
//!
//! 对比实现 `bench-dyn-box`: 等价手写 `Box<dyn>` 虚派发路径 (对应原版 C++ 虚函数形态),
//! 体现 enum_dispatch 静态化收益。

#[cfg(feature = "bench-dyn-box")]
use std::hint::black_box;
use std::mem::size_of_val;

use v8_ast::{
  ast::{
    AstBinaryOp, AstNode, AstNodeBase, AstNodeFactory, AstNodeTrait, AstNodeType, EmptyStatement,
    HoleCheckMode, ThisExpression, VariableProxyBindingMode,
  },
  value::{AstRawString, AstValue},
  visitor::AstVisitor,
};
use v8_zone::{AccountingAllocator, Zone};

/// 合成树参数: (深度, 分支因子); 树形为满 b 叉树, 内部节点运算、叶子字面量/代理
pub const TREE_PARAMS: [(usize, usize); 2] = [(6, 2), (8, 3)];

/// Zone 节点分配批量
pub const ALLOC_BATCH: usize = 1000;

/// 本 crate 核心评测操作 (评测 id 基于 NAME 生成)
pub trait AstBench {
  /// 实现名, 评测 id 前缀
  const NAME: &'static str;

  /// 合成表达式树句柄, 两侧形态不同 (enum 树 / 虚指针树)
  type Tree;

  /// 代表节点构造分配: 在 Zone(v8-ast)/堆(Box<dyn>) 上分配 count 个代表性节点,
  /// 返回分配触动的字节数合计 (确定性)
  fn alloc_nodes(&self, count: usize) -> u64;

  /// 构造深度 depth、分支 branch 的确定性满 b 叉合成表达式树
  fn build_tree(&self, depth: usize, branch: usize) -> Self::Tree;

  /// AstVisitor/虚派发全树访问, 返回访问节点数 (确定性自校验)
  fn visit_tree(&self, tree: &Self::Tree) -> u64;

  /// 节点尺寸/布局检查扫描: 全树逐节点走 node_type/类别谓词/base 引用路径,
  /// 返回表达式类别内部节点计数 (确定性)
  fn layout_check(&self, tree: &Self::Tree) -> u64;
}

// ── v8-ast 实现 (enum_dispatch 静态派发) ──

/// v8-ast 公开 API 评测实现
pub struct V8AstBench;

/// 全树访问计数器
struct VisitCounter {
  visited: u64,
}

impl AstVisitor for VisitCounter {
  fn enter_node(&mut self, _node: &AstNode) -> bool {
    self.visited += 1;
    true
  }
}

/// 布局/类别检查器: 逐节点走 node_type + 类别谓词 + base 路径
struct LayoutChecker {
  inner_expr: u64,
}

impl AstVisitor for LayoutChecker {
  fn enter_node(&mut self, node: &AstNode) -> bool {
    // node_type 由 enum_dispatch 静态转发至各节点实现 (bit_field 解码)
    if node.node_type() == AstNodeType::FailureExpression {
      return true;
    }
    if node.is_expression() && !node.is_literal() {
      // base() 引用路径: position 读取
      if node.base().position >= 0 {
        self.inner_expr += 1;
      }
    }
    true
  }
}

impl AstBench for V8AstBench {
  const NAME: &'static str = "v8-ast";
  type Tree = AstNode;

  fn alloc_nodes(&self, count: usize) -> u64 {
    let allocator = AccountingAllocator::new();
    let zone = Zone::new(&allocator, "bench_zone");
    let factory = AstNodeFactory::with_zone(&zone);
    let name = AstRawString::from_utf8("x");
    let mut touched = 0u64;
    for i in 0..count {
      // 代表节点类型轮转: 表达式(字面量/代理/二元) + 语句(表达式语句/块)
      let node = match i % 5 {
        0 => AstNode::Literal(AstNodeFactory::new_literal(
          i as i32,
          AstValue::Smi(i as i32),
        )),
        1 => AstNode::VariableProxy(AstNodeFactory::new_variable_proxy(
          i as i32,
          name.clone(),
          VariableProxyBindingMode::Bound,
          HoleCheckMode::Elided,
        )),
        2 | 3 => {
          let left = AstNode::Literal(AstNodeFactory::new_literal(0, AstValue::Smi(1)));
          let right = AstNode::Literal(AstNodeFactory::new_literal(1, AstValue::Smi(2)));
          AstNode::BinaryOperation(AstNodeFactory::new_binary_operation(
            i as i32,
            AstBinaryOp::Add,
            left,
            right,
          ))
        }
        4 => AstNode::ExpressionStatement(AstNodeFactory::new_expression_statement(
          i as i32,
          AstNode::ThisExpression(ThisExpression {
            base: AstNodeBase::new(i as i32, AstNodeType::ThisExpression),
          }),
        )),
        _ => AstNode::Block(AstNodeFactory::new_block(
          i as i32,
          vec![AstNode::EmptyStatement(EmptyStatement {
            base: AstNodeBase::new(i as i32, AstNodeType::EmptyStatement),
          })],
          false,
        )),
      };
      touched += factory
        .alloc_node(node)
        .map_or(0, |n| size_of_val(&*n) as u64);
    }
    touched + zone.allocation_size() as u64
  }

  fn build_tree(&self, depth: usize, branch: usize) -> AstNode {
    build_ast_subtree(0, depth, branch)
  }

  fn visit_tree(&self, tree: &AstNode) -> u64 {
    let mut counter = VisitCounter { visited: 0 };
    counter.visit(tree);
    counter.visited
  }

  fn layout_check(&self, tree: &AstNode) -> u64 {
    let mut checker = LayoutChecker { inner_expr: 0 };
    checker.visit(tree);
    checker.inner_expr
  }
}

/// 确定性合成子树: depth==0 为叶子 (字面量/变量代理轮转), 内部节点按 branch
/// 选 BinaryOperation(2) / NaryOperation(3+)
fn build_ast_subtree(pos: i32, depth: usize, branch: usize) -> AstNode {
  if depth == 0 {
    return if pos % 2 == 0 {
      AstNode::Literal(AstNodeFactory::new_literal(
        pos,
        AstValue::Number(pos as f64),
      ))
    } else {
      AstNode::VariableProxy(AstNodeFactory::new_variable_proxy(
        pos,
        AstRawString::from_utf8("v"),
        VariableProxyBindingMode::Unbound,
        HoleCheckMode::Elided,
      ))
    };
  }
  let child_base = pos * branch as i32 + 1;
  if branch <= 2 {
    let left = build_ast_subtree(child_base, depth - 1, branch);
    let right = build_ast_subtree(child_base + 1, depth - 1, branch);
    AstNode::BinaryOperation(AstNodeFactory::new_binary_operation(
      pos,
      AstBinaryOp::Add,
      left,
      right,
    ))
  } else {
    let first = build_ast_subtree(child_base, depth - 1, branch);
    let subsequent = (2..branch as i32)
      .map(|k| build_ast_subtree(child_base + k, depth - 1, branch))
      .collect();
    AstNode::NaryOperation(AstNodeFactory::new_nary_operation(
      pos,
      AstBinaryOp::Add,
      first,
      subsequent,
    ))
  }
}

// ── 对比实现: Box<dyn> 虚派发手写路径 (feature bench-dyn-box) ──

#[cfg(feature = "bench-dyn-box")]
pub struct DynBoxBench;

/// 节点类别 (对应原版 AstNodeType 虚查询的手写等价)
#[cfg(feature = "bench-dyn-box")]
#[derive(Clone, Copy, PartialEq)]
pub enum DynKind {
  Literal,
  Proxy,
  Binary,
  Nary,
}

/// 虚节点接口: 对应原版 AstNode 虚函数形态 (vtable 派发)
#[cfg(feature = "bench-dyn-box")]
pub trait DynNode {
  /// 虚派发子节点遍历 (对应 walk_node 虚分发)
  fn walk(&self, visitor: &mut dyn DynVisitor);
  /// 类别虚查询 (对应 node_type 虚函数)
  fn kind(&self) -> DynKind;
  /// 源码位置虚读取 (对应 position 虚函数)
  fn position(&self) -> i32;
}

/// 虚访问者 (对应 AstVisitor 的 dyn 形态)
#[cfg(feature = "bench-dyn-box")]
pub trait DynVisitor {
  fn visit_node(&mut self, node: &dyn DynNode);
}

#[cfg(feature = "bench-dyn-box")]
pub struct DynBinary {
  pub pos: i32,
  pub children: Vec<Box<dyn DynNode>>,
}

#[cfg(feature = "bench-dyn-box")]
pub struct DynLiteral {
  pub pos: i32,
  pub num: f64,
}

#[cfg(feature = "bench-dyn-box")]
pub struct DynProxy {
  pub pos: i32,
  pub name: String,
}

#[cfg(feature = "bench-dyn-box")]
impl DynNode for DynBinary {
  fn walk(&self, visitor: &mut dyn DynVisitor) {
    visitor.visit_node(self);
    for child in &self.children {
      child.walk(visitor);
    }
  }
  fn kind(&self) -> DynKind {
    if self.children.len() == 2 {
      DynKind::Binary
    } else {
      DynKind::Nary
    }
  }
  fn position(&self) -> i32 {
    self.pos
  }
}

#[cfg(feature = "bench-dyn-box")]
impl DynNode for DynLiteral {
  fn walk(&self, visitor: &mut dyn DynVisitor) {
    // 载荷保活 (与 v8-ast 侧 Literal.value 驻留等价)
    black_box(self.num);
    visitor.visit_node(self);
  }
  fn kind(&self) -> DynKind {
    DynKind::Literal
  }
  fn position(&self) -> i32 {
    self.pos
  }
}

#[cfg(feature = "bench-dyn-box")]
impl DynNode for DynProxy {
  fn walk(&self, visitor: &mut dyn DynVisitor) {
    black_box(&self.name);
    visitor.visit_node(self);
  }
  fn kind(&self) -> DynKind {
    DynKind::Proxy
  }
  fn position(&self) -> i32 {
    self.pos
  }
}

#[cfg(feature = "bench-dyn-box")]
struct DynVisitCounter {
  visited: u64,
}

#[cfg(feature = "bench-dyn-box")]
impl DynVisitor for DynVisitCounter {
  fn visit_node(&mut self, _node: &dyn DynNode) {
    self.visited += 1;
  }
}

#[cfg(feature = "bench-dyn-box")]
struct DynLayoutChecker {
  inner_expr: u64,
}

#[cfg(feature = "bench-dyn-box")]
impl DynVisitor for DynLayoutChecker {
  fn visit_node(&mut self, node: &dyn DynNode) {
    let kind = node.kind();
    let is_inner = kind == DynKind::Binary || kind == DynKind::Nary;
    if is_inner && node.position() >= 0 {
      self.inner_expr += 1;
    }
  }
}

/// 确定性合成虚子树 (与 build_ast_subtree 同形)
#[cfg(feature = "bench-dyn-box")]
fn build_dyn_subtree(pos: i32, depth: usize, branch: usize) -> Box<dyn DynNode> {
  if depth == 0 {
    return if pos % 2 == 0 {
      Box::new(DynLiteral {
        pos,
        num: pos as f64,
      })
    } else {
      Box::new(DynProxy {
        pos,
        name: "v".to_string(),
      })
    };
  }
  let child_base = pos * branch as i32 + 1;
  let children = (0..branch as i32)
    .map(|k| build_dyn_subtree(child_base + k, depth - 1, branch))
    .collect();
  Box::new(DynBinary { pos, children })
}

#[cfg(feature = "bench-dyn-box")]
impl AstBench for DynBoxBench {
  const NAME: &'static str = "dyn-box";
  type Tree = Box<dyn DynNode>;

  fn alloc_nodes(&self, count: usize) -> u64 {
    let mut touched = 0u64;
    for i in 0..count {
      // 与 V8AstBench 同构的代表节点轮转, 分配落堆 (Box::new)
      let node: Box<dyn DynNode> = match i % 6 {
        0 => Box::new(DynLiteral {
          pos: i as i32,
          num: i as f64,
        }),
        1 => Box::new(DynProxy {
          pos: i as i32,
          name: "x".to_string(),
        }),
        2 | 3 => Box::new(DynBinary {
          pos: i as i32,
          children: vec![
            Box::new(DynLiteral { pos: 0, num: 1.0 }),
            Box::new(DynLiteral { pos: 1, num: 2.0 }),
          ],
        }),
        4 => Box::new(DynBinary {
          pos: i as i32,
          children: vec![Box::new(DynLiteral { pos: 0, num: 0.0 })],
        }),
        _ => Box::new(DynBinary {
          pos: i as i32,
          children: vec![Box::new(DynProxy {
            pos: i as i32,
            name: "s".to_string(),
          })],
        }),
      };
      touched += size_of_val(&*node) as u64 + size_of::<DynBinary>() as u64;
      black_box(node);
    }
    touched
  }

  fn build_tree(&self, depth: usize, branch: usize) -> Box<dyn DynNode> {
    build_dyn_subtree(0, depth, branch)
  }

  fn visit_tree(&self, tree: &Box<dyn DynNode>) -> u64 {
    let mut counter = DynVisitCounter { visited: 0 };
    tree.walk(&mut counter);
    counter.visited
  }

  fn layout_check(&self, tree: &Box<dyn DynNode>) -> u64 {
    let mut checker = DynLayoutChecker { inner_expr: 0 };
    tree.walk(&mut checker);
    checker.inner_expr
  }
}
