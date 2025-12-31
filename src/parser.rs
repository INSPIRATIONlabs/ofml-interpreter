//! OFML Parser - Recursive descent parser
//! Based on OFML 2.0 specification Sections 3.5-3.8

use crate::ast::*;
use crate::lexer::{tokenize, LexError, SpannedToken, Token};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Lexer error: {0}")]
    Lex(#[from] LexError),
    #[error("Unexpected token '{found}' at position {pos}, expected {expected}")]
    UnexpectedToken {
        found: String,
        expected: String,
        pos: usize,
    },
    #[error("Unexpected end of input, expected {expected}")]
    UnexpectedEof { expected: String },
    #[error("Invalid syntax: {message} at position {pos}")]
    InvalidSyntax { message: String, pos: usize },
}

pub type ParseResult<T> = Result<T, ParseError>;

/// OFML Parser
pub struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
}

impl Parser {
    /// Create a new parser from source code
    pub fn new(source: &str) -> ParseResult<Self> {
        let tokens = tokenize(source)?;
        Ok(Self { tokens, pos: 0 })
    }

    /// Parse a complete translation unit
    pub fn parse(&mut self) -> ParseResult<TranslationUnit> {
        let mut unit = TranslationUnit::default();

        // Optional package declaration
        if self.check(&Token::Package) {
            unit.package = Some(self.parse_package_decl()?);
        }

        // Import declarations
        while self.check(&Token::Import) {
            unit.imports.push(self.parse_import_decl()?);
        }

        // Statements
        while !self.is_at_end() {
            unit.statements.push(self.parse_stmt()?);
        }

        Ok(unit)
    }

    // ============================================================
    // Helpers
    // ============================================================

    fn current(&self) -> Option<&SpannedToken> {
        self.tokens.get(self.pos)
    }

    fn current_token(&self) -> Option<&Token> {
        self.current().map(|t| &t.token)
    }

    fn current_pos(&self) -> usize {
        self.current().map(|t| t.span.start).unwrap_or(0)
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn advance(&mut self) -> Option<&SpannedToken> {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1)
    }

    fn check(&self, token: &Token) -> bool {
        self.current_token().map(|t| t == token).unwrap_or(false)
    }

    #[allow(dead_code)]
    fn check_any(&self, tokens: &[Token]) -> bool {
        tokens.iter().any(|t| self.check(t))
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, token: &Token) -> ParseResult<&SpannedToken> {
        if self.check(token) {
            Ok(self.advance().unwrap())
        } else {
            Err(self.unexpected_token(&format!("{:?}", token)))
        }
    }

    fn unexpected_token(&self, expected: &str) -> ParseError {
        if let Some(t) = self.current() {
            ParseError::UnexpectedToken {
                found: format!("{:?}", t.token),
                expected: expected.to_string(),
                pos: t.span.start,
            }
        } else {
            ParseError::UnexpectedEof {
                expected: expected.to_string(),
            }
        }
    }

    fn span_from(&self, start: usize) -> Span {
        let end = self
            .tokens
            .get(self.pos.saturating_sub(1))
            .map(|t| t.span.end)
            .unwrap_or(start);
        Span::new(start, end)
    }

    // ============================================================
    // Package and Import
    // ============================================================

    fn parse_package_decl(&mut self) -> ParseResult<QualifiedName> {
        self.expect(&Token::Package)?;
        let name = self.parse_qualified_name()?;
        self.expect(&Token::Semi)?;
        Ok(name)
    }

    fn parse_import_decl(&mut self) -> ParseResult<ImportDecl> {
        let start = self.current_pos();
        self.expect(&Token::Import)?;
        let path = self.parse_qualified_name()?;

        // Check for wildcard ::*
        let wildcard = if self.check(&Token::ColonColon) {
            self.advance();
            self.expect(&Token::Star)?;
            true
        } else {
            false
        };

        self.expect(&Token::Semi)?;
        Ok(ImportDecl {
            path,
            wildcard,
            span: self.span_from(start),
        })
    }

    fn parse_qualified_name(&mut self) -> ParseResult<QualifiedName> {
        let start = self.current_pos();
        let absolute = self.match_token(&Token::ColonColon);
        let mut parts = Vec::new();

        // First identifier
        if let Some(Token::Ident(name)) = self.current_token().cloned() {
            self.advance();
            parts.push(name);
        } else {
            return Err(self.unexpected_token("identifier"));
        }

        // Additional parts
        while self.match_token(&Token::ColonColon) {
            if let Some(Token::Ident(name)) = self.current_token().cloned() {
                self.advance();
                parts.push(name);
            } else if self.check(&Token::Star) {
                // Wildcard - don't consume it here
                self.pos -= 1; // Back up the ::
                break;
            } else {
                break;
            }
        }

        Ok(QualifiedName {
            absolute,
            parts,
            span: self.span_from(start),
        })
    }

    // ============================================================
    // Statements
    // ============================================================

    fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        // Check for modifiers first
        let modifiers = self.parse_modifiers();

        match self.current_token() {
            Some(Token::Var) => self.parse_var_decl_list(modifiers),
            Some(Token::Class) => self.parse_class_decl(modifiers).map(Stmt::Class),
            Some(Token::Func) => self.parse_func_decl(modifiers).map(Stmt::Func),
            Some(Token::If) => self.parse_if_stmt(),
            Some(Token::Switch) => self.parse_switch_stmt(),
            Some(Token::While) => self.parse_while_stmt(),
            Some(Token::Do) => self.parse_do_while_stmt(),
            Some(Token::For) => self.parse_for_stmt(),
            Some(Token::Foreach) => self.parse_foreach_stmt(),
            Some(Token::Return) => self.parse_return_stmt(),
            Some(Token::Break) => self.parse_break_stmt(),
            Some(Token::Continue) => self.parse_continue_stmt(),
            Some(Token::Throw) => self.parse_throw_stmt(),
            Some(Token::Try) => self.parse_try_stmt(),
            Some(Token::LBrace) => self.parse_block().map(Stmt::Block),
            Some(Token::Semi) => {
                self.advance();
                Ok(Stmt::Empty)
            }
            _ => {
                if !modifiers.is_empty() {
                    return Err(ParseError::InvalidSyntax {
                        message: "Unexpected modifiers".to_string(),
                        pos: self.current_pos(),
                    });
                }
                self.parse_expr_stmt()
            }
        }
    }

    fn parse_modifiers(&mut self) -> Vec<Modifier> {
        let mut modifiers = Vec::new();
        loop {
            let modifier = match self.current_token() {
                Some(Token::Public) => Modifier::Public,
                Some(Token::Private) => Modifier::Private,
                Some(Token::Protected) => Modifier::Protected,
                Some(Token::Static) => Modifier::Static,
                Some(Token::Final) => Modifier::Final,
                Some(Token::Abstract) => Modifier::Abstract,
                Some(Token::Native) => Modifier::Native,
                Some(Token::Transient) => Modifier::Transient,
                _ => break,
            };
            self.advance();
            modifiers.push(modifier);
        }
        modifiers
    }

    /// Parse a variable declaration list as a statement: `var a, b = 1, c;`
    /// Returns Stmt::Var for single declaration, Stmt::VarList for multiple.
    fn parse_var_decl_list(&mut self, modifiers: Vec<Modifier>) -> ParseResult<Stmt> {
        self.expect(&Token::Var)?;

        let mut declarations = Vec::new();

        loop {
            let var_start = self.current_pos();

            // Parse variable name
            let name = if let Some(Token::Ident(name)) = self.current_token().cloned() {
                self.advance();
                name
            } else {
                return Err(self.unexpected_token("identifier"));
            };

            // Parse optional initializer
            let initializer = if self.match_token(&Token::Eq) {
                Some(self.parse_expr()?)
            } else {
                None
            };

            declarations.push(VarDecl {
                modifiers: modifiers.clone(),
                name,
                initializer,
                span: self.span_from(var_start),
            });

            // Check for more declarations
            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        self.expect(&Token::Semi)?;

        // Return single Var or VarList depending on count
        if declarations.len() == 1 {
            Ok(Stmt::Var(declarations.pop().unwrap()))
        } else {
            Ok(Stmt::VarList(declarations))
        }
    }

    /// Parse a single variable declaration (for use in for loops, etc.)
    fn parse_var_decl(&mut self, modifiers: Vec<Modifier>) -> ParseResult<VarDecl> {
        let start = self.current_pos();
        self.expect(&Token::Var)?;

        // Parse variable name
        let name = if let Some(Token::Ident(name)) = self.current_token().cloned() {
            self.advance();
            name
        } else {
            return Err(self.unexpected_token("identifier"));
        };

        let initializer = if self.match_token(&Token::Eq) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        self.expect(&Token::Semi)?;

        Ok(VarDecl {
            modifiers,
            name,
            initializer,
            span: self.span_from(start),
        })
    }

    fn parse_class_decl(&mut self, modifiers: Vec<Modifier>) -> ParseResult<ClassDecl> {
        let start = self.current_pos();
        self.expect(&Token::Class)?;

        let name = if let Some(Token::Ident(name)) = self.current_token().cloned() {
            self.advance();
            name
        } else {
            return Err(self.unexpected_token("class name"));
        };

        // Optional parent class
        let parent = if self.match_token(&Token::Colon) {
            Some(self.parse_qualified_name()?)
        } else {
            None
        };

        self.expect(&Token::LBrace)?;

        let mut members = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            members.push(self.parse_class_member()?);
        }

        self.expect(&Token::RBrace)?;

        Ok(ClassDecl {
            modifiers,
            name,
            parent,
            members,
            span: self.span_from(start),
        })
    }

    fn parse_class_member(&mut self) -> ParseResult<ClassMember> {
        let modifiers = self.parse_modifiers();

        match self.current_token() {
            Some(Token::Var) => self.parse_var_decl(modifiers).map(ClassMember::Var),
            Some(Token::Func) => self.parse_func_decl(modifiers).map(ClassMember::Func),
            Some(Token::Rule) => self.parse_rule_decl().map(ClassMember::Rule),
            _ => {
                // OFML allows expression statements at class level (e.g., hash initialization)
                if !modifiers.is_empty() {
                    return Err(ParseError::InvalidSyntax {
                        message: "Unexpected modifiers on expression".to_string(),
                        pos: self.current_pos(),
                    });
                }
                let expr = self.parse_expr()?;
                self.expect(&Token::Semi)?;
                Ok(ClassMember::Expr(expr))
            }
        }
    }

    fn parse_func_decl(&mut self, modifiers: Vec<Modifier>) -> ParseResult<FuncDecl> {
        let start = self.current_pos();
        self.expect(&Token::Func)?;

        let name = if let Some(Token::Ident(name)) = self.current_token().cloned() {
            self.advance();
            name
        } else {
            return Err(self.unexpected_token("function name"));
        };

        self.expect(&Token::LParen)?;

        let (params, variadic) = self.parse_param_list()?;

        self.expect(&Token::RParen)?;

        // Native functions have no body
        let body = if modifiers.contains(&Modifier::Native) || self.check(&Token::Semi) {
            if self.check(&Token::Semi) {
                self.advance();
            }
            None
        } else {
            Some(self.parse_block()?)
        };

        Ok(FuncDecl {
            modifiers,
            name,
            params,
            variadic,
            body,
            span: self.span_from(start),
        })
    }

    fn parse_rule_decl(&mut self) -> ParseResult<RuleDecl> {
        let start = self.current_pos();
        self.expect(&Token::Rule)?;

        let name = if let Some(Token::Ident(name)) = self.current_token().cloned() {
            self.advance();
            name
        } else {
            return Err(self.unexpected_token("rule name"));
        };

        self.expect(&Token::LParen)?;
        let (params, _) = self.parse_param_list()?;
        self.expect(&Token::RParen)?;

        let body = self.parse_block()?;

        Ok(RuleDecl {
            name,
            params,
            body,
            span: self.span_from(start),
        })
    }

    fn parse_param_list(&mut self) -> ParseResult<(Vec<String>, bool)> {
        let mut params = Vec::new();
        let mut variadic = false;

        if !self.check(&Token::RParen) {
            // First parameter
            if let Some(Token::Ident(name)) = self.current_token().cloned() {
                self.advance();
                params.push(name);
            } else {
                return Err(self.unexpected_token("parameter name"));
            }

            // Additional parameters
            while self.match_token(&Token::Comma) {
                if self.match_token(&Token::Ellipsis) {
                    variadic = true;
                    break;
                }
                if let Some(Token::Ident(name)) = self.current_token().cloned() {
                    self.advance();
                    params.push(name);
                } else {
                    return Err(self.unexpected_token("parameter name"));
                }
            }
        }

        Ok((params, variadic))
    }

    fn parse_block(&mut self) -> ParseResult<Block> {
        let start = self.current_pos();
        self.expect(&Token::LBrace)?;

        let mut stmts = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            stmts.push(self.parse_stmt()?);
        }

        self.expect(&Token::RBrace)?;

        Ok(Block {
            stmts,
            span: self.span_from(start),
        })
    }

    fn parse_if_stmt(&mut self) -> ParseResult<Stmt> {
        let start = self.current_pos();
        self.expect(&Token::If)?;
        self.expect(&Token::LParen)?;
        let condition = self.parse_expr()?;
        self.expect(&Token::RParen)?;

        let then_branch = Box::new(self.parse_stmt()?);

        let else_branch = if self.match_token(&Token::Else) {
            Some(Box::new(self.parse_stmt()?))
        } else {
            None
        };

        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
            span: self.span_from(start),
        }))
    }

    fn parse_switch_stmt(&mut self) -> ParseResult<Stmt> {
        let start = self.current_pos();
        self.expect(&Token::Switch)?;
        self.expect(&Token::LParen)?;
        let expr = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        self.expect(&Token::LBrace)?;

        let mut cases = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            let value = if self.match_token(&Token::Case) {
                let v = Some(self.parse_expr()?);
                self.expect(&Token::Colon)?;
                v
            } else if self.match_token(&Token::Default) {
                self.expect(&Token::Colon)?;
                None
            } else {
                return Err(self.unexpected_token("case or default"));
            };

            let mut stmts = Vec::new();
            while !self.check(&Token::Case)
                && !self.check(&Token::Default)
                && !self.check(&Token::RBrace)
                && !self.is_at_end()
            {
                stmts.push(self.parse_stmt()?);
            }

            cases.push(SwitchCase { value, stmts });
        }

        self.expect(&Token::RBrace)?;

        Ok(Stmt::Switch(SwitchStmt {
            label: None,
            expr,
            cases,
            span: self.span_from(start),
        }))
    }

    fn parse_while_stmt(&mut self) -> ParseResult<Stmt> {
        let start = self.current_pos();
        self.expect(&Token::While)?;
        self.expect(&Token::LParen)?;
        let condition = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        let body = Box::new(self.parse_stmt()?);

        Ok(Stmt::While(WhileStmt {
            label: None,
            condition,
            body,
            span: self.span_from(start),
        }))
    }

    fn parse_do_while_stmt(&mut self) -> ParseResult<Stmt> {
        let start = self.current_pos();
        self.expect(&Token::Do)?;
        let body = Box::new(self.parse_stmt()?);
        self.expect(&Token::While)?;
        self.expect(&Token::LParen)?;
        let condition = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        self.expect(&Token::Semi)?;

        Ok(Stmt::DoWhile(DoWhileStmt {
            label: None,
            body,
            condition,
            span: self.span_from(start),
        }))
    }

    fn parse_for_stmt(&mut self) -> ParseResult<Stmt> {
        let start = self.current_pos();
        self.expect(&Token::For)?;
        self.expect(&Token::LParen)?;

        // Init
        let init = if self.check(&Token::Semi) {
            self.advance();
            None
        } else if self.check(&Token::Var) {
            let stmt = self.parse_var_decl(vec![])?;
            Some(Box::new(Stmt::Var(stmt)))
        } else {
            let expr = self.parse_expr()?;
            self.expect(&Token::Semi)?;
            Some(Box::new(Stmt::Expr(expr)))
        };

        // Condition
        let condition = if self.check(&Token::Semi) {
            None
        } else {
            Some(self.parse_expr()?)
        };
        self.expect(&Token::Semi)?;

        // Update
        let update = if self.check(&Token::RParen) {
            None
        } else {
            Some(self.parse_expr()?)
        };
        self.expect(&Token::RParen)?;

        let body = Box::new(self.parse_stmt()?);

        Ok(Stmt::For(ForStmt {
            label: None,
            init,
            condition,
            update,
            body,
            span: self.span_from(start),
        }))
    }

    fn parse_foreach_stmt(&mut self) -> ParseResult<Stmt> {
        let start = self.current_pos();
        self.expect(&Token::Foreach)?;
        self.expect(&Token::LParen)?;

        let var_name = if let Some(Token::Ident(name)) = self.current_token().cloned() {
            self.advance();
            name
        } else {
            return Err(self.unexpected_token("variable name"));
        };

        self.expect(&Token::Semi)?;
        let iterable = self.parse_expr()?;
        self.expect(&Token::RParen)?;

        let body = Box::new(self.parse_stmt()?);

        Ok(Stmt::Foreach(ForeachStmt {
            label: None,
            var_name,
            iterable,
            body,
            span: self.span_from(start),
        }))
    }

    fn parse_return_stmt(&mut self) -> ParseResult<Stmt> {
        self.expect(&Token::Return)?;

        // return; or return expr;
        // Note: return(expr) is handled as return followed by a parenthesized expression,
        // which allows for cases like return (a + b) + c;
        let value = if self.check(&Token::Semi) || self.is_at_end() {
            None
        } else {
            Some(self.parse_expr()?)
        };

        self.expect(&Token::Semi)?;
        Ok(Stmt::Return(value))
    }

    fn parse_break_stmt(&mut self) -> ParseResult<Stmt> {
        self.expect(&Token::Break)?;
        let label = if let Some(Token::Ident(name)) = self.current_token().cloned() {
            self.advance();
            Some(name)
        } else {
            None
        };
        self.expect(&Token::Semi)?;
        Ok(Stmt::Break(label))
    }

    fn parse_continue_stmt(&mut self) -> ParseResult<Stmt> {
        self.expect(&Token::Continue)?;
        let label = if let Some(Token::Ident(name)) = self.current_token().cloned() {
            self.advance();
            Some(name)
        } else {
            None
        };
        self.expect(&Token::Semi)?;
        Ok(Stmt::Continue(label))
    }

    fn parse_throw_stmt(&mut self) -> ParseResult<Stmt> {
        self.expect(&Token::Throw)?;
        let expr = self.parse_expr()?;
        self.expect(&Token::Semi)?;
        Ok(Stmt::Throw(expr))
    }

    fn parse_try_stmt(&mut self) -> ParseResult<Stmt> {
        let start = self.current_pos();
        self.expect(&Token::Try)?;
        let try_block = self.parse_block()?;

        let (catch_var, catch_block) = if self.match_token(&Token::Catch) {
            self.expect(&Token::LParen)?;

            // Optional & for reference parameter
            let _is_ref = self.match_token(&Token::Amp);

            // Variable name
            let var = if let Some(Token::Ident(name)) = self.current_token().cloned() {
                self.advance();
                Some(name)
            } else {
                None
            };

            // Optional : Type annotation
            if self.match_token(&Token::Colon) {
                // Skip the type name (can be qualified)
                if self.check(&Token::ColonColon)
                    || matches!(self.current_token(), Some(Token::Ident(_)))
                {
                    let _ = self.parse_qualified_name();
                }
            }

            self.expect(&Token::RParen)?;
            (var, Some(self.parse_block()?))
        } else {
            (None, None)
        };

        let finally_block = if self.match_token(&Token::Finally) {
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Stmt::Try(TryStmt {
            try_block,
            catch_var,
            catch_block,
            finally_block,
            span: self.span_from(start),
        }))
    }

    fn parse_expr_stmt(&mut self) -> ParseResult<Stmt> {
        let expr = self.parse_expr()?;
        self.expect(&Token::Semi)?;
        Ok(Stmt::Expr(expr))
    }

    // ============================================================
    // Expressions (Operator Precedence Parsing)
    // ============================================================

    fn parse_expr(&mut self) -> ParseResult<Expr> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.parse_conditional()?;

        if let Some(op) = self.match_assign_op() {
            let value = self.parse_assignment()?;
            return Ok(Expr::Assign(AssignExpr {
                target: Box::new(expr),
                op,
                value: Box::new(value),
            }));
        }

        Ok(expr)
    }

    fn match_assign_op(&mut self) -> Option<AssignOp> {
        let op = match self.current_token() {
            Some(Token::Eq) => AssignOp::Assign,
            Some(Token::PlusEq) => AssignOp::AddAssign,
            Some(Token::MinusEq) => AssignOp::SubAssign,
            Some(Token::StarEq) => AssignOp::MulAssign,
            Some(Token::SlashEq) => AssignOp::DivAssign,
            Some(Token::PercentEq) => AssignOp::ModAssign,
            Some(Token::LtLtEq) => AssignOp::ShlAssign,
            Some(Token::GtGtEq) => AssignOp::ShrAssign,
            Some(Token::GtGtGtEq) => AssignOp::UshrAssign,
            Some(Token::AmpEq) => AssignOp::BitAndAssign,
            Some(Token::PipeEq) => AssignOp::BitOrAssign,
            Some(Token::CaretEq) => AssignOp::BitXorAssign,
            _ => return None,
        };
        self.advance();
        Some(op)
    }

    fn parse_conditional(&mut self) -> ParseResult<Expr> {
        let expr = self.parse_logical_or()?;

        if self.match_token(&Token::Question) {
            let then_expr = self.parse_expr()?;
            self.expect(&Token::Colon)?;
            let else_expr = self.parse_conditional()?;
            return Ok(Expr::Conditional(ConditionalExpr {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            }));
        }

        Ok(expr)
    }

    fn parse_logical_or(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_logical_and()?;

        while self.match_token(&Token::PipePipe) {
            let right = self.parse_logical_and()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_bitwise_or()?;

        while self.match_token(&Token::AmpAmp) {
            let right = self.parse_bitwise_or()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_bitwise_or(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_bitwise_xor()?;

        while self.match_token(&Token::Pipe) {
            let right = self.parse_bitwise_xor()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op: BinaryOp::BitOr,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_bitwise_xor(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_bitwise_and()?;

        while self.match_token(&Token::Caret) {
            let right = self.parse_bitwise_and()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op: BinaryOp::BitXor,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_bitwise_and(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_minmax()?;

        while self.match_token(&Token::Amp) {
            let right = self.parse_minmax()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op: BinaryOp::BitAnd,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_minmax(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_equality()?;

        loop {
            let op = if self.match_token(&Token::LtQuestion) {
                BinaryOp::Min
            } else if self.match_token(&Token::GtQuestion) {
                BinaryOp::Max
            } else {
                break;
            };

            let right = self.parse_equality()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_comparison()?;

        loop {
            let op = if self.match_token(&Token::EqEq) {
                BinaryOp::Eq
            } else if self.match_token(&Token::BangEq) {
                BinaryOp::Ne
            } else if self.match_token(&Token::TildeEq) {
                BinaryOp::PatternMatch
            } else {
                break;
            };

            let right = self.parse_comparison()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_shift()?;

        // Check for instanceof
        if self.match_token(&Token::Instanceof) {
            let type_name = self.parse_shift()?;
            return Ok(Expr::Instanceof(InstanceofExpr {
                expr: Box::new(left),
                type_name: Box::new(type_name),
            }));
        }

        loop {
            let op = if self.match_token(&Token::Lt) {
                BinaryOp::Lt
            } else if self.match_token(&Token::LtEq) {
                BinaryOp::Le
            } else if self.match_token(&Token::Gt) {
                BinaryOp::Gt
            } else if self.match_token(&Token::GtEq) {
                BinaryOp::Ge
            } else {
                break;
            };

            let right = self.parse_shift()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_shift(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_additive()?;

        loop {
            let op = if self.match_token(&Token::LtLt) {
                BinaryOp::Shl
            } else if self.match_token(&Token::GtGt) {
                BinaryOp::Shr
            } else if self.match_token(&Token::GtGtGt) {
                BinaryOp::Ushr
            } else {
                break;
            };

            let right = self.parse_additive()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = if self.match_token(&Token::Plus) {
                BinaryOp::Add
            } else if self.match_token(&Token::Minus) {
                BinaryOp::Sub
            } else {
                break;
            };

            let right = self.parse_multiplicative()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_unary()?;

        loop {
            let op = if self.match_token(&Token::Star) {
                BinaryOp::Mul
            } else if self.match_token(&Token::Slash) {
                BinaryOp::Div
            } else if self.match_token(&Token::Percent) {
                BinaryOp::Mod
            } else {
                break;
            };

            let right = self.parse_unary()?;
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> ParseResult<Expr> {
        if self.match_token(&Token::Plus) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                op: UnaryOp::Pos,
                operand: Box::new(operand),
            }));
        }

        if self.match_token(&Token::Minus) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
            }));
        }

        if self.match_token(&Token::Bang) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                op: UnaryOp::Not,
                operand: Box::new(operand),
            }));
        }

        if self.match_token(&Token::BangBang) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                op: UnaryOp::Test,
                operand: Box::new(operand),
            }));
        }

        if self.match_token(&Token::Tilde) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                op: UnaryOp::BitNot,
                operand: Box::new(operand),
            }));
        }

        if self.match_token(&Token::Dollar) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                op: UnaryOp::Resolve,
                operand: Box::new(operand),
            }));
        }

        if self.match_token(&Token::PlusPlus) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                op: UnaryOp::PreInc,
                operand: Box::new(operand),
            }));
        }

        if self.match_token(&Token::MinusMinus) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                op: UnaryOp::PreDec,
                operand: Box::new(operand),
            }));
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&Token::LParen) {
                // Function call
                let args = self.parse_arg_list()?;
                self.expect(&Token::RParen)?;
                expr = Expr::Call(CallExpr {
                    callee: Box::new(expr),
                    args,
                });
            } else if self.match_token(&Token::LBracket) {
                // Index or range access
                if self.check(&Token::Colon) {
                    // Range from start
                    self.advance();
                    let end = if self.check(&Token::RBracket) {
                        None
                    } else {
                        Some(Box::new(self.parse_expr()?))
                    };
                    self.expect(&Token::RBracket)?;
                    expr = Expr::Range(RangeExpr {
                        object: Box::new(expr),
                        start: None,
                        end,
                    });
                } else {
                    let index = self.parse_expr()?;
                    if self.match_token(&Token::Colon) {
                        // Range with start
                        let end = if self.check(&Token::RBracket) {
                            None
                        } else {
                            Some(Box::new(self.parse_expr()?))
                        };
                        self.expect(&Token::RBracket)?;
                        expr = Expr::Range(RangeExpr {
                            object: Box::new(expr),
                            start: Some(Box::new(index)),
                            end,
                        });
                    } else {
                        // Simple index
                        self.expect(&Token::RBracket)?;
                        expr = Expr::Index(IndexExpr {
                            object: Box::new(expr),
                            index: Box::new(index),
                        });
                    }
                }
            } else if self.match_token(&Token::Dot) {
                // Member access
                let member = if let Some(Token::Ident(name)) = self.current_token().cloned() {
                    self.advance();
                    name
                } else {
                    return Err(self.unexpected_token("member name"));
                };
                expr = Expr::Member(MemberExpr {
                    object: Box::new(expr),
                    member,
                });
            } else if self.match_token(&Token::PlusPlus) {
                expr = Expr::Unary(UnaryExpr {
                    op: UnaryOp::PostInc,
                    operand: Box::new(expr),
                });
            } else if self.match_token(&Token::MinusMinus) {
                expr = Expr::Unary(UnaryExpr {
                    op: UnaryOp::PostDec,
                    operand: Box::new(expr),
                });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_arg_list(&mut self) -> ParseResult<Vec<Expr>> {
        self.parse_expr_list(&Token::RParen)
    }

    fn parse_expr_list(&mut self, terminator: &Token) -> ParseResult<Vec<Expr>> {
        let mut args = Vec::new();

        if !self.check(terminator) {
            args.push(self.parse_expr()?);
            while self.match_token(&Token::Comma) {
                args.push(self.parse_expr()?);
            }
        }

        Ok(args)
    }

    fn parse_primary(&mut self) -> ParseResult<Expr> {
        // Literals
        if let Some(Token::DecInt(n)) = self.current_token().cloned() {
            self.advance();
            return Ok(Expr::Int(n));
        }
        if let Some(Token::HexInt(n)) = self.current_token().cloned() {
            self.advance();
            return Ok(Expr::Int(n));
        }
        if let Some(Token::OctInt(n)) = self.current_token().cloned() {
            self.advance();
            return Ok(Expr::Int(n));
        }
        if let Some(Token::Zero(n)) = self.current_token().cloned() {
            self.advance();
            return Ok(Expr::Int(n));
        }
        if let Some(Token::CharLit(n)) = self.current_token().cloned() {
            self.advance();
            return Ok(Expr::Int(n));
        }
        if let Some(Token::Float(f)) = self.current_token().cloned() {
            self.advance();
            return Ok(Expr::Float(f));
        }
        if let Some(Token::StringLit(s)) = self.current_token().cloned() {
            self.advance();
            return Ok(Expr::String(s));
        }
        if let Some(Token::Symbol(s)) = self.current_token().cloned() {
            self.advance();
            return Ok(Expr::Symbol(s));
        }

        // Keywords
        if self.match_token(&Token::Null) {
            return Ok(Expr::Null);
        }
        if self.match_token(&Token::SelfKw) {
            return Ok(Expr::SelfRef);
        }
        if self.match_token(&Token::Super) {
            return Ok(Expr::SuperRef);
        }

        // Array literal: [a, b, c] or []
        if self.match_token(&Token::LBracket) {
            let elements = self.parse_expr_list(&Token::RBracket)?;
            self.expect(&Token::RBracket)?;
            return Ok(Expr::Array(elements));
        }

        // List literal: @(a, b, c)
        if self.match_token(&Token::AtLParen) {
            let elements = self.parse_arg_list()?;
            self.expect(&Token::RParen)?;
            return Ok(Expr::List(elements));
        }

        // Parenthesized expression
        if self.match_token(&Token::LParen) {
            let expr = self.parse_expr()?;
            self.expect(&Token::RParen)?;
            return Ok(Expr::Paren(Box::new(expr)));
        }

        // Qualified name starting with ::
        if self.check(&Token::ColonColon) {
            let name = self.parse_qualified_name()?;
            return Ok(Expr::QualifiedName(name));
        }

        // Identifier or qualified name
        if let Some(Token::Ident(name)) = self.current_token().cloned() {
            self.advance();

            // Check if it's a qualified name
            if self.check(&Token::ColonColon) {
                // Need to reconstruct as qualified name
                let mut parts = vec![name];
                while self.match_token(&Token::ColonColon) {
                    if let Some(Token::Ident(part)) = self.current_token().cloned() {
                        self.advance();
                        parts.push(part);
                    } else {
                        break;
                    }
                }
                return Ok(Expr::QualifiedName(QualifiedName {
                    absolute: false,
                    parts,
                    span: Span::default(),
                }));
            }

            return Ok(Expr::Ident(name));
        }

        Err(self.unexpected_token("expression"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> ParseResult<TranslationUnit> {
        let mut parser = Parser::new(source)?;
        parser.parse()
    }

    #[test]
    fn test_package_decl() {
        let unit = parse("package ::vitra::workit;").unwrap();
        let pkg = unit.package.unwrap();
        assert!(pkg.absolute);
        assert_eq!(pkg.parts, vec!["vitra", "workit"]);
    }

    #[test]
    fn test_import_decl() {
        let unit = parse("import ::ofml::oi::*;").unwrap();
        assert_eq!(unit.imports.len(), 1);
        assert!(unit.imports[0].wildcard);
        assert_eq!(unit.imports[0].path.parts, vec!["ofml", "oi"]);
    }

    #[test]
    fn test_var_decl() {
        let unit = parse("var x = 42;").unwrap();
        assert_eq!(unit.statements.len(), 1);
        if let Stmt::Var(v) = &unit.statements[0] {
            assert_eq!(v.name, "x");
            assert!(matches!(v.initializer, Some(Expr::Int(42))));
        } else {
            panic!("Expected var declaration");
        }
    }

    #[test]
    fn test_class_decl() {
        let source = r#"
            public class MyClass: ParentClass {
                var x = 1;
                public func test() { }
            }
        "#;
        let unit = parse(source).unwrap();
        if let Stmt::Class(c) = &unit.statements[0] {
            assert_eq!(c.name, "MyClass");
            assert!(c.parent.is_some());
            assert_eq!(c.members.len(), 2);
        } else {
            panic!("Expected class declaration");
        }
    }

    #[test]
    fn test_func_decl() {
        let source = "func add(a, b) { return(a + b); }";
        let unit = parse(source).unwrap();
        if let Stmt::Func(f) = &unit.statements[0] {
            assert_eq!(f.name, "add");
            assert_eq!(f.params, vec!["a", "b"]);
        } else {
            panic!("Expected function declaration");
        }
    }

    #[test]
    fn test_if_stmt() {
        let source = "if (x > 0) { y = 1; } else { y = 0; }";
        let unit = parse(source).unwrap();
        assert!(matches!(unit.statements[0], Stmt::If(_)));
    }

    #[test]
    fn test_while_stmt() {
        let source = "while (x > 0) { x = x - 1; }";
        let unit = parse(source).unwrap();
        assert!(matches!(unit.statements[0], Stmt::While(_)));
    }

    #[test]
    fn test_foreach_stmt() {
        let source = "foreach (item; items) { process(item); }";
        let unit = parse(source).unwrap();
        assert!(matches!(unit.statements[0], Stmt::Foreach(_)));
    }

    #[test]
    fn test_expressions() {
        let source = "var x = 1 + 2 * 3;";
        let unit = parse(source).unwrap();
        if let Stmt::Var(v) = &unit.statements[0] {
            // Should be 1 + (2 * 3) due to precedence
            assert!(matches!(v.initializer, Some(Expr::Binary(_))));
        }
    }

    #[test]
    fn test_method_call() {
        let source = "obj.method(arg1, arg2);";
        let unit = parse(source).unwrap();
        if let Stmt::Expr(Expr::Call(c)) = &unit.statements[0] {
            assert!(matches!(c.callee.as_ref(), Expr::Member(_)));
            assert_eq!(c.args.len(), 2);
        } else {
            panic!("Expected method call");
        }
    }

    #[test]
    fn test_array_literal() {
        let source = "var arr = [1, 2, 3];";
        let unit = parse(source).unwrap();
        if let Stmt::Var(v) = &unit.statements[0] {
            if let Some(Expr::Array(elements)) = &v.initializer {
                assert_eq!(elements.len(), 3);
            } else {
                panic!("Expected array literal");
            }
        }
    }

    #[test]
    fn test_real_cls_code() {
        let source = r#"
            package ::vitra::workit;
            import ::ofml::oi::*;

            public class WkPlGroupSingle: WkPlGroup
            {
                var sAddTables = Hash();

                public func initialize(pFa, pNa)
                {
                    WkPlGroup::initialize(pFa, pNa);
                }

                public func addTable(pWidth)
                {
                    var tRefObj = lastObj();
                    tRefObj.setPropValue(@GWK_ANBAU, "1");

                    if (tEl != NULL) {
                        tRefObj.setPropValue(@GWK_ANBAU, "1");
                    } else {
                        tRefObj.setPropValue(@GWK_ANBAU, "0");
                    }
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_var_decl_list() {
        // Test single declaration
        let source = "var x = 1;";
        let result = parse(source).expect("Should parse single var");
        assert_eq!(result.statements.len(), 1);
        match &result.statements[0] {
            Stmt::Var(decl) => assert_eq!(decl.name, "x"),
            _ => panic!("Expected Stmt::Var"),
        }

        // Test multiple declarations
        let source = "var a, b = 2, c;";
        let result = parse(source).expect("Should parse var list");
        assert_eq!(result.statements.len(), 1);
        match &result.statements[0] {
            Stmt::VarList(decls) => {
                assert_eq!(decls.len(), 3);
                assert_eq!(decls[0].name, "a");
                assert!(decls[0].initializer.is_none());
                assert_eq!(decls[1].name, "b");
                assert!(decls[1].initializer.is_some());
                assert_eq!(decls[2].name, "c");
                assert!(decls[2].initializer.is_none());
            }
            _ => panic!("Expected Stmt::VarList"),
        }

        // Test two declarations
        let source = "var x = 1, y = 2;";
        let result = parse(source).expect("Should parse two vars");
        assert_eq!(result.statements.len(), 1);
        match &result.statements[0] {
            Stmt::VarList(decls) => {
                assert_eq!(decls.len(), 2);
                assert_eq!(decls[0].name, "x");
                assert_eq!(decls[1].name, "y");
            }
            _ => panic!("Expected Stmt::VarList"),
        }
    }

    #[test]
    fn test_return_with_complex_expression() {
        // Test return with parenthesized expression followed by more operators
        // This was a regression where return(expr) was parsed specially and broke
        // expressions like return (a + b) + c;
        let source = "func test() { return (1 + 2) + 3; }";
        let result = parse(source).expect("Should parse return with complex expression");
        assert_eq!(result.statements.len(), 1);

        // Test return with nested parentheses
        let source = "func test() { return ((a + b) * c) + d; }";
        let result = parse(source).expect("Should parse return with nested parentheses");
        assert_eq!(result.statements.len(), 1);

        // Test return with simple parenthesized expression (should still work)
        let source = "func test() { return (x); }";
        let result = parse(source).expect("Should parse return with simple parentheses");
        assert_eq!(result.statements.len(), 1);

        // Test return without parentheses
        let source = "func test() { return x + 1; }";
        let result = parse(source).expect("Should parse return without parentheses");
        assert_eq!(result.statements.len(), 1);

        // Test empty return
        let source = "func test() { return; }";
        let result = parse(source).expect("Should parse empty return");
        assert_eq!(result.statements.len(), 1);
    }
}
