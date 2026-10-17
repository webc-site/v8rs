//! JavaScript 递归下降与 Pratt 表达式语法分析器 (Parser)
//!
//! 零拷贝 Token 所有权转移模式：关键字与符号原地 skip 跳过，
//! 标识符、字符串与数值字面量利用 std::mem::replace 就地 take 移入 AST 节点，彻底杜绝 Token 复制

use std::{
  mem::{replace, take},
  sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
  },
};

use itoa::Buffer;
use v8_numbers::conversions::double_to_string;

use crate::{
  error::{Error, Result},
  runtime::{
    ast::*,
    lexer::{Token, TokenKind},
    value::{JsValue, SLICE_KEY, SharedFunctionInfo},
  },
};

pub struct Parser<'a> {
  tokens: Vec<Token>,
  cursor: usize,
  /// 词法用的同一段源码; 函数 toString 靠 token 偏移在此切出原文 (#23 ⑤)
  source: &'a str,
  /// break/continue 解析期校验上下文 (同原版 parser 解析标签目标): 活跃标签 → 是否迭代语句
  ctrl_labels: Vec<(String, bool)>,
  /// 裸 break/continue 合法圈层: 迭代语句深度 (continue 与 break 共用) 与 switch 深度 (仅 break)
  loop_depth: usize,
  switch_depth: usize,
}

impl<'a> Parser<'a> {
  pub fn new(tokens: Vec<Token>, source: &'a str) -> Self {
    Self {
      tokens,
      cursor: 0,
      source,
      ctrl_labels: Vec::new(),
      loop_depth: 0,
      switch_depth: 0,
    }
  }

  #[inline]
  fn peek(&self) -> &TokenKind {
    if self.cursor < self.tokens.len() {
      &self.tokens[self.cursor].kind
    } else {
      &TokenKind::Eof
    }
  }

  /// 向后第 n 个 token (0 = peek)。accessor 的 `get`/`set` 是上下文关键字, 靠后一个 token 判定
  #[inline]
  fn peek_at(&self, n: usize) -> &TokenKind {
    self
      .tokens
      .get(self.cursor + n)
      .map_or(&TokenKind::Eof, |t| &t.kind)
  }

  #[inline]
  fn skip(&mut self) {
    if self.cursor < self.tokens.len() {
      self.cursor += 1;
    }
  }

  /// 函数语法的原文切片 (#23 ⑤): `start` 是 `function`/方法名/`get` 记号的起始偏移,
  /// 终点取刚消费的闭合 `}` 之后一个字节 —— token 起点恒在字符边界上, 非 ASCII 标识名也安全
  fn slice_source(&self, start: usize) -> String {
    let end = self.tokens[self.cursor.saturating_sub(1)].pos + 1;
    self.source[start.min(end)..end].to_string()
  }

  #[inline]
  fn take(&mut self) -> TokenKind {
    if self.cursor < self.tokens.len() {
      let kind = replace(&mut self.tokens[self.cursor].kind, TokenKind::Eof);
      self.cursor += 1;
      kind
    } else {
      TokenKind::Eof
    }
  }

  #[inline]
  fn match_token(&mut self, expected: &TokenKind) -> bool {
    if self.peek() == expected {
      self.skip();
      true
    } else {
      false
    }
  }

  #[inline]
  fn expect(&mut self, expected: &TokenKind) -> Result<()> {
    if self.peek() == expected {
      self.skip();
      Ok(())
    } else {
      let (line, col) = if self.cursor < self.tokens.len() {
        (self.tokens[self.cursor].line, self.tokens[self.cursor].col)
      } else {
        (0, 0)
      };
      Err(Error::ParseError(format!(
        "Expected token {:?}, found {:?} at line {line}:{col}",
        expected,
        self.peek()
      )))
    }
  }

  /// 上下文关键字 `of`: 词法上是普通标识符, 只在 for-of 头位判定 (17 轮)
  #[inline]
  fn at_contextual_of(&self) -> bool {
    matches!(self.peek(), TokenKind::Identifier(k) if k == "of")
  }

  fn expect_contextual_of(&mut self) -> Result<()> {
    if self.at_contextual_of() {
      self.skip();
      Ok(())
    } else {
      Err(Error::ParseError(format!(
        "Expected 'of' in for-of head, found {:?}",
        self.peek()
      )))
    }
  }

  pub fn parse_program(&mut self) -> Result<Vec<Stmt>> {
    let mut stmts = Vec::new();
    while self.peek() != &TokenKind::Eof {
      stmts.push(self.parse_statement()?);
    }
    // 解构暂态在出口单点展开 (与 parse_block 同钩子, 两链只见普通节点)
    Ok(desugar_stmts(stmts))
  }

  /// 语句解析单点入口: 标记语句识别 + 循环/switch 圈层计数 (break/continue 解析期校验),
  /// 其余委派 parse_statement_inner (原版 parser 在消费语句关键字时同步 push 可中断上下文)
  fn parse_statement(&mut self) -> Result<Stmt> {
    if matches!(self.peek(), TokenKind::Identifier(_)) && self.peek_at(1) == &TokenKind::Colon {
      return self.parse_labeled_statement();
    }
    let is_loop = matches!(
      self.peek(),
      TokenKind::For | TokenKind::Do | TokenKind::While
    );
    let is_switch = matches!(self.peek(), TokenKind::Switch);
    if is_loop {
      self.loop_depth += 1;
    }
    if is_switch {
      self.switch_depth += 1;
    }
    let r = self.parse_statement_inner();
    if is_loop {
      self.loop_depth -= 1;
    }
    if is_switch {
      self.switch_depth -= 1;
    }
    r
  }

  /// `L1: L2: stmt` — 连续标签收拢成一条 Labeled 节点 (同原版 labels 头链);
  /// 标签对体内 break/continue 可见, 出体即弹栈
  fn parse_labeled_statement(&mut self) -> Result<Stmt> {
    let start = self.ctrl_labels.len();
    let mut labels = Vec::new();
    while let TokenKind::Identifier(name) = self.peek() {
      if self.peek_at(1) != &TokenKind::Colon {
        break;
      }
      labels.push(name.clone());
      self.skip();
      self.skip();
    }
    // 标签是否指迭代语句由冒号链后的头记号定 (continue L 校验用, 与原版解析期解析 target 同口径)
    let is_iteration = matches!(
      self.peek(),
      TokenKind::For | TokenKind::Do | TokenKind::While
    );
    for l in &labels {
      self.ctrl_labels.push((l.clone(), is_iteration));
    }
    let body = Box::new(self.parse_statement()?);
    self.ctrl_labels.truncate(start);
    Ok(Stmt::Labeled { labels, body })
  }

  /// break/continue 后的可选标签: 同行标识符才取 (原版 [no LineTerminator here] 限制,
  /// 换行即按 ASI 断句, 标签不再属于本语句)
  fn take_label(&mut self, kw_line: usize) -> Result<Option<String>> {
    if let TokenKind::Identifier(name) = self.peek()
      && self.tokens[self.cursor.min(self.tokens.len() - 1)].line == kw_line
    {
      let name = name.clone();
      self.skip();
      return Ok(Some(name));
    }
    Ok(None)
  }

  /// 函数体是 break/continue/label 的词法边界: 内层不能指外层目标 (原版同口径 SyntaxError)
  fn parse_function_block(&mut self) -> Result<Vec<Stmt>> {
    let labels = take(&mut self.ctrl_labels);
    let (ld, sd) = (self.loop_depth, self.switch_depth);
    self.loop_depth = 0;
    self.switch_depth = 0;
    let r = self.parse_block();
    self.ctrl_labels = labels;
    self.loop_depth = ld;
    self.switch_depth = sd;
    r
  }

  fn parse_statement_inner(&mut self) -> Result<Stmt> {
    match self.peek() {
      TokenKind::Var | TokenKind::Let | TokenKind::Const => {
        self.skip();
        // 纯标识符名册走原形 (VarDecl/VarDecls, 两链既有形态零扰动);
        // 一旦混入解构模式, 已积累的普通声明冲刷成单条 VarDecl, 逐条拆成语句序列
        let mut decls: Vec<(String, Option<Expr>)> = Vec::new();
        let mut stmts: Vec<Stmt> = Vec::new();
        loop {
          let pat = if matches!(self.peek(), TokenKind::LBracket | TokenKind::LBrace) {
            Some(self.parse_pattern()?)
          } else {
            None
          };
          let name = match &pat {
            Some(_) => None,
            None => match self.take() {
              TokenKind::Identifier(s) => Some(s),
              other => {
                return Err(Error::ParseError(format!(
                  "Expected identifier after var/let/const, got {other:?}"
                )));
              }
            },
          };
          let init = if self.match_token(&TokenKind::Equal) {
            Some(self.parse_expression(2)?)
          } else {
            None
          };
          match name {
            Some(n) if stmts.is_empty() => decls.push((n, init)),
            Some(n) => stmts.push(Stmt::VarDecl { name: n, init }),
            None => {
              if stmts.is_empty() {
                for (n, i) in take(&mut decls) {
                  stmts.push(Stmt::VarDecl { name: n, init: i });
                }
              }
              stmts.push(Stmt::VarPattern {
                pat: pat.unwrap(),
                init,
              });
            }
          }
          if !self.match_token(&TokenKind::Comma) {
            break;
          }
        }
        let _ = self.match_token(&TokenKind::Semicolon);
        if stmts.is_empty() {
          if decls.len() == 1 {
            let (name, init) = decls.pop().unwrap_or_default();
            Ok(Stmt::VarDecl { name, init })
          } else {
            Ok(Stmt::VarDecls(decls))
          }
        } else {
          for (n, i) in take(&mut decls) {
            stmts.push(Stmt::VarDecl { name: n, init: i });
          }
          Ok(splat(stmts))
        }
      }
      TokenKind::Function => {
        let start = self.tokens[self.cursor].pos;
        self.skip();
        let name = match self.take() {
          TokenKind::Identifier(s) => s,
          other => {
            return Err(Error::ParseError(format!(
              "Expected function name, got {other:?}"
            )));
          }
        };
        self.expect(&TokenKind::LParen)?;
        let (params, prelude, rest) = self.parse_param_table()?;
        let mut body = self.parse_function_block()?;
        if !prelude.is_empty() {
          body = splice_prelude(prelude, body);
        }
        let src = Some(self.slice_source(start));
        let shared = Arc::new(SharedFunctionInfo::new_with_src(
          Some(name),
          params,
          body,
          src,
          rest,
        ));
        Ok(Stmt::FunctionDecl(shared))
      }
      TokenKind::Return => {
        self.skip();
        let expr = if self.peek() != &TokenKind::Semicolon && self.peek() != &TokenKind::RBrace {
          Some(self.parse_expression(0)?)
        } else {
          None
        };
        let _ = self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::Return(expr))
      }
      TokenKind::If => {
        self.skip();
        self.expect(&TokenKind::LParen)?;
        let cond = self.parse_expression(0)?;
        self.expect(&TokenKind::RParen)?;
        let then_branch = Box::new(self.parse_statement()?);
        let else_branch = if self.match_token(&TokenKind::Else) {
          Some(Box::new(self.parse_statement()?))
        } else {
          None
        };
        Ok(Stmt::If {
          cond,
          then_branch,
          else_branch,
        })
      }
      TokenKind::Semicolon => {
        self.skip();
        Ok(Stmt::Empty)
      }
      TokenKind::Do => {
        self.skip();
        let body = Box::new(self.parse_statement()?);
        self.expect(&TokenKind::While)?;
        self.expect(&TokenKind::LParen)?;
        let cond = self.parse_expression(0)?;
        self.expect(&TokenKind::RParen)?;
        let _ = self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::DoWhile { body, cond })
      }
      TokenKind::While => {
        self.skip();
        self.expect(&TokenKind::LParen)?;
        let cond = self.parse_expression(0)?;
        self.expect(&TokenKind::RParen)?;
        let body = Box::new(self.parse_statement()?);
        Ok(Stmt::While { cond, body })
      }
      TokenKind::For => {
        self.skip();
        self.expect(&TokenKind::LParen)?;
        if self.peek() == &TokenKind::Var
          || self.peek() == &TokenKind::Let
          || self.peek() == &TokenKind::Const
        {
          let is_var = true;
          self.skip();
          // for-of 解构声明头 (17 轮): `for (const [a,b] of …)` / `for (let {x} of …)`
          if matches!(self.peek(), TokenKind::LBracket | TokenKind::LBrace) {
            let pat = self.parse_pattern()?;
            self.expect_contextual_of()?;
            let src = self.parse_expression(0)?;
            self.expect(&TokenKind::RParen)?;
            let body = Box::new(self.parse_statement()?);
            return Ok(Stmt::ForOf {
              head: ForOfHead::Pattern(pat),
              src,
              body,
            });
          }
          let mut decls = Vec::new();
          let var_name = match self.take() {
            TokenKind::Identifier(id) => id,
            other => {
              return Err(Error::ParseError(format!(
                "Expected loop variable, got {other:?}"
              )));
            }
          };
          if self.match_token(&TokenKind::In) {
            let object = self.parse_expression(0)?;
            self.expect(&TokenKind::RParen)?;
            let body = Box::new(self.parse_statement()?);
            return Ok(Stmt::ForIn {
              variable: var_name,
              is_var,
              object,
              body,
            });
          }
          // for-of 简单声明绑定 `for (var x of a)` (17 轮)
          if self.at_contextual_of() {
            self.skip();
            let src = self.parse_expression(0)?;
            self.expect(&TokenKind::RParen)?;
            let body = Box::new(self.parse_statement()?);
            return Ok(Stmt::ForOf {
              head: ForOfHead::Binding(var_name),
              src,
              body,
            });
          }
          let init_val = if self.match_token(&TokenKind::Equal) {
            Some(self.parse_expression(2)?)
          } else {
            None
          };
          decls.push((var_name, init_val));
          while self.match_token(&TokenKind::Comma) {
            let next_name = match self.take() {
              TokenKind::Identifier(id) => id,
              other => {
                return Err(Error::ParseError(format!(
                  "Expected identifier in for-loop variable declaration, got {other:?}"
                )));
              }
            };
            let next_init = if self.match_token(&TokenKind::Equal) {
              Some(self.parse_expression(2)?)
            } else {
              None
            };
            decls.push((next_name, next_init));
          }
          self.expect(&TokenKind::Semicolon)?;
          let cond = if self.peek() == &TokenKind::Semicolon {
            None
          } else {
            Some(self.parse_expression(0)?)
          };
          self.expect(&TokenKind::Semicolon)?;
          let step = if self.peek() == &TokenKind::RParen {
            None
          } else {
            Some(self.parse_expression(0)?)
          };
          self.expect(&TokenKind::RParen)?;
          let body = Box::new(self.parse_statement()?);
          let init_stmt = if decls.len() == 1 {
            let (name, init) = decls.pop().unwrap();
            Stmt::VarDecl { name, init }
          } else {
            Stmt::VarDecls(decls)
          };
          return Ok(Stmt::For {
            init: Some(Box::new(init_stmt)),
            cond,
            step,
            body,
          });
        }
        let init = if self.peek() == &TokenKind::Semicolon {
          self.skip();
          None
        } else if matches!(self.peek(), TokenKind::LBracket | TokenKind::LBrace)
          && self.looks_like_destructure_for_head()
        {
          // for-of 无声明赋值模式头 (17 轮): `for ([a,b] of …)` / `for ({x} of …)` ——
          // 配平扫描后跟 `of` 判型, 模式复用 parse_pattern 单点
          let pat = self.parse_pattern()?;
          self.skip(); // 上下文关键字 `of` (扫描已确认)
          let src = self.parse_expression(0)?;
          self.expect(&TokenKind::RParen)?;
          let body = Box::new(self.parse_statement()?);
          return Ok(Stmt::ForOf {
            head: ForOfHead::Pattern(pat),
            src,
            body,
          });
        } else {
          let init_expr = self.parse_expression(0)?;
          if let Expr::Identifier(var_name) = &init_expr
            && self.match_token(&TokenKind::In)
          {
            let object = self.parse_expression(0)?;
            self.expect(&TokenKind::RParen)?;
            let body = Box::new(self.parse_statement()?);
            return Ok(Stmt::ForIn {
              variable: var_name.clone(),
              is_var: false,
              object,
              body,
            });
          }
          // for-of 表达式头 (17 轮): `for (p of a)` 复用既有绑定 / `for (o.p of a)` 成员写位
          if self.at_contextual_of() {
            self.skip();
            let head = match init_expr {
              Expr::Identifier(n) => ForOfHead::Binding(n),
              e @ (Expr::Member { .. } | Expr::Index { .. }) => ForOfHead::Target(e),
              other => {
                return Err(Error::ParseError(format!(
                  "Invalid for-of assignment target: {other:?}"
                )));
              }
            };
            let src = self.parse_expression(0)?;
            self.expect(&TokenKind::RParen)?;
            let body = Box::new(self.parse_statement()?);
            return Ok(Stmt::ForOf { head, src, body });
          }
          self.expect(&TokenKind::Semicolon)?;
          Some(Box::new(Stmt::Expr(init_expr)))
        };
        let cond = if self.peek() == &TokenKind::Semicolon {
          None
        } else {
          Some(self.parse_expression(0)?)
        };
        self.expect(&TokenKind::Semicolon)?;
        let step = if self.peek() == &TokenKind::RParen {
          None
        } else {
          Some(self.parse_expression(0)?)
        };
        self.expect(&TokenKind::RParen)?;
        let body = Box::new(self.parse_statement()?);
        Ok(Stmt::For {
          init,
          cond,
          step,
          body,
        })
      }
      TokenKind::Switch => {
        self.skip();
        self.expect(&TokenKind::LParen)?;
        let discriminant = self.parse_expression(0)?;
        self.expect(&TokenKind::RParen)?;
        self.expect(&TokenKind::LBrace)?;
        let mut cases = Vec::new();
        while self.peek() != &TokenKind::RBrace && self.peek() != &TokenKind::Eof {
          if self.match_token(&TokenKind::Case) {
            let test = self.parse_expression(0)?;
            self.expect(&TokenKind::Colon)?;
            let mut body = Vec::new();
            while self.peek() != &TokenKind::Case
              && self.peek() != &TokenKind::Default
              && self.peek() != &TokenKind::RBrace
              && self.peek() != &TokenKind::Eof
            {
              body.push(self.parse_statement()?);
            }
            cases.push(SwitchCase {
              test: Some(test),
              body,
            });
          } else if self.match_token(&TokenKind::Default) {
            self.expect(&TokenKind::Colon)?;
            let mut body = Vec::new();
            while self.peek() != &TokenKind::Case
              && self.peek() != &TokenKind::Default
              && self.peek() != &TokenKind::RBrace
              && self.peek() != &TokenKind::Eof
            {
              body.push(self.parse_statement()?);
            }
            cases.push(SwitchCase { test: None, body });
          } else {
            return Err(Error::ParseError(format!(
              "Expected 'case' or 'default', got {:?}",
              self.peek()
            )));
          }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(Stmt::Switch {
          discriminant,
          cases,
        })
      }
      TokenKind::Break => {
        let kw_line = self.tokens[self.cursor.min(self.tokens.len() - 1)].line;
        self.skip();
        let label = self.take_label(kw_line)?;
        match &label {
          None => {
            if self.loop_depth + self.switch_depth == 0 {
              return Err(Error::ParseError("Illegal break statement".to_string()));
            }
          }
          Some(l) => {
            if !self.ctrl_labels.iter().any(|(n, _)| n == l) {
              return Err(Error::ParseError(format!("Undefined label '{}'", l)));
            }
          }
        }
        let _ = self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::Break(label))
      }
      TokenKind::Continue => {
        let kw_line = self.tokens[self.cursor.min(self.tokens.len() - 1)].line;
        self.skip();
        let label = self.take_label(kw_line)?;
        match &label {
          None => {
            if self.loop_depth == 0 {
              return Err(Error::ParseError(
                "Illegal continue statement: no surrounding iteration statement".to_string(),
              ));
            }
          }
          Some(l) => match self.ctrl_labels.iter().find(|(n, _)| n == l) {
            None => {
              return Err(Error::ParseError(format!("Undefined label '{}'", l)));
            }
            Some((_, false)) => {
              return Err(Error::ParseError(format!(
                "Illegal continue statement: '{}' does not denote an iteration statement",
                l
              )));
            }
            Some((_, true)) => {}
          },
        }
        let _ = self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::Continue(label))
      }
      TokenKind::Try => {
        self.skip();
        let block = self.parse_block()?;
        let mut catch_param = None;
        let mut catch_block = None;
        if self.match_token(&TokenKind::Catch) {
          if self.match_token(&TokenKind::LParen) {
            if let TokenKind::Identifier(p) = self.take() {
              catch_param = Some(p);
            }
            self.expect(&TokenKind::RParen)?;
          }
          catch_block = Some(self.parse_block()?);
        }
        let mut finally_block = None;
        if self.match_token(&TokenKind::Finally) {
          finally_block = Some(self.parse_block()?);
        }
        Ok(Stmt::Try {
          block,
          catch_param,
          catch_block,
          finally_block,
        })
      }
      TokenKind::Throw => {
        self.skip();
        let expr = self.parse_expression(0)?;
        let _ = self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::Throw(expr))
      }
      TokenKind::LBrace => {
        let block = self.parse_block()?;
        Ok(Stmt::Block(block))
      }
      // 语句位数组解构赋值 `[a,b] = c` (12.38): 词法上与索引/数组字面量同头,
      // 靠配平扫描后的孤立 `=` 判型; 大括号头按 JS 惯例仍是块
      TokenKind::LBracket if self.looks_like_destructure_assign() => {
        let pat = self.parse_pattern()?;
        self.expect(&TokenKind::Equal)?;
        let value = self.parse_expression(0)?;
        let _ = self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::Expr(Expr::AssignPattern {
          pat,
          value: Box::new(value),
        }))
      }
      _ => {
        let expr = self.parse_expression(0)?;
        let _ = self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::Expr(expr))
      }
    }
  }

  fn parse_block(&mut self) -> Result<Vec<Stmt>> {
    self.expect(&TokenKind::LBrace)?;
    let mut stmts = Vec::new();
    while self.peek() != &TokenKind::RBrace && self.peek() != &TokenKind::Eof {
      stmts.push(self.parse_statement()?);
    }
    self.expect(&TokenKind::RBrace)?;
    Ok(desugar_stmts(stmts))
  }

  /// 消费 `(a, [b], {c}, ...r)` 参数表 (左圆括号已吃), 返回 (参数名表, 函数体前言语句, rest 绑定)。
  /// 解构参数折成 `$argN` 槽位名 + 体内首部一条 VarPattern (调用方在构造
  /// SharedFunctionInfo 之前 desugar 拼接, 保证 hoisted_vars 统计看到展开后的普通节点);
  /// rest 不占 params 按位槽, 由 SharedFunctionInfo.rest_param 交给两链进帧绑定 (12.40)
  fn parse_param_table(&mut self) -> Result<ParamTable> {
    let mut params = Vec::new();
    let mut prelude = Vec::new();
    let mut rest = None;
    if self.peek() != &TokenKind::RParen {
      loop {
        if self.at_spread() {
          self.eat_spread();
          match self.take() {
            TokenKind::Identifier(r) => {
              // node: `...r = x` 与 `f(...r,)` 都是 SyntaxError; rest 起点 = 已按位形参数
              if self.peek() == &TokenKind::Equal {
                return Err(Error::ParseError(
                  "Rest parameter cannot have a default".to_string(),
                ));
              }
              rest = Some((r, params.len()));
            }
            other => {
              return Err(Error::ParseError(format!(
                "Expected identifier after rest '...', got {other:?}"
              )));
            }
          }
          break;
        }
        if matches!(self.peek(), TokenKind::LBracket | TokenKind::LBrace) {
          let p = self.parse_pattern()?;
          let pat = self.wrap_default(p)?;
          let slot = gen_name("arg");
          params.push(slot.clone());
          prelude.push(Stmt::VarPattern {
            pat,
            init: Some(Expr::Identifier(slot)),
          });
        } else {
          match self.take() {
            TokenKind::Identifier(p) => {
              params.push(p.clone());
              // 简单形参默认 `a = expr`: 折成体内前言 `var a = a===undefined ? expr : a`
              // (实参槽缺省即 undefined, 短路条件保证 fallback 只在触发时求值)
              if self.match_token(&TokenKind::Equal) {
                let fb = self.parse_expression(2)?;
                prelude.push(Stmt::VarPattern {
                  pat: Pattern::Default {
                    pat: Box::new(Pattern::Binding(p.clone())),
                    fallback: Box::new(fb),
                  },
                  init: Some(Expr::Identifier(p)),
                });
              }
            }
            other => {
              return Err(Error::ParseError(format!(
                "Expected parameter name, got {other:?}"
              )));
            }
          }
        }
        if !self.match_token(&TokenKind::Comma) {
          break;
        }
      }
    }
    self.expect(&TokenKind::RParen)?;
    Ok((params, prelude, rest))
  }

  /// 解析一个解构模式 (12.38); 顶层不吞 `= 默认值` (声明/赋值右值靠它),
  /// 逐项默认值由调用方经 wrap_default 挂上
  fn parse_pattern(&mut self) -> Result<Pattern> {
    match self.peek() {
      TokenKind::LBracket => {
        self.skip();
        let mut elements: Vec<Option<Pattern>> = Vec::new();
        let mut rest: Option<Box<Pattern>> = None;
        let mut first = true;
        loop {
          if self.match_token(&TokenKind::RBracket) {
            break;
          }
          if !first {
            self.expect(&TokenKind::Comma)?;
            if self.match_token(&TokenKind::RBracket) {
              break; // 尾逗号 `[a,]`
            }
          }
          first = false;
          if self.at_spread() {
            self.eat_spread();
            rest = Some(Box::new(self.parse_pattern()?));
            self.expect(&TokenKind::RBracket)?;
            break;
          }
          if self.peek() == &TokenKind::Comma {
            // 空位 `[a,,b]`: 占位 None, 逗号留给下一轮配平
            elements.push(None);
            continue;
          }
          let p = self.parse_pattern()?;
          elements.push(Some(self.wrap_default(p)?));
        }
        Ok(Pattern::Array { elements, rest })
      }
      TokenKind::LBrace => {
        self.skip();
        let mut props: Vec<(String, Pattern)> = Vec::new();
        let mut rest: Option<Box<Pattern>> = None;
        let mut first = true;
        loop {
          if self.match_token(&TokenKind::RBrace) {
            break;
          }
          if !first {
            self.expect(&TokenKind::Comma)?;
            if self.match_token(&TokenKind::RBrace) {
              break;
            }
          }
          first = false;
          if self.at_spread() {
            self.eat_spread();
            match self.parse_pattern()? {
              // 剩余只收标识符绑定 (`{...r}`); 嵌套剩余折十五 B
              Pattern::Binding(id) => rest = Some(Box::new(Pattern::Binding(id))),
              other => {
                return Err(Error::ParseError(format!(
                  "Object rest must be an identifier binding, got {other:?}"
                )));
              }
            }
            self.expect(&TokenKind::RBrace)?;
            break;
          }
          let key = match self.take() {
            TokenKind::Identifier(s) => s,
            TokenKind::String(s) => s,
            other => {
              return Err(Error::ParseError(format!(
                "Expected destructuring key, got {other:?}"
              )));
            }
          };
          let pat = if self.match_token(&TokenKind::Colon) {
            let p = self.parse_pattern()?;
            self.wrap_default(p)?
          } else {
            // 短语法 `{x}` / 带默认 `{x = 1}`
            self.wrap_default(Pattern::Binding(key.clone()))?
          };
          props.push((key, pat));
        }
        Ok(Pattern::Object { props, rest })
      }
      TokenKind::Identifier(_) => match self.take() {
        TokenKind::Identifier(s) => Ok(Pattern::Binding(s)),
        other => Err(Error::ParseError(format!(
          "Expected destructuring pattern, got {other:?}"
        ))),
      },
      other => Err(Error::ParseError(format!(
        "Expected destructuring pattern, got {other:?}"
      ))),
    }
  }

  /// 模式项后跟 `=` 则包一层默认值 (node 口径: 实取值为 undefined 才触发)
  fn wrap_default(&mut self, pat: Pattern) -> Result<Pattern> {
    if self.match_token(&TokenKind::Equal) {
      Ok(Pattern::Default {
        pat: Box::new(pat),
        fallback: Box::new(self.parse_expression(2)?),
      })
    } else {
      Ok(pat)
    }
  }

  /// 语句起头的 `[...]` 是否为解构赋值: 非破坏性扫到配平 `]` 后紧跟孤立 `=`
  /// (`=>` 词法是 Equal+Greater, `==` 是独立 token, 各再排除一位)
  fn looks_like_destructure_assign(&self) -> bool {
    let Some(close) = self.scan_balanced_closer() else {
      return false;
    };
    let eq = self
      .tokens
      .get(close + 1)
      .is_some_and(|t| t.kind == TokenKind::Equal);
    let tail_ok = self
      .tokens
      .get(close + 2)
      .is_none_or(|t| !matches!(t.kind, TokenKind::Equal | TokenKind::Greater));
    eq && tail_ok
  }

  /// 起头 `[`/`{` 非破坏性配平扫描 (调用方保证游标在开括上), 返回配平闭括的下标
  fn scan_balanced_closer(&self) -> Option<usize> {
    let mut depth = 0usize;
    let mut i = self.cursor;
    while i < self.tokens.len() {
      match &self.tokens[i].kind {
        TokenKind::LBracket | TokenKind::LBrace | TokenKind::LParen => depth += 1,
        TokenKind::RBracket | TokenKind::RBrace | TokenKind::RParen => {
          depth -= 1;
          if depth == 0 {
            return Some(i);
          }
        }
        _ => {}
      }
      i += 1;
    }
    None
  }

  /// for 头起头的 `[...]`/`{...}` 是否为解构 for-of 模式头: 配平闭括后紧跟上下文 `of` (17 轮)
  fn looks_like_destructure_for_head(&self) -> bool {
    self.scan_balanced_closer().is_some_and(|close| {
      self
        .tokens
        .get(close + 1)
        .is_some_and(|t| matches!(&t.kind, TokenKind::Identifier(k) if k == "of"))
    })
  }

  fn token_to_ident(tok: &TokenKind) -> Option<String> {
    match tok {
      TokenKind::Identifier(id) => Some(id.clone()),
      TokenKind::Var => Some("var".to_string()),
      TokenKind::Let => Some("let".to_string()),
      TokenKind::Const => Some("const".to_string()),
      TokenKind::Function => Some("function".to_string()),
      TokenKind::Return => Some("return".to_string()),
      TokenKind::If => Some("if".to_string()),
      TokenKind::Else => Some("else".to_string()),
      TokenKind::While => Some("while".to_string()),
      TokenKind::For => Some("for".to_string()),
      TokenKind::Break => Some("break".to_string()),
      TokenKind::Continue => Some("continue".to_string()),
      TokenKind::New => Some("new".to_string()),
      TokenKind::This => Some("this".to_string()),
      TokenKind::Typeof => Some("typeof".to_string()),
      TokenKind::Try => Some("try".to_string()),
      TokenKind::Catch => Some("catch".to_string()),
      TokenKind::Finally => Some("finally".to_string()),
      TokenKind::Throw => Some("throw".to_string()),
      TokenKind::Instanceof => Some("instanceof".to_string()),
      TokenKind::In => Some("in".to_string()),
      TokenKind::Switch => Some("switch".to_string()),
      TokenKind::Case => Some("case".to_string()),
      TokenKind::Default => Some("default".to_string()),
      TokenKind::Do => Some("do".to_string()),
      TokenKind::Void => Some("void".to_string()),
      TokenKind::Delete => Some("delete".to_string()),
      TokenKind::True => Some("true".to_string()),
      TokenKind::False => Some("false".to_string()),
      TokenKind::Null => Some("null".to_string()),
      TokenKind::Undefined => Some("undefined".to_string()),
      _ => None,
    }
  }

  // 优先级获取 (编译期查表分派)
  const fn get_precedence(op: &TokenKind) -> u8 {
    match op {
      TokenKind::Comma => 1,

      TokenKind::Equal
      | TokenKind::PlusEqual
      | TokenKind::MinusEqual
      | TokenKind::StarEqual
      | TokenKind::SlashEqual
      | TokenKind::PercentEqual
      | TokenKind::AmpersandEqual
      | TokenKind::PipeEqual
      | TokenKind::CaretEqual
      | TokenKind::DoubleLessEqual
      | TokenKind::DoubleGreaterEqual
      | TokenKind::TripleGreaterEqual => 2,

      TokenKind::DoublePipe => 3,
      TokenKind::DoubleAmpersand => 4,
      TokenKind::Pipe => 5,
      TokenKind::Caret => 6,
      TokenKind::Ampersand => 7,

      TokenKind::DoubleEqual
      | TokenKind::TripleEqual
      | TokenKind::ExclamationEqual
      | TokenKind::ExclamationDoubleEqual => 8,

      TokenKind::Less
      | TokenKind::LessEqual
      | TokenKind::Greater
      | TokenKind::GreaterEqual
      | TokenKind::Instanceof
      | TokenKind::In => 9,

      TokenKind::DoubleLess | TokenKind::DoubleGreater | TokenKind::TripleGreater => 10,

      TokenKind::Plus | TokenKind::Minus => 11,
      TokenKind::Star | TokenKind::Slash | TokenKind::Percent => 12,

      _ => 0,
    }
  }

  /// 当前位置是否为 `...` (词法器原样吐三个 Dot, 没有独立 token, 12.28 spread 脱糖)
  #[inline]
  fn at_spread(&self) -> bool {
    self.peek() == &TokenKind::Dot
      && self.peek_at(1) == &TokenKind::Dot
      && self.peek_at(2) == &TokenKind::Dot
  }

  /// 消费 `...`
  fn eat_spread(&mut self) {
    self.skip();
    self.skip();
    self.skip();
  }

  /// 把含 `...` 的项表拼成"展平一层再串接"的数组表达式: `[a, ...b, c]` → `[a].concat(Array.from(b), [c])`。
  /// 仅在有 spread 时被调用 (无 spread 的表走原字面量路, 免多一跳 concat);
  /// `Array.from` 包一层是为了字符串展开 (`[...""]` → `[]`, 直接 concat 会把整串当一个元素),
  /// 顺带让被展开数组取快照 (node: spread 复制, `[...(a=nums, 0)]` 后改元素不影响已展平副本)
  fn flatten_spread_items(items: Vec<(bool, Expr)>) -> Expr {
    let mut base: Vec<Expr> = Vec::new();
    let mut args: Vec<Expr> = Vec::new();
    let mut cur: Vec<Expr> = Vec::new();
    let mut base_taken = false;
    for (spread, e) in items {
      if spread {
        if !base_taken {
          base = take(&mut cur);
          base_taken = true;
        } else if !cur.is_empty() {
          args.push(Expr::Array(take(&mut cur)));
        }
        args.push(Expr::Call {
          callee: Box::new(Expr::Member {
            object: Box::new(Expr::Identifier("Array".to_string())),
            property: "from".to_string(),
          }),
          args: vec![e],
        });
      } else {
        cur.push(e);
      }
    }
    if !cur.is_empty() {
      args.push(Expr::Array(take(&mut cur)));
    }
    let mut expr = Expr::Array(base);
    for a in args {
      expr = Expr::Call {
        callee: Box::new(Expr::Member {
          object: Box::new(expr),
          property: "concat".to_string(),
        }),
        args: vec![a],
      };
    }
    expr
  }

  // Pratt 解析表达式
  pub fn parse_expression(&mut self, min_bp: u8) -> Result<Expr> {
    let mut left = self.parse_prefix()?;

    loop {
      // 处理后缀成员/调用/索引/自增
      while self.peek() == &TokenKind::LParen
        || self.peek() == &TokenKind::LBracket
        || self.peek() == &TokenKind::Dot
        || self.peek() == &TokenKind::DoublePlus
        || self.peek() == &TokenKind::DoubleMinus
      {
        match self.peek() {
          TokenKind::LParen => {
            self.skip();
            let mut items: Vec<(bool, Expr)> = Vec::new();
            if self.peek() != &TokenKind::RParen {
              loop {
                let spread = self.at_spread();
                if spread {
                  self.eat_spread();
                }
                items.push((spread, self.parse_expression(2)?));
                if !self.match_token(&TokenKind::Comma) {
                  break;
                }
              }
            }
            self.expect(&TokenKind::RParen)?;
            let has_spread = items.iter().any(|(spread, _)| *spread);
            left = if has_spread {
              // `f(...a)` → `f.apply(thisArg, 展平数组)`; thisArg: 成员调用的对象体取
              // 表达式克隆 (含副作用的接收者会被二次求值, 记偏差档), 其余 undefined
              let array_expr = Self::flatten_spread_items(items);
              let this_arg = match &left {
                Expr::Member { object, .. } | Expr::Index { object, .. } => (**object).clone(),
                _ => Expr::Literal(JsValue::Undefined),
              };
              Expr::Call {
                callee: Box::new(Expr::Member {
                  object: Box::new(left),
                  property: "apply".to_string(),
                }),
                args: vec![this_arg, array_expr],
              }
            } else {
              Expr::Call {
                callee: Box::new(left),
                args: items.into_iter().map(|(_, e)| e).collect(),
              }
            };
          }
          TokenKind::LBracket => {
            self.skip();
            let index = self.parse_expression(0)?;
            self.expect(&TokenKind::RBracket)?;
            left = Expr::Index {
              object: Box::new(left),
              index: Box::new(index),
            };
          }
          TokenKind::Dot => {
            self.skip();
            let tok = self.take();
            let prop = if let Some(name) = Self::token_to_ident(&tok) {
              name
            } else {
              return Err(Error::ParseError(format!(
                "Expected identifier after '.', got {tok:?}"
              )));
            };
            left = Expr::Member {
              object: Box::new(left),
              property: prop,
            };
          }
          TokenKind::DoublePlus => {
            self.skip();
            left = Expr::Unary {
              op: UnaryOp::PostInc,
              expr: Box::new(left),
            };
          }
          TokenKind::DoubleMinus => {
            self.skip();
            left = Expr::Unary {
              op: UnaryOp::PostDec,
              expr: Box::new(left),
            };
          }
          _ => unreachable!(),
        }
      }

      // 三元条件运算符 cond ? cons : alt (优先级高于赋值，右结合)
      if self.peek() == &TokenKind::Question && min_bp <= 2 {
        self.skip();
        let consequent = self.parse_expression(1)?;
        self.expect(&TokenKind::Colon)?;
        let alternate = self.parse_expression(2)?;
        left = Expr::Conditional {
          condition: Box::new(left),
          consequent: Box::new(consequent),
          alternate: Box::new(alternate),
        };
        continue;
      }

      let prec = Self::get_precedence(self.peek());
      if prec == 0 || prec < min_bp {
        break;
      }

      let op_token = self.take();

      // 处理赋值操作符
      if let Some(assign_op) = match &op_token {
        TokenKind::Equal => Some(AssignOp::Assign),
        TokenKind::PlusEqual => Some(AssignOp::AddAssign),
        TokenKind::MinusEqual => Some(AssignOp::SubAssign),
        TokenKind::StarEqual => Some(AssignOp::MulAssign),
        TokenKind::SlashEqual => Some(AssignOp::DivAssign),
        TokenKind::PercentEqual => Some(AssignOp::ModAssign),
        TokenKind::AmpersandEqual => Some(AssignOp::BitAndAssign),
        TokenKind::PipeEqual => Some(AssignOp::BitOrAssign),
        TokenKind::CaretEqual => Some(AssignOp::BitXorAssign),
        TokenKind::DoubleLessEqual => Some(AssignOp::ShlAssign),
        TokenKind::DoubleGreaterEqual => Some(AssignOp::ShrAssign),
        TokenKind::TripleGreaterEqual => Some(AssignOp::SarAssign),
        _ => None,
      } {
        let right = self.parse_expression(prec)?;
        left = Expr::Assign {
          target: Box::new(left),
          op: assign_op,
          value: Box::new(right),
        };
        continue;
      }

      // 二元操作符
      let bin_op = match op_token {
        TokenKind::Comma => BinaryOp::Comma,
        TokenKind::Plus => BinaryOp::Add,
        TokenKind::Minus => BinaryOp::Sub,
        TokenKind::Star => BinaryOp::Mul,
        TokenKind::Slash => BinaryOp::Div,
        TokenKind::Percent => BinaryOp::Mod,
        TokenKind::Ampersand => BinaryOp::BitAnd,
        TokenKind::Pipe => BinaryOp::BitOr,
        TokenKind::Caret => BinaryOp::BitXor,
        TokenKind::DoubleLess => BinaryOp::Shl,
        TokenKind::DoubleGreater => BinaryOp::Shr,
        TokenKind::TripleGreater => BinaryOp::Sar,
        TokenKind::DoubleEqual => BinaryOp::Eq,
        TokenKind::TripleEqual => BinaryOp::StrictEq,
        TokenKind::ExclamationEqual => BinaryOp::NotEq,
        TokenKind::ExclamationDoubleEqual => BinaryOp::StrictNotEq,
        TokenKind::Less => BinaryOp::Lt,
        TokenKind::LessEqual => BinaryOp::Lte,
        TokenKind::Greater => BinaryOp::Gt,
        TokenKind::GreaterEqual => BinaryOp::Gte,
        TokenKind::Instanceof => BinaryOp::InstanceOf,
        TokenKind::In => BinaryOp::In,
        TokenKind::DoubleAmpersand => BinaryOp::LogicalAnd,
        TokenKind::DoublePipe => BinaryOp::LogicalOr,
        _ => {
          return Err(Error::ParseError(format!(
            "Unexpected binary operator {op_token:?}"
          )));
        }
      };

      let right = self.parse_expression(prec + 1)?;
      left = Expr::Binary {
        op: bin_op,
        left: Box::new(left),
        right: Box::new(right),
      };
    }

    Ok(left)
  }

  fn parse_prefix(&mut self) -> Result<Expr> {
    match self.peek() {
      TokenKind::Number(_) => {
        if let TokenKind::Number(n) = self.take() {
          Ok(Expr::Literal(JsValue::Number(n)))
        } else {
          unreachable!()
        }
      }
      TokenKind::BigInt(_) => {
        if let TokenKind::BigInt(n) = self.take() {
          Ok(Expr::Literal(JsValue::BigInt(n)))
        } else {
          unreachable!()
        }
      }
      TokenKind::String(_) => {
        if let TokenKind::String(s) = self.take() {
          Ok(Expr::Literal(JsValue::String(s.into())))
        } else {
          unreachable!()
        }
      }
      TokenKind::RegExp { .. } => {
        // C-④a: 解析期不再预编译预物化 (旧形态让同一字面量节点跨求值复用实例,
        // 违规范 per-eval 且 lastIndex 可跨调用泄漏); 只落 pattern/flags, 求值期新建实例
        if let TokenKind::RegExp { pattern, flags } = self.take() {
          Ok(Expr::RegExp { pattern, flags })
        } else {
          unreachable!()
        }
      }
      TokenKind::True => {
        self.skip();
        Ok(Expr::Literal(JsValue::Boolean(true)))
      }
      TokenKind::False => {
        self.skip();
        Ok(Expr::Literal(JsValue::Boolean(false)))
      }
      TokenKind::Null => {
        self.skip();
        Ok(Expr::Literal(JsValue::Null))
      }
      TokenKind::Undefined => {
        self.skip();
        Ok(Expr::Literal(JsValue::Undefined))
      }
      TokenKind::This => {
        self.skip();
        Ok(Expr::This)
      }
      TokenKind::Identifier(_) => {
        let start = self.tokens[self.cursor].pos;
        if let TokenKind::Identifier(id) = self.take() {
          // `id => ...`: 词法器把 => 拆成 Equal+Greater 两 token, 单参数箭头的识别只能在这里做
          if self.peek() == &TokenKind::Equal && self.peek_at(1) == &TokenKind::Greater {
            return self.parse_arrow_tail(vec![id], Vec::new(), None, start);
          }
          Ok(Expr::Identifier(id))
        } else {
          unreachable!()
        }
      }
      TokenKind::LParen => {
        let start = self.tokens[self.cursor].pos;
        if self.paren_is_arrow_head() {
          self.skip();
          // 参数表与 function 共用 parse_param_table 单点 (解构/默认/rest 形参自动可用)
          let (params, prelude, rest) = self.parse_param_table()?;
          return self.parse_arrow_tail(params, prelude, rest, start);
        }
        self.skip();
        let expr = self.parse_expression(0)?;
        self.expect(&TokenKind::RParen)?;
        Ok(expr)
      }
      TokenKind::Plus => {
        self.skip();
        let expr = self.parse_expression(12)?;
        Ok(Expr::Unary {
          op: UnaryOp::Pos,
          expr: Box::new(expr),
        })
      }
      TokenKind::Minus => {
        self.skip();
        let expr = self.parse_expression(12)?;
        Ok(Expr::Unary {
          op: UnaryOp::Neg,
          expr: Box::new(expr),
        })
      }
      TokenKind::Void => {
        self.skip();
        let expr = self.parse_expression(12)?;
        Ok(Expr::Unary {
          op: UnaryOp::Void,
          expr: Box::new(expr),
        })
      }
      TokenKind::Delete => {
        self.skip();
        let expr = self.parse_expression(12)?;
        Ok(Expr::Unary {
          op: UnaryOp::Delete,
          expr: Box::new(expr),
        })
      }
      TokenKind::Exclamation => {
        self.skip();
        let expr = self.parse_expression(12)?;
        Ok(Expr::Unary {
          op: UnaryOp::Not,
          expr: Box::new(expr),
        })
      }
      TokenKind::Tilde => {
        self.skip();
        let expr = self.parse_expression(12)?;
        Ok(Expr::Unary {
          op: UnaryOp::BitNot,
          expr: Box::new(expr),
        })
      }
      TokenKind::Typeof => {
        self.skip();
        let expr = self.parse_expression(12)?;
        Ok(Expr::Typeof(Box::new(expr)))
      }
      TokenKind::New => {
        self.skip();
        let mut callee = self.parse_prefix()?;
        while self.peek() == &TokenKind::Dot || self.peek() == &TokenKind::LBracket {
          if self.match_token(&TokenKind::Dot) {
            let tok = self.take();
            let prop = if let Some(name) = Self::token_to_ident(&tok) {
              name
            } else {
              return Err(Error::ParseError(format!(
                "Expected property name after '.', got {tok:?}"
              )));
            };
            callee = Expr::Member {
              object: Box::new(callee),
              property: prop,
            };
          } else if self.match_token(&TokenKind::LBracket) {
            let index = self.parse_expression(0)?;
            self.expect(&TokenKind::RBracket)?;
            callee = Expr::Index {
              object: Box::new(callee),
              index: Box::new(index),
            };
          }
        }
        let mut args = Vec::new();
        if self.match_token(&TokenKind::LParen) {
          if self.peek() != &TokenKind::RParen {
            loop {
              if self.at_spread() {
                // `new C(...a)` 需要 Construct 语义 (Reflect.construct), apply 脱糖替代不了
                return Err(Error::ParseError(
                  "new 的展开实参未接入 (战役 C)".to_string(),
                ));
              }
              args.push(self.parse_expression(2)?);
              if !self.match_token(&TokenKind::Comma) {
                break;
              }
            }
          }
          self.expect(&TokenKind::RParen)?;
        }
        Ok(Expr::New {
          callee: Box::new(callee),
          args,
        })
      }
      TokenKind::DoublePlus => {
        self.skip();
        let expr = self.parse_expression(12)?;
        Ok(Expr::Unary {
          op: UnaryOp::PreInc,
          expr: Box::new(expr),
        })
      }
      TokenKind::DoubleMinus => {
        self.skip();
        let expr = self.parse_expression(12)?;
        Ok(Expr::Unary {
          op: UnaryOp::PreDec,
          expr: Box::new(expr),
        })
      }
      TokenKind::LBracket => {
        self.skip();
        let mut items: Vec<(bool, Expr)> = Vec::new();
        if self.peek() != &TokenKind::RBracket {
          loop {
            // 尾逗号 `[1,2,]` 收括号即止 (不添元素); 数组空位 `[1,,3]` 逗号相毗邻 → 洞位。
            // 战役 C ③: 空位落 TheHole 哨兵 (读面物化 undefined、in/keys/for-in 即不存在),
            // 不再以 Undefined 近似填槽
            if self.peek() == &TokenKind::RBracket {
              break;
            }
            if self.peek() == &TokenKind::Comma {
              items.push((false, Expr::Literal(JsValue::TheHole)));
            } else {
              let spread = self.at_spread();
              if spread {
                self.eat_spread();
              }
              items.push((spread, self.parse_expression(2)?));
            }
            if !self.match_token(&TokenKind::Comma) {
              break;
            }
          }
        }
        self.expect(&TokenKind::RBracket)?;
        if items.iter().any(|(spread, _)| *spread) {
          Ok(Self::flatten_spread_items(items))
        } else {
          Ok(Expr::Array(items.into_iter().map(|(_, e)| e).collect()))
        }
      }
      TokenKind::LBrace => self.parse_object_literal(),
      TokenKind::Function => {
        let start = self.tokens[self.cursor].pos;
        self.skip();
        // 具名才 take: 匿名时 `(` 必须原样留给参数表
        let name = if matches!(self.peek(), TokenKind::Identifier(_)) {
          match self.take() {
            TokenKind::Identifier(id) => Some(id),
            _ => None,
          }
        } else {
          None
        };
        self.parse_function_tail(name, start)
      }
      other => {
        let (line, col) = if self.cursor < self.tokens.len() {
          (self.tokens[self.cursor].line, self.tokens[self.cursor].col)
        } else {
          (0, 0)
        };
        Err(Error::ParseError(format!(
          "Unexpected prefix token {other:?} at line {line}:{col}"
        )))
      }
    }
  }

  /// 解析 `(...)` 参数表 + `{}` 函数体, 产出函数值
  ///
  /// `function` 表达式、对象方法简写与 accessor 三条语法共用同一段后缀;
  /// `start` 是这条函数语法的原文起点 (供 toString 切片)
  fn parse_function_tail(&mut self, name: Option<String>, start: usize) -> Result<Expr> {
    self.expect(&TokenKind::LParen)?;
    let (params, prelude, rest) = self.parse_param_table()?;
    let mut body = self.parse_function_block()?;
    if !prelude.is_empty() {
      body = splice_prelude(prelude, body);
    }
    let src = Some(self.slice_source(start));
    Ok(Expr::FunctionExpr(Arc::new(
      SharedFunctionInfo::new_with_src(name, params, body, src, rest),
    )))
  }

  /// 光标处的 `(...)` 是否是箭头函数参数表: 向前扫到配平 `)` 后紧跟 `=` `>`
  ///
  /// 只数圆括号层深度 — 串内括号是独立 token, 大括号/方括号里的圆括号照常配平,
  /// 非法源码最多误判成参数表后在 parse_param_table 报语法错, 与 JS 一致
  fn paren_is_arrow_head(&self) -> bool {
    let mut depth = 0usize;
    let mut i = self.cursor;
    while i < self.tokens.len() {
      match &self.tokens[i].kind {
        TokenKind::LParen => depth += 1,
        TokenKind::RParen => {
          depth -= 1;
          if depth == 0 {
            return matches!(self.tokens.get(i + 1), Some(t) if t.kind == TokenKind::Equal)
              && matches!(self.tokens.get(i + 2), Some(t) if t.kind == TokenKind::Greater);
          }
        }
        _ => {}
      }
      i += 1;
    }
    false
  }

  /// `=>` 与函数体: 花括号走 parse_block, 表达式体折成单条 Return 语句
  ///
  /// 表达式体按 AssignmentExpression 优先级 (2) 解析: 逗号表达式必须留在外层,
  /// 否则 `f(x => a, b)` 的实参表会被吞掉后半段
  fn parse_arrow_tail(
    &mut self,
    params: Vec<String>,
    prelude: Vec<Stmt>,
    rest: Option<(String, usize)>,
    start: usize,
  ) -> Result<Expr> {
    self.skip();
    self.skip();
    let mut body = if self.peek() == &TokenKind::LBrace {
      self.parse_function_block()?
    } else {
      let e = self.parse_expression(2)?;
      vec![Stmt::Return(Some(e))]
    };
    if !prelude.is_empty() {
      body = splice_prelude(prelude, body);
    }
    let src = Some(self.slice_arrow_source(start));
    Ok(Expr::FunctionExpr(Arc::new(SharedFunctionInfo::new_arrow(
      params, body, src, rest,
    ))))
  }

  /// 箭头函数原文切片: 终点取最后一条 token 的真实末尾 (表达式体结尾常是多字节 token,
  /// function 的 `pos+1` 口径只适用于闭合 `}`)
  fn slice_arrow_source(&self, start: usize) -> String {
    let last = self.cursor.saturating_sub(1).min(self.tokens.len() - 1);
    let bytes = self.source.as_bytes();
    let mut end = self.tokens[last].pos;
    match &self.tokens[last].kind {
      TokenKind::String(_) => {
        let quote = bytes.get(end).copied();
        end += 1;
        while end < bytes.len() {
          match bytes[end] {
            b'\\' => end += 2,
            q if Some(q) == quote => {
              end += 1;
              break;
            }
            _ => end += 1,
          }
        }
      }
      _ => {
        while end < bytes.len()
          && (bytes[end].is_ascii_alphanumeric()
            || matches!(bytes[end], b'_' | b'$' | b'.' | 0x80..=0xff))
        {
          end += 1;
        }
      }
    }
    let end = end.max(self.tokens[last].pos + 1);
    self.source[start.min(end)..end].to_string()
  }

  /// 对象字面量键。第二个返回值是标识符键的字面名, 供 `{x}` 值简写复用
  fn parse_object_key(&mut self) -> Result<(ObjectKey, Option<String>)> {
    if self.match_token(&TokenKind::LBracket) {
      let expr = self.parse_expression(0)?;
      self.expect(&TokenKind::RBracket)?;
      return Ok((ObjectKey::Computed(expr), None));
    }
    let tok = self.take();
    let is_ident = matches!(tok, TokenKind::Identifier(_));
    let k_str = match tok {
      TokenKind::Identifier(k) => k,
      TokenKind::String(s) => s,
      // 数字键必须按 ToString 归一 (`{1e21: v}` 的键是 "1e+21"), 否则和
      // `obj[1e21]` 读点用的键不是同一个
      TokenKind::Number(n) => double_to_string(n),
      TokenKind::BigInt(n) => {
        let mut buf = Buffer::new();
        buf.format(n).to_string()
      }
      tok => {
        if let Some(ident) = Self::token_to_ident(&tok) {
          ident
        } else {
          return Err(Error::ParseError(format!(
            "Expected object key, got {tok:?}"
          )));
        }
      }
    };
    let literal = is_ident.then(|| k_str.clone());
    Ok((ObjectKey::Ident(k_str), literal))
  }

  /// 对象字面量: `:` 数据属性、`get`/`set` accessor、`()` 方法简写、裸标识符值简写、
  /// `...` 展开 (脱糖成 Object.assign 串接, 12.28)
  fn parse_object_literal(&mut self) -> Result<Expr> {
    // 段表: 数据属性攒组, 遇 spread 封组 —— `{...a, x:1}` → `Object.assign({}, a, {x:1})`
    enum Seg {
      Prop(ObjectProp),
      Spread(Expr),
    }
    self.skip();
    let mut segs: Vec<Seg> = Vec::new();
    while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
      if self.at_spread() {
        self.eat_spread();
        let e = self.parse_expression(2)?;
        segs.push(Seg::Spread(e));
        if !self.match_token(&TokenKind::Comma) {
          break;
        }
        continue;
      }
      // 属性起点: accessor 前缀 (`get`/`set`) 也含在 toString 原文里, 要在判 accessor 前取
      let prop_start = self.tokens[self.cursor].pos;
      // `get`/`set` 是上下文关键字: 紧跟 `:` `(` `,` `}` 时它是普通键名或方法名, 不是 accessor 前缀
      let accessor = matches!(self.peek(), TokenKind::Identifier(k) if k == "get" || k == "set")
        && !matches!(
          self.peek_at(1),
          TokenKind::Colon | TokenKind::LParen | TokenKind::Comma | TokenKind::RBrace
        );
      let kind = if accessor {
        match self.take() {
          TokenKind::Identifier(k) if k == "get" => PropKind::Get,
          TokenKind::Identifier(_) => PropKind::Set,
          other => {
            return Err(Error::ParseError(format!(
              "Expected get/set, got {other:?}"
            )));
          }
        }
      } else {
        PropKind::Data
      };
      let (key, ident) = self.parse_object_key()?;
      let value = match self.peek() {
        TokenKind::Colon => {
          self.skip();
          self.parse_expression(2)?
        }
        // `{f(){}}` 是持函数的数据属性, `{get p(){}}` 才是 accessor
        TokenKind::LParen => {
          let name = match &key {
            ObjectKey::Ident(s) => Some(s.clone()),
            ObjectKey::Computed(_) => None,
          };
          self.parse_function_tail(name, prop_start)?
        }
        TokenKind::Comma | TokenKind::RBrace => match ident {
          Some(n) => Expr::Identifier(n),
          None => {
            return Err(Error::ParseError(
              "Shorthand property must be an identifier".to_string(),
            ));
          }
        },
        other => {
          return Err(Error::ParseError(format!(
            "Expected ':' or '(' in object literal, got {other:?}"
          )));
        }
      };
      segs.push(Seg::Prop(ObjectProp { key, value, kind }));
      if !self.match_token(&TokenKind::Comma) {
        break;
      }
    }
    self.expect(&TokenKind::RBrace)?;
    if segs.iter().any(|s| matches!(s, Seg::Spread(_))) {
      let mut args: Vec<Expr> = vec![Expr::Object(Vec::new())];
      let mut cur: Vec<ObjectProp> = Vec::new();
      for s in segs {
        match s {
          Seg::Prop(p) => cur.push(p),
          Seg::Spread(e) => {
            if !cur.is_empty() {
              args.push(Expr::Object(take(&mut cur)));
            }
            args.push(e);
          }
        }
      }
      if !cur.is_empty() {
        args.push(Expr::Object(cur));
      }
      return Ok(Expr::Call {
        callee: Box::new(Expr::Member {
          object: Box::new(Expr::Identifier("Object".to_string())),
          property: "assign".to_string(),
        }),
        args,
      });
    }
    // 走到这里必无 Spread 段
    let entries: Vec<ObjectProp> = segs
      .into_iter()
      .filter_map(|s| match s {
        Seg::Prop(p) => Some(p),
        Seg::Spread(_) => None,
      })
      .collect();
    Ok(Expr::Object(entries))
  }
}

// ==================== 解构暂态 desugar 展开单点 (12.38) ====================

/// 形参表编译结果 (12.40): (按位形参名表, 体内前言语句, rest 绑定 (名, 收集起点))
type ParamTable = (Vec<String>, Vec<Stmt>, Option<(String, usize)>);

/// 临时名发生器: `$d0/$arg0/$u1/...` 模块静态计数, `$` 为合法标识符首字符且 Rust 侧不回炉词法
static DESUGAR_ID: AtomicUsize = AtomicUsize::new(0);

fn gen_name(prefix: &str) -> String {
  format!("${prefix}{}", DESUGAR_ID.fetch_add(1, Ordering::Relaxed))
}

/// 语句表展开: 暂态节点摊平成普通节点序列, 复合语句逐层递归重建 ——
/// 两条执行链 (AST 解释器 / 字节码桥) 从这里之后只见 VarDecl/Expr/Block 等既有形态
fn desugar_stmts(stmts: Vec<Stmt>) -> Vec<Stmt> {
  stmts.into_iter().flat_map(desugar_stmt).collect()
}

/// 展开后的语句序列收拢回单语句位 (if/循环体): 1 条原样, 多条包 Block
/// (本引擎 Block 不加额外作用域, var 语义不变)
fn splat(mut stmts: Vec<Stmt>) -> Stmt {
  if stmts.len() == 1 {
    stmts.remove(0)
  } else {
    Stmt::Block(stmts)
  }
}

/// 解构参数前言展开后拼到函数体头部 (必须先于 SharedFunctionInfo 构造,
/// hoisted_vars/uses_arguments 按最终 body 统计)
fn splice_prelude(mut prelude: Vec<Stmt>, mut body: Vec<Stmt>) -> Vec<Stmt> {
  prelude = desugar_stmts(prelude);
  prelude.append(&mut body);
  prelude
}

fn desugar_stmt(s: Stmt) -> Vec<Stmt> {
  match s {
    Stmt::VarPattern { pat, init } => expand_pattern(pat, init, true),
    Stmt::Expr(Expr::AssignPattern { pat, value }) => expand_pattern(pat, Some(*value), false),
    Stmt::Block(v) => vec![Stmt::Block(desugar_stmts(v))],
    Stmt::If {
      cond,
      then_branch,
      else_branch,
    } => vec![Stmt::If {
      cond,
      then_branch: Box::new(splat(desugar_stmt(*then_branch))),
      else_branch: else_branch.map(|b| Box::new(splat(desugar_stmt(*b)))),
    }],
    Stmt::While { cond, body } => vec![Stmt::While {
      cond,
      body: Box::new(splat(desugar_stmt(*body))),
    }],
    Stmt::DoWhile { body, cond } => vec![Stmt::DoWhile {
      body: Box::new(splat(desugar_stmt(*body))),
      cond,
    }],
    Stmt::For {
      init,
      cond,
      step,
      body,
    } => vec![Stmt::For {
      init: init.map(|i| Box::new(splat(desugar_stmt(*i)))),
      cond,
      step,
      body: Box::new(splat(desugar_stmt(*body))),
    }],
    Stmt::ForIn {
      variable,
      is_var,
      object,
      body,
    } => vec![Stmt::ForIn {
      variable,
      is_var,
      object,
      body: Box::new(splat(desugar_stmt(*body))),
    }],
    Stmt::ForOf { head, src, body } => expand_forof(head, src, *body),
    Stmt::Labeled { labels, body } => {
      let was_forof = matches!(&*body, Stmt::ForOf { .. });
      let mut inner = desugar_stmt(*body);
      // for-of 展开出的 `$fsrc/$fidx` 导言留在环前同层, 标签只落尾部 For
      // (若让 splat 包成 Block[导言, For], continue L 找不到循环目标)
      if was_forof
        && inner.len() > 1
        && let Some(last) = inner.pop()
      {
        let mut out = inner;
        out.push(Stmt::Labeled {
          labels,
          body: Box::new(last),
        });
        return out;
      }
      vec![Stmt::Labeled {
        labels,
        body: Box::new(splat(inner)),
      }]
    }
    Stmt::Switch {
      discriminant,
      cases,
    } => vec![Stmt::Switch {
      discriminant,
      cases: cases
        .into_iter()
        .map(|c| SwitchCase {
          test: c.test,
          body: desugar_stmts(c.body),
        })
        .collect(),
    }],
    Stmt::Try {
      block,
      catch_param,
      catch_block,
      finally_block,
    } => vec![Stmt::Try {
      block: desugar_stmts(block),
      catch_param,
      catch_block: catch_block.map(desugar_stmts),
      finally_block: finally_block.map(desugar_stmts),
    }],
    other => vec![other],
  }
}

/// for-of 单点展开 (17 轮): node 数组迭代器本就是**活长度索引循环** (每步重读 length、按下标取元),
/// 摊成 `$src/$idx` 三段式经典 For 与 node 逐位口径天然一致 —— 迭代中 push 会被吃到、洞读
/// undefined、字符串/arguments 同样走索引读; 非可迭代源 `.length` 得 undefined 静默零轮
/// (node 抛 TypeError, 迭代器协议族挂战役 C, 十四轮残档同根)
fn expand_forof(head: ForOfHead, src: Expr, body: Stmt) -> Vec<Stmt> {
  let fsrc = gen_name("fsrc");
  let fidx = gen_name("fidx");
  let elem = Expr::Index {
    object: Box::new(Expr::Identifier(fsrc.clone())),
    index: Box::new(Expr::Identifier(fidx.clone())),
  };
  // 声明式与非声明式头同走 bind 的 var 形态 (引擎 let/const≡var, 块级每迭代另挂 11.6)
  let binds = match head {
    ForOfHead::Binding(name) => vec![bind(name, elem, true)],
    ForOfHead::Pattern(pat) => expand_pattern(pat, Some(elem), true),
    ForOfHead::Target(target) => vec![Stmt::Expr(Expr::Assign {
      target: Box::new(target),
      op: AssignOp::Assign,
      value: Box::new(elem),
    })],
  };
  let mut inner = binds;
  inner.extend(desugar_stmt(body));
  let loop_stmt = Stmt::For {
    init: None,
    cond: Some(Expr::Binary {
      op: BinaryOp::Lt,
      left: Box::new(Expr::Identifier(fidx.clone())),
      right: Box::new(Expr::Member {
        object: Box::new(Expr::Identifier(fsrc.clone())),
        property: "length".to_string(),
      }),
    }),
    step: Some(Expr::Assign {
      target: Box::new(Expr::Identifier(fidx.clone())),
      op: AssignOp::Assign,
      value: Box::new(Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Identifier(fidx.clone())),
        right: Box::new(Expr::Literal(JsValue::Number(1.0))),
      }),
    }),
    body: Box::new(splat(inner)),
  };
  vec![
    Stmt::VarDecl {
      name: fsrc,
      init: Some(src),
    },
    Stmt::VarDecl {
      name: fidx,
      init: Some(Expr::Literal(JsValue::Number(0.0))),
    },
    loop_stmt,
  ]
}

/// 单条解构的根展开: 纯 Binding 免临时量, 其余把右值落一次临时变量再逐项递归
fn expand_pattern(pat: Pattern, init: Option<Expr>, declare: bool) -> Vec<Stmt> {
  let src = init.unwrap_or(Expr::Literal(JsValue::Undefined));
  match pat {
    Pattern::Binding(name) => vec![bind(name, src, declare)],
    other => destructure(other, src, declare),
  }
}

/// 把 `src` (右值表达式, 可能含副作用) 解构成语句序列; 容器层先落临时变量保证只求值一次
fn destructure(pat: Pattern, src: Expr, declare: bool) -> Vec<Stmt> {
  match pat {
    Pattern::Binding(name) => vec![bind(name, src, declare)],
    Pattern::Default {
      pat: inner,
      fallback,
    } => {
      let mut out = Vec::new();
      // 非标识符源先快照: 属性读/getter 只发生一次 (node 口径)
      let snap = if matches!(src, Expr::Identifier(_)) {
        src
      } else {
        let t = gen_name("u");
        out.push(Stmt::VarDecl {
          name: t.clone(),
          init: Some(src),
        });
        Expr::Identifier(t)
      };
      let cond = undefined_or(&snap, *fallback);
      match *inner {
        // 叶绑定: 条件表达式直接作初值, 免中间变量
        Pattern::Binding(name) => out.push(bind(name, cond, declare)),
        nested => {
          let t = gen_name("v");
          out.push(Stmt::VarDecl {
            name: t.clone(),
            init: Some(cond),
          });
          out.extend(destructure(nested, Expr::Identifier(t), declare));
        }
      }
      out
    }
    Pattern::Array { elements, rest } => {
      let mut out = Vec::new();
      let base = gen_name("d");
      out.push(Stmt::VarDecl {
        name: base.clone(),
        init: Some(src),
      });
      let local = Expr::Identifier(base);
      let len = elements.len();
      for (i, el) in elements.into_iter().enumerate() {
        if let Some(p) = el {
          out.extend(destructure(
            p,
            Expr::Index {
              object: Box::new(local.clone()),
              index: Box::new(Expr::Literal(JsValue::Number(i as f64))),
            },
            declare,
          ));
        }
      }
      if let Some(r) = rest {
        // `...rest` = 从已占位数起 slice (越界得 undefined.slice → 折 undefined 记账档)
        out.extend(destructure(
          *r,
          Expr::Call {
            callee: Box::new(Expr::Member {
              object: Box::new(local),
              property: SLICE_KEY.to_string(),
            }),
            args: vec![Expr::Literal(JsValue::Number(len as f64))],
          },
          declare,
        ));
      }
      out
    }
    Pattern::Object { props, rest } => {
      let mut out = Vec::new();
      let base = gen_name("d");
      out.push(Stmt::VarDecl {
        name: base.clone(),
        init: Some(src),
      });
      let used: Vec<String> = props.iter().map(|(k, _)| k.clone()).collect();
      for (key, p) in props {
        out.extend(destructure(
          p,
          Expr::Member {
            object: Box::new(Expr::Identifier(base.clone())),
            property: key,
          },
          declare,
        ));
      }
      if let Some(r) = rest {
        // 剩余对象 = 浅拷贝后按消费键逐个 delete: Object.assign({}, base)
        let rec = gen_name("r");
        out.push(Stmt::VarDecl {
          name: rec.clone(),
          init: Some(Expr::Call {
            callee: Box::new(Expr::Member {
              object: Box::new(Expr::Identifier("Object".to_string())),
              property: "assign".to_string(),
            }),
            args: vec![Expr::Object(Vec::new()), Expr::Identifier(base)],
          }),
        });
        for k in used {
          out.push(Stmt::Expr(Expr::Unary {
            op: UnaryOp::Delete,
            expr: Box::new(Expr::Member {
              object: Box::new(Expr::Identifier(rec.clone())),
              property: k,
            }),
          }));
        }
        out.extend(destructure(*r, Expr::Identifier(rec), declare));
      }
      out
    }
  }
}

/// 叶写入: declare=true 走 var 声明, false 是普通赋值 (语句位 `[a]=b` 写既有绑定)
fn bind(name: String, value: Expr, declare: bool) -> Stmt {
  if declare {
    Stmt::VarDecl {
      name,
      init: Some(value),
    }
  } else {
    Stmt::Expr(Expr::Assign {
      target: Box::new(Expr::Identifier(name)),
      op: AssignOp::Assign,
      value: Box::new(value),
    })
  }
}

/// `src === undefined ? fallback : src` (node 口径: 仅 undefined 触发默认值, null 不触发)
fn undefined_or(src: &Expr, fallback: Expr) -> Expr {
  Expr::Conditional {
    condition: Box::new(Expr::Binary {
      op: BinaryOp::StrictEq,
      left: Box::new(src.clone()),
      right: Box::new(Expr::Literal(JsValue::Undefined)),
    }),
    consequent: Box::new(fallback),
    alternate: Box::new(src.clone()),
  }
}
