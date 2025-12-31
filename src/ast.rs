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
    /// Expression statement (e.g., hash initialization: sAddTables[@W140] = [...])
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
    /// Index access: expr[index]
    Index(IndexExpr),
    /// Range access: expr[start:end]
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

/// Index access: expr[index]
#[derive(Debug, Clone)]
pub struct IndexExpr {
    pub object: Box<Expr>,
    pub index: Box<Expr>,
}

/// Range access: expr[start:end]
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
