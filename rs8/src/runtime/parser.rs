//! JavaScript 递归下降与 Pratt 表达式语法分析器 (Parser)
//!
//! 零拷贝 Token 所有权转移模式：关键字与符号原地 skip 跳过，
//! 标识符、字符串与数值字面量利用 std::mem::replace 就地 take 移入 AST 节点，彻底杜绝 Token 复制

use std::{mem::replace, sync::Arc};

use itoa::Buffer;

use crate::{
  error::{Error, Result},
  runtime::{
    ast::*,
    lexer::{Token, TokenKind},
    value::{JsValue, SharedFunctionInfo},
  },
};

pub struct Parser {
  tokens: Vec<Token>,
  cursor: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self { tokens, cursor: 0 }
  }

  #[inline]
  fn peek(&self) -> &TokenKind {
    if self.cursor < self.tokens.len() {
      &self.tokens[self.cursor].kind
    } else {
      &TokenKind::Eof
    }
  }

  #[inline]
  fn skip(&mut self) {
    if self.cursor < self.tokens.len() {
      self.cursor += 1;
    }
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

  pub fn parse_program(&mut self) -> Result<Vec<Stmt>> {
    let mut stmts = Vec::new();
    while self.peek() != &TokenKind::Eof {
      stmts.push(self.parse_statement()?);
    }
    Ok(stmts)
  }

  fn parse_statement(&mut self) -> Result<Stmt> {
    match self.peek() {
      TokenKind::Var | TokenKind::Let | TokenKind::Const => {
        self.skip();
        let mut decls = Vec::new();
        loop {
          let name = match self.take() {
            TokenKind::Identifier(s) => s,
            other => {
              return Err(Error::ParseError(format!(
                "Expected identifier after var/let/const, got {other:?}"
              )));
            }
          };
          let init = if self.match_token(&TokenKind::Equal) {
            Some(self.parse_expression(2)?)
          } else {
            None
          };
          decls.push((name, init));
          if !self.match_token(&TokenKind::Comma) {
            break;
          }
        }
        let _ = self.match_token(&TokenKind::Semicolon);
        if decls.len() == 1 {
          let (name, init) = decls.pop().unwrap();
          Ok(Stmt::VarDecl { name, init })
        } else {
          Ok(Stmt::VarDecls(decls))
        }
      }
      TokenKind::Function => {
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
        let mut params = Vec::new();
        if self.peek() != &TokenKind::RParen {
          loop {
            match self.take() {
              TokenKind::Identifier(p) => params.push(p),
              other => {
                return Err(Error::ParseError(format!(
                  "Expected parameter name, got {other:?}"
                )));
              }
            }
            if !self.match_token(&TokenKind::Comma) {
              break;
            }
          }
        }
        self.expect(&TokenKind::RParen)?;
        let body = self.parse_block()?;
        let shared = Arc::new(SharedFunctionInfo::new(Some(name), params, body));
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
        self.skip();
        let _ = self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::Break)
      }
      TokenKind::Continue => {
        self.skip();
        let _ = self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::Continue)
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
    Ok(stmts)
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
            let mut args = Vec::new();
            if self.peek() != &TokenKind::RParen {
              loop {
                args.push(self.parse_expression(2)?);
                if !self.match_token(&TokenKind::Comma) {
                  break;
                }
              }
            }
            self.expect(&TokenKind::RParen)?;
            left = Expr::Call {
              callee: Box::new(left),
              args,
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
          Ok(Expr::Literal(JsValue::String(s)))
        } else {
          unreachable!()
        }
      }
      TokenKind::RegExp { .. } => {
        if let TokenKind::RegExp { pattern, flags } = self.take() {
          let re = JsValue::new_regexp(&pattern, &flags)?;
          Ok(Expr::Literal(re))
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
        if let TokenKind::Identifier(id) = self.take() {
          Ok(Expr::Identifier(id))
        } else {
          unreachable!()
        }
      }
      TokenKind::LParen => {
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
        let mut items = Vec::new();
        if self.peek() != &TokenKind::RBracket {
          loop {
            items.push(self.parse_expression(2)?);
            if !self.match_token(&TokenKind::Comma) {
              break;
            }
          }
        }
        self.expect(&TokenKind::RBracket)?;
        Ok(Expr::Array(items))
      }
      TokenKind::LBrace => {
        self.skip();
        let mut entries = Vec::new();
        if self.peek() != &TokenKind::RBrace {
          loop {
            let key = if self.match_token(&TokenKind::LBracket) {
              let expr = self.parse_expression(0)?;
              self.expect(&TokenKind::RBracket)?;
              ObjectKey::Computed(expr)
            } else {
              let tok = self.take();
              let k_str = match tok {
                TokenKind::Identifier(k) => k,
                TokenKind::String(s) => s,
                TokenKind::Number(n) => {
                  if n.fract() == 0.0 && !n.is_nan() && !n.is_infinite() {
                    let mut buf = Buffer::new();
                    buf.format(n as i64).to_string()
                  } else {
                    n.to_string()
                  }
                }
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
              ObjectKey::Ident(k_str)
            };
            self.expect(&TokenKind::Colon)?;
            let val = self.parse_expression(2)?;
            entries.push((key, val));
            if !self.match_token(&TokenKind::Comma) {
              break;
            }
          }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(Expr::Object(entries))
      }
      TokenKind::Function => {
        self.skip();
        let name = if let TokenKind::Identifier(_) = self.peek() {
          if let TokenKind::Identifier(id) = self.take() {
            Some(id)
          } else {
            None
          }
        } else {
          None
        };
        self.expect(&TokenKind::LParen)?;
        let mut params = Vec::new();
        if self.peek() != &TokenKind::RParen {
          loop {
            match self.take() {
              TokenKind::Identifier(p) => params.push(p),
              other => {
                return Err(Error::ParseError(format!("Expected param, got {other:?}")));
              }
            }
            if !self.match_token(&TokenKind::Comma) {
              break;
            }
          }
        }
        self.expect(&TokenKind::RParen)?;
        let body = self.parse_block()?;
        let shared = Arc::new(SharedFunctionInfo::new(name, params, body));
        Ok(Expr::FunctionExpr(shared))
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
}
