//! AST (Abstract Syntax Tree) types for OFML
//! Based on OFML 2.0 specification Sections 3.5-3.8

use std::fmt;

/// Source location for error reporting
#[derive(Debug, Clone, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// A node with source location
#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

// ============================================================
// Top-Level Constructs
// ============================================================

/// A complete translation unit (source file)
#[derive(Debug, Clone, Default)]
pub struct TranslationUnit {
    pub package: Option<QualifiedName>,
    pub imports: Vec<ImportDecl>,
    pub statements: Vec<Stmt>,
}

/// Package declaration: `package ::vitra::workit;`
pub type PackageDecl = QualifiedName;

/// Import declaration: `import ::ofml::oi::*;`
#[derive(Debug, Clone)]
pub struct ImportDecl {
    pub path: QualifiedName,
    pub wildcard: bool, // ends with ::*
    pub span: Span,
}

/// Qualified name: `::vitra::workit::ClassName`
#[derive(Debug, Clone)]
pub struct QualifiedName {
    pub absolute: bool, // starts with ::
    pub parts: Vec<String>,
    pub span: Span,
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.absolute {
            write!(f, "::")?;
        }
        write!(f, "{}", self.parts.join("::"))
    }
}

// ============================================================
// Class Declaration
// ============================================================

/// Class declaration
#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub parent: Option<QualifiedName>,
    pub members: Vec<ClassMember>,
    pub span: Span,
}

/// Class member (variable, function, or rule)
#[derive(Debug, Clone)]
pub enum ClassMember {
    Var(VarDecl),
    Func(FuncDecl),
    Rule(RuleDecl),
    /// Expression statement (e.g., hash initialization: `sAddTables[@W140] = [...]`)
    Expr(Expr),
}

/// Access modifier
#[derive(Debug, Clone, PartialEq)]
pub enum Modifier {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Abstract,
    Native,
    Transient,
}

// ============================================================
// Variable Declaration
// ============================================================

/// Variable declaration: `var x = expr;`
#[derive(Debug, Clone)]
pub struct VarDecl {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub initializer: Option<Expr>,
    pub span: Span,
}

// ============================================================
// Function Declaration
// ============================================================

/// Function declaration: `func name(params) { body }`
#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub params: Vec<String>,
    pub variadic: bool,      // has ... parameter
    pub body: Option<Block>, // None for native functions
    pub span: Span,
}

/// Rule declaration: `rule name(params) { body }`
#[derive(Debug, Clone)]
pub struct RuleDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Block,
    pub span: Span,
}

// ============================================================
// Statements
// ============================================================

/// Statement
#[derive(Debug, Clone)]
pub enum Stmt {
    /// Single variable declaration
    Var(VarDecl),
    /// Multiple variable declarations: `var a, b = 1, c;`
    VarList(Vec<VarDecl>),
    /// Class declaration
    Class(ClassDecl),
    /// Function declaration
    Func(FuncDecl),
    /// Expression statement
    Expr(Expr),
    /// Block statement
    Block(Block),
    /// If statement
    If(IfStmt),
    /// Switch statement
    Switch(SwitchStmt),
    /// While loop
    While(WhileStmt),
    /// Do-while loop
    DoWhile(DoWhileStmt),
    /// For loop
    For(ForStmt),
    /// Foreach loop
    Foreach(ForeachStmt),
    /// Return statement
    Return(Option<Expr>),
    /// Break statement
    Break(Option<String>), // optional label
    /// Continue statement
    Continue(Option<String>), // optional label
    /// Throw statement
    Throw(Expr),
    /// Try-catch-finally
    Try(TryStmt),
    /// Empty statement
    Empty,
}

/// Block: `{ statements }`
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// If statement: `if (expr) stmt [else stmt]`
#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
    pub span: Span,
}

/// Switch statement
#[derive(Debug, Clone)]
pub struct SwitchStmt {
    pub label: Option<String>,
    pub expr: Expr,
    pub cases: Vec<SwitchCase>,
    pub span: Span,
}

/// Switch case or default
#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub value: Option<Expr>, // None for default
    pub stmts: Vec<Stmt>,
}

/// While loop: `while (expr) stmt`
#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub label: Option<String>,
    pub condition: Expr,
    pub body: Box<Stmt>,
    pub span: Span,
}

/// Do-while loop: `do stmt while (expr);`
#[derive(Debug, Clone)]
pub struct DoWhileStmt {
    pub label: Option<String>,
    pub body: Box<Stmt>,
    pub condition: Expr,
    pub span: Span,
}

/// For loop: `for (init; cond; update) stmt`
#[derive(Debug, Clone)]
pub struct ForStmt {
    pub label: Option<String>,
    pub init: Option<Box<Stmt>>,
    pub condition: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Box<Stmt>,
    pub span: Span,
}

/// Foreach loop: `foreach (var; expr) stmt`
#[derive(Debug, Clone)]
pub struct ForeachStmt {
    pub label: Option<String>,
    pub var_name: String,
    pub iterable: Expr,
    pub body: Box<Stmt>,
    pub span: Span,
}

/// Try-catch-finally
#[derive(Debug, Clone)]
pub struct TryStmt {
    pub try_block: Block,
    pub catch_var: Option<String>,
    pub catch_block: Option<Block>,
    pub finally_block: Option<Block>,
    pub span: Span,
}

// ============================================================
// Expressions
// ============================================================

/// Expression
#[derive(Debug, Clone)]
pub enum Expr {
    /// Integer literal
    Int(i64),
    /// Float literal
    Float(f64),
    /// String literal
    String(String),
    /// Symbol literal: @name
    Symbol(String),
    /// NULL
    Null,
    /// Self reference
    SelfRef,
    /// Super reference
    SuperRef,
    /// Identifier
    Ident(String),
    /// Qualified name: ::pkg::name
    QualifiedName(QualifiedName),
    /// Array/Vector literal: [a, b, c]
    Array(Vec<Expr>),
    /// List literal: @(a, b, c)
    List(Vec<Expr>),
    /// Binary operation
    Binary(BinaryExpr),
    /// Unary operation
    Unary(UnaryExpr),
    /// Conditional expression: cond ? then : else
    Conditional(ConditionalExpr),
    /// Assignment
    Assign(AssignExpr),
    /// Function call: func(args)
    Call(CallExpr),
    /// Index access: `expr[index]`
    Index(IndexExpr),
    /// Range access: `expr[start:end]`
    Range(RangeExpr),
    /// Member access: expr.name
    Member(MemberExpr),
    /// Instanceof check: expr instanceof Type
    Instanceof(InstanceofExpr),
    /// Parenthesized expression
    Paren(Box<Expr>),
}

/// Binary expression
#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    PatternMatch, // ~=
    // Logical
    And,
    Or,
    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Ushr, // >>>
    // Min/Max
    Min, // <?
    Max, // >?
}

/// Unary expression
#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub operand: Box<Expr>,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,     // -
    Pos,     // +
    Not,     // !
    BitNot,  // ~
    Test,    // !!
    Resolve, // $
    PreInc,  // ++x
    PreDec,  // --x
    PostInc, // x++
    PostDec, // x--
}

/// Conditional expression: cond ? then : else
#[derive(Debug, Clone)]
pub struct ConditionalExpr {
    pub condition: Box<Expr>,
    pub then_expr: Box<Expr>,
    pub else_expr: Box<Expr>,
}

/// Assignment expression
#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub target: Box<Expr>,
    pub op: AssignOp,
    pub value: Box<Expr>,
}

/// Assignment operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssignOp {
    Assign,       // =
    AddAssign,    // +=
    SubAssign,    // -=
    MulAssign,    // *=
    DivAssign,    // /=
    ModAssign,    // %=
    ShlAssign,    // <<=
    ShrAssign,    // >>=
    UshrAssign,   // >>>=
    BitAndAssign, // &=
    BitOrAssign,  // |=
    BitXorAssign, // ^=
}

/// Function call
#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

/// Index access: `expr[index]`
#[derive(Debug, Clone)]
pub struct IndexExpr {
    pub object: Box<Expr>,
    pub index: Box<Expr>,
}

/// Range access: `expr[start:end]`
#[derive(Debug, Clone)]
pub struct RangeExpr {
    pub object: Box<Expr>,
    pub start: Option<Box<Expr>>,
    pub end: Option<Box<Expr>>,
}

/// Member access: expr.name
#[derive(Debug, Clone)]
pub struct MemberExpr {
    pub object: Box<Expr>,
    pub member: String,
}

/// Instanceof expression
#[derive(Debug, Clone)]
pub struct InstanceofExpr {
    pub expr: Box<Expr>,
    pub type_name: Box<Expr>,
}

// ============================================================
// Utility Implementations
// ============================================================

impl From<i64> for Expr {
    fn from(n: i64) -> Self {
        Expr::Int(n)
    }
}

impl From<f64> for Expr {
    fn from(n: f64) -> Self {
        Expr::Float(n)
    }
}

impl From<String> for Expr {
    fn from(s: String) -> Self {
        Expr::String(s)
    }
}

impl From<&str> for Expr {
    fn from(s: &str) -> Self {
        Expr::String(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_new() {
        let span = Span::new(10, 20);
        assert_eq!(span.start, 10);
        assert_eq!(span.end, 20);
    }

    #[test]
    fn test_span_default() {
        let span = Span::default();
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 0);
    }

    #[test]
    fn test_span_debug_clone() {
        let span = Span::new(5, 15);
        let debug = format!("{:?}", span);
        assert!(debug.contains("Span"));
        let cloned = span.clone();
        assert_eq!(cloned.start, span.start);
        assert_eq!(cloned.end, span.end);
    }

    #[test]
    fn test_spanned_new() {
        let spanned = Spanned::new(42i64, Span::new(0, 5));
        assert_eq!(spanned.node, 42);
        assert_eq!(spanned.span.start, 0);
        assert_eq!(spanned.span.end, 5);
    }

    #[test]
    fn test_qualified_name_display_absolute() {
        let name = QualifiedName {
            absolute: true,
            parts: vec!["vitra".to_string(), "workit".to_string(), "Table".to_string()],
            span: Span::default(),
        };
        assert_eq!(format!("{}", name), "::vitra::workit::Table");
    }

    #[test]
    fn test_qualified_name_display_relative() {
        let name = QualifiedName {
            absolute: false,
            parts: vec!["local".to_string(), "Class".to_string()],
            span: Span::default(),
        };
        assert_eq!(format!("{}", name), "local::Class");
    }

    #[test]
    fn test_qualified_name_display_single() {
        let name = QualifiedName {
            absolute: false,
            parts: vec!["MyClass".to_string()],
            span: Span::default(),
        };
        assert_eq!(format!("{}", name), "MyClass");
    }

    #[test]
    fn test_import_decl() {
        let import = ImportDecl {
            path: QualifiedName {
                absolute: true,
                parts: vec!["ofml".to_string(), "oi".to_string()],
                span: Span::default(),
            },
            wildcard: true,
            span: Span::new(0, 20),
        };
        assert!(import.wildcard);
        assert_eq!(format!("{}", import.path), "::ofml::oi");
    }

    #[test]
    fn test_expr_from_i64() {
        let expr: Expr = 42i64.into();
        match expr {
            Expr::Int(n) => assert_eq!(n, 42),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_expr_from_f64() {
        let expr: Expr = 3.14f64.into();
        match expr {
            Expr::Float(n) => assert!((n - 3.14).abs() < f64::EPSILON),
            _ => panic!("Expected Float"),
        }
    }

    #[test]
    fn test_expr_from_string() {
        let expr: Expr = String::from("hello").into();
        match expr {
            Expr::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_expr_from_str() {
        let expr: Expr = "world".into();
        match expr {
            Expr::String(s) => assert_eq!(s, "world"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_translation_unit_default() {
        let tu = TranslationUnit::default();
        assert!(tu.package.is_none());
        assert!(tu.imports.is_empty());
        assert!(tu.statements.is_empty());
    }

    #[test]
    fn test_modifier_eq() {
        assert_eq!(Modifier::Public, Modifier::Public);
        assert_ne!(Modifier::Public, Modifier::Private);
        assert_eq!(Modifier::Static, Modifier::Static);
    }

    #[test]
    fn test_binary_op_debug() {
        let op = BinaryOp::Add;
        let debug = format!("{:?}", op);
        assert!(debug.contains("Add"));
    }

    #[test]
    fn test_unary_op_debug() {
        let op = UnaryOp::Not;
        let debug = format!("{:?}", op);
        assert!(debug.contains("Not"));
    }

    #[test]
    fn test_assign_op_eq() {
        assert_eq!(AssignOp::Assign, AssignOp::Assign);
        assert_ne!(AssignOp::Assign, AssignOp::AddAssign);
    }

    #[test]
    fn test_class_decl_debug() {
        let class = ClassDecl {
            modifiers: vec![Modifier::Public],
            name: "MyClass".to_string(),
            parent: None,
            members: vec![],
            span: Span::default(),
        };
        let debug = format!("{:?}", class);
        assert!(debug.contains("MyClass"));
    }

    #[test]
    fn test_var_decl() {
        let var = VarDecl {
            modifiers: vec![Modifier::Private],
            name: "counter".to_string(),
            initializer: Some(Expr::Int(0)),
            span: Span::new(10, 30),
        };
        assert_eq!(var.name, "counter");
        assert!(var.initializer.is_some());
    }

    #[test]
    fn test_func_decl() {
        let func = FuncDecl {
            modifiers: vec![Modifier::Public],
            name: "calculate".to_string(),
            params: vec!["a".to_string(), "b".to_string()],
            variadic: false,
            body: Some(Block {
                stmts: vec![],
                span: Span::default(),
            }),
            span: Span::default(),
        };
        assert_eq!(func.name, "calculate");
        assert_eq!(func.params.len(), 2);
        assert!(!func.variadic);
    }

    #[test]
    fn test_func_decl_variadic() {
        let func = FuncDecl {
            modifiers: vec![],
            name: "varargs".to_string(),
            params: vec!["args".to_string()],
            variadic: true,
            body: None,
            span: Span::default(),
        };
        assert!(func.variadic);
        assert!(func.body.is_none());
    }

    #[test]
    fn test_block_stmt() {
        let block = Block {
            stmts: vec![
                Stmt::Expr(Expr::Int(1)),
                Stmt::Expr(Expr::Int(2)),
            ],
            span: Span::new(0, 100),
        };
        assert_eq!(block.stmts.len(), 2);
    }

    #[test]
    fn test_call_expr() {
        let call = CallExpr {
            callee: Box::new(Expr::Ident("print".to_string())),
            args: vec![Expr::String("hello".to_string())],
        };
        assert_eq!(call.args.len(), 1);
    }

    #[test]
    fn test_index_expr() {
        let index = IndexExpr {
            object: Box::new(Expr::Ident("arr".to_string())),
            index: Box::new(Expr::Int(0)),
        };
        let debug = format!("{:?}", index);
        assert!(debug.contains("IndexExpr"));
    }

    #[test]
    fn test_member_expr() {
        let member = MemberExpr {
            object: Box::new(Expr::Ident("obj".to_string())),
            member: "field".to_string(),
        };
        assert_eq!(member.member, "field");
    }

    #[test]
    fn test_instanceof_expr() {
        let instance = InstanceofExpr {
            expr: Box::new(Expr::Ident("x".to_string())),
            type_name: Box::new(Expr::Ident("MyClass".to_string())),
        };
        let debug = format!("{:?}", instance);
        assert!(debug.contains("InstanceofExpr"));
    }

    #[test]
    fn test_if_stmt() {
        let if_stmt = IfStmt {
            condition: Expr::Int(1), // Non-zero as truthy condition
            then_branch: Box::new(Stmt::Empty),
            else_branch: None,
            span: Span::default(),
        };
        assert!(if_stmt.else_branch.is_none());
    }

    #[test]
    fn test_while_stmt() {
        let while_stmt = WhileStmt {
            label: None,
            condition: Expr::Int(1),
            body: Box::new(Stmt::Empty),
            span: Span::default(),
        };
        let debug = format!("{:?}", while_stmt);
        assert!(debug.contains("WhileStmt"));
    }

    #[test]
    fn test_for_stmt() {
        let for_stmt = ForStmt {
            label: None,
            init: None,
            condition: Some(Expr::Int(1)),
            update: None,
            body: Box::new(Stmt::Empty),
            span: Span::default(),
        };
        assert!(for_stmt.init.is_none());
        assert!(for_stmt.condition.is_some());
    }

    #[test]
    fn test_switch_case() {
        let case = SwitchCase {
            value: Some(Expr::Int(1)),
            stmts: vec![Stmt::Break(None)],
        };
        assert!(case.value.is_some());
        assert_eq!(case.stmts.len(), 1);
    }

    #[test]
    fn test_try_stmt() {
        let try_stmt = TryStmt {
            try_block: Block {
                stmts: vec![],
                span: Span::default(),
            },
            catch_var: Some("e".to_string()),
            catch_block: Some(Block {
                stmts: vec![],
                span: Span::default(),
            }),
            finally_block: None,
            span: Span::default(),
        };
        assert!(try_stmt.catch_var.is_some());
        assert!(try_stmt.finally_block.is_none());
    }

    #[test]
    fn test_all_binary_ops() {
        let ops = vec![
            BinaryOp::Add, BinaryOp::Sub, BinaryOp::Mul, BinaryOp::Div, BinaryOp::Mod,
            BinaryOp::Lt, BinaryOp::Le, BinaryOp::Gt, BinaryOp::Ge, BinaryOp::Eq, BinaryOp::Ne,
            BinaryOp::And, BinaryOp::Or, BinaryOp::BitAnd, BinaryOp::BitOr, BinaryOp::BitXor,
            BinaryOp::Shl, BinaryOp::Shr, BinaryOp::Ushr, BinaryOp::Min, BinaryOp::Max,
            BinaryOp::PatternMatch,
        ];
        for op in &ops {
            let debug = format!("{:?}", op);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_all_unary_ops() {
        let ops = vec![
            UnaryOp::Neg, UnaryOp::Pos, UnaryOp::Not, UnaryOp::BitNot,
            UnaryOp::Test, UnaryOp::Resolve,
            UnaryOp::PreInc, UnaryOp::PreDec, UnaryOp::PostInc, UnaryOp::PostDec,
        ];
        for op in &ops {
            let debug = format!("{:?}", op);
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_all_assign_ops() {
        let ops = vec![
            AssignOp::Assign, AssignOp::AddAssign, AssignOp::SubAssign,
            AssignOp::MulAssign, AssignOp::DivAssign, AssignOp::ModAssign,
            AssignOp::ShlAssign, AssignOp::ShrAssign, AssignOp::UshrAssign,
            AssignOp::BitAndAssign, AssignOp::BitOrAssign, AssignOp::BitXorAssign,
        ];
        for op in &ops {
            let cloned = *op;
            assert_eq!(cloned, *op);
        }
    }

    #[test]
    fn test_rule_decl() {
        let rule = RuleDecl {
            name: "onPropertyChange".to_string(),
            params: vec!["prop".to_string()],
            body: Block {
                stmts: vec![],
                span: Span::default(),
            },
            span: Span::default(),
        };
        assert_eq!(rule.name, "onPropertyChange");
    }

    #[test]
    fn test_class_member_variants() {
        let var_member = ClassMember::Var(VarDecl {
            modifiers: vec![],
            name: "x".to_string(),
            initializer: None,
            span: Span::default(),
        });
        let debug = format!("{:?}", var_member);
        assert!(debug.contains("Var"));

        let func_member = ClassMember::Func(FuncDecl {
            modifiers: vec![],
            name: "f".to_string(),
            params: vec![],
            variadic: false,
            body: None,
            span: Span::default(),
        });
        let debug = format!("{:?}", func_member);
        assert!(debug.contains("Func"));
    }
}
