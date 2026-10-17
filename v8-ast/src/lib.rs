#![cfg_attr(docsrs, feature(doc_cfg))]

//! v8-ast — V8 抽象语法树 (AST) 体系微 crate
//!
//! 原版对应:
//! - `v8/src/ast/ast.h` / `ast.cc`: AST 节点全集 (60 类节点)、enum_dispatch 动态派发、AstVisitor 遍历器
//! - `v8/src/ast/ast-value-factory.h`: AstRawString, AstConsString, AstBigInt, AstValueFactory
//! - `v8/src/ast/modules.h` / `modules.cc`: 模块导入/导出请求与描述符
//! - `v8/src/ast/variables.h` / `scopes.h`: 作用域与变量定义
//!
//! 文件 → 模块映射 (脚本生成基线 + 修订层, 复跑脚本覆盖基线):
//! - ast.h + src/parsing/token.h → ast_generated.rs (tool/src/ast/ast.h.js: 节点枚举/操作符/Statement/Expression/Declaration 桥接)
//! - ast.h / ast.cc → ast.rs (节点结构体 + AstNode 统一枚举 + AstNodeFactory 含 zone 分配路径) + visitor.rs (AstVisitor)
//! - ast-value-factory.h/.cc → value.rs + value_generated.rs (tool/src/ast/ast-value-factory.h.js: 177 项字符串常量表)
//! - modules.h/.cc → modules.rs + modules_generated.rs (tool/src/ast/modules.h.js)
//! - variables.h/.cc → variables.rs + variables_generated.rs (tool/src/ast/variables.h.js: 空洞检测常量)
//! - scopes.h/.cc → scopes.rs (解析期语义: 声明/查找/分配/消解, ScopeId/VariableId arena)
//! - prettyprinter / ast-source-ranges / ast-traversal-visitor / ast-function-literal-id-reindexer → 第三阶段

mod ast_generated;
mod modules_generated;
mod value_generated;
mod variables_generated;
pub mod visitors_generated;

pub mod ast;
mod error;
pub mod modules;
pub mod scopes;
pub mod value;
pub mod variables;
pub mod visitor;

pub use ast::{
  AstAssignOp, AstBinaryOp, AstCompareOp, AstCountOp, AstNode, AstNodeBase, AstNodeTrait,
  AstNodeType, AstUnaryOp, CatchPrediction, ClassLiteralOptions, FUNCTION_LITERAL_ID_TOP_LEVEL,
  FUNCTION_LITERAL_ID_UNASSIGNED, FunctionLiteralOptions, FunctionSyntaxKind, PreparseData,
};
pub use ast_generated::{Declaration, Expression, Statement};
pub use error::{Error, Result};
pub use modules::SourceTextModuleDescriptor;
pub use scopes::{ScopeId, ScopeTree, UnresolvedRef, VariableId};
pub use v8_common::VariableMode;
pub use value::{AstBigInt, AstConsString, AstRawString, AstValue, AstValueFactory};
pub use value_generated::AstStringConstant;
pub use variables::Variable;
pub use variables_generated::HoleCheckState;
pub use visitor::AstVisitor;
pub use visitors_generated::{
  AstTraversalVisitor, DefaultAstTraversalVisitor, FunctionLiteralIdReindexer, SimpleAstPrinter,
  SourceRangeCollector, SourceRangeKind, ast_printer_skel, ast_visitor_skel, print_ast,
};
