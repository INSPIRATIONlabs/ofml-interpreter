//! OFML Lexer - Based on official OFML 2.0 specification
//! See: docs/3d-rendering/ofml_analysis/ofml_spec.pdf

use logos::Logos;

/// Token types for the OFML language
/// Based on Section 3.2 of OFML specification
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\x0B\x0C]+")] // whitespace including VT, FF
#[logos(skip r"//[^\r\n]*")] // line comments
#[logos(skip r"#[^\r\n]*")] // hash-style line comments (used in some OFML files)
#[logos(skip r"/\*([^*]|\*[^/])*\*/")] // block comments
pub enum Token {
    // ============================================================
    // Keywords (Section 3.2.4)
    // ============================================================
    #[token("abstract")]
    Abstract,
    #[token("break")]
    Break,
    #[token("case")]
    Case,
    #[token("catch")]
    Catch,
    #[token("class")]
    Class,
    #[token("continue")]
    Continue,
    #[token("default")]
    Default,
    #[token("do")]
    Do,
    #[token("else")]
    Else,
    #[token("final")]
    Final,
    #[token("finally")]
    Finally,
    #[token("for")]
    For,
    #[token("foreach")]
    Foreach,
    #[token("func")]
    Func,
    #[token("goto")]
    Goto,
    #[token("if")]
    If,
    #[token("import")]
    Import,
    #[token("instanceof")]
    Instanceof,
    #[token("native")]
    Native,
    #[token("operator")]
    Operator,
    #[token("package")]
    Package,
    #[token("private")]
    Private,
    #[token("protected")]
    Protected,
    #[token("public")]
    Public,
    #[token("return")]
    Return,
    #[token("rule")]
    Rule,
    #[token("self")]
    SelfKw,
    #[token("static")]
    Static,
    #[token("super")]
    Super,
    #[token("switch")]
    Switch,
    #[token("throw")]
    Throw,
    #[token("transient")]
    Transient,
    #[token("try")]
    Try,
    #[token("var")]
    Var,
    #[token("while")]
    While,
    #[token("NULL")]
    Null,

    // ============================================================
    // Operators (Section 3.2.6) - ordered by length for matching
    // ============================================================

    // Triple-character operators
    #[token(">>>=")]
    GtGtGtEq,
    #[token(">>>")]
    GtGtGt,
    #[token("<<=")]
    LtLtEq,
    #[token(">>=")]
    GtGtEq,

    // Double-character operators
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,
    #[token("!!")]
    BangBang,
    #[token("==")]
    EqEq,
    #[token("!=")]
    BangEq,
    #[token("~=")]
    TildeEq,
    #[token("<=")]
    LtEq,
    #[token(">=")]
    GtEq,
    #[token("<<")]
    LtLt,
    #[token(">>")]
    GtGt,
    #[token("&&")]
    AmpAmp,
    #[token("||")]
    PipePipe,
    #[token("=>")]
    FatArrow,
    #[token("<?")]
    LtQuestion,
    #[token(">?")]
    GtQuestion,
    #[token("*=")]
    StarEq,
    #[token("/=")]
    SlashEq,
    #[token("%=")]
    PercentEq,
    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("&=")]
    AmpEq,
    #[token("^=")]
    CaretEq,
    #[token("|=")]
    PipeEq,
    #[token("::")]
    ColonColon,
    #[token("@(")]
    AtLParen,

    // Single-character operators
    #[token(".")]
    Dot,
    #[token("!")]
    Bang,
    #[token("~")]
    Tilde,
    #[token("$")]
    Dollar,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("&")]
    Amp,
    #[token("^")]
    Caret,
    #[token("|")]
    Pipe,
    #[token("?")]
    Question,
    #[token(":")]
    Colon,
    #[token("=")]
    Eq,
    #[token(",")]
    Comma,
    #[token("@")]
    At,

    // ============================================================
    // Delimiters (Section 3.2.7)
    // ============================================================
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(";")]
    Semi,
    #[token("...")]
    Ellipsis,

    // ============================================================
    // Literals (Section 3.2.5)
    // ============================================================

    // Hexadecimal integer: 0x... or 0X...
    #[regex(r"0[xX][0-9a-fA-F]+", |lex| {
        i64::from_str_radix(&lex.slice()[2..], 16).ok()
    })]
    HexInt(i64),

    // Octal integer: 0...
    #[regex(r"0[0-7]+", |lex| {
        i64::from_str_radix(&lex.slice()[1..], 8).ok()
    })]
    OctInt(i64),

    // Decimal integer
    #[regex(r"[1-9][0-9]*", |lex| lex.slice().parse::<i64>().ok())]
    DecInt(i64),

    // Zero (special case - could be start of octal but alone is 0)
    #[token("0", |_| Some(0i64))]
    Zero(i64),

    // Floating point numbers
    #[regex(r"[0-9]+\.[0-9]*([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>().ok())]
    #[regex(r"\.[0-9]+([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>().ok())]
    #[regex(r"[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

    // String literal (double quotes)
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        unescape_string(&s[1..s.len()-1])
    })]
    StringLit(String),

    // Character literal (single quotes) -> treated as integer
    #[regex(r"'([^'\\]|\\.)'", |lex| {
        let s = lex.slice();
        parse_char_literal(&s[1..s.len()-1])
    })]
    CharLit(i64),

    // Symbol literal: @identifier
    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice()[1..].to_string())]
    Symbol(String),

    // ============================================================
    // Identifier (Section 3.2.3)
    // ============================================================
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string(), priority = 1)]
    Ident(String),
}

/// Unescape string literals according to OFML spec (Section 3.2.5)
fn unescape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('a') => result.push('\x07'), // BEL
                Some('b') => result.push('\x08'), // BS
                Some('t') => result.push('\t'),   // HT
                Some('n') => result.push('\n'),   // NL
                Some('v') => result.push('\x0B'), // VT
                Some('f') => result.push('\x0C'), // FF
                Some('r') => result.push('\r'),   // CR
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some('\\') => result.push('\\'),
                Some('x') => {
                    // Hex escape: \xHH...
                    let mut hex = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_hexdigit() {
                            hex.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    if let Ok(code) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = char::from_u32(code) {
                            result.push(ch);
                        }
                    }
                }
                Some(c) if c.is_ascii_digit() && c != '8' && c != '9' => {
                    // Octal escape: \OOO
                    let mut oct = String::from(c);
                    for _ in 0..2 {
                        if let Some(&c) = chars.peek() {
                            if ('0'..='7').contains(&c) {
                                oct.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                    }
                    if let Ok(code) = u32::from_str_radix(&oct, 8) {
                        if let Some(ch) = char::from_u32(code) {
                            result.push(ch);
                        }
                    }
                }
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Parse character literal to integer value
fn parse_char_literal(s: &str) -> i64 {
    let unescaped = unescape_string(s);
    unescaped.chars().next().map(|c| c as i64).unwrap_or(0)
}

impl Token {
    /// Get integer value from any integer token type
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Token::DecInt(n)
            | Token::HexInt(n)
            | Token::OctInt(n)
            | Token::Zero(n)
            | Token::CharLit(n) => Some(*n),
            _ => None,
        }
    }

    /// Get float value
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Token::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Get string value
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Token::StringLit(s) => Some(s),
            _ => None,
        }
    }

    /// Check if token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Token::Abstract
                | Token::Break
                | Token::Case
                | Token::Catch
                | Token::Class
                | Token::Continue
                | Token::Default
                | Token::Do
                | Token::Else
                | Token::Final
                | Token::Finally
                | Token::For
                | Token::Foreach
                | Token::Func
                | Token::Goto
                | Token::If
                | Token::Import
                | Token::Instanceof
                | Token::Native
                | Token::Operator
                | Token::Package
                | Token::Private
                | Token::Protected
                | Token::Public
                | Token::Return
                | Token::Rule
                | Token::SelfKw
                | Token::Static
                | Token::Super
                | Token::Switch
                | Token::Throw
                | Token::Transient
                | Token::Try
                | Token::Var
                | Token::While
                | Token::Null
        )
    }
}

/// A token with source location information
#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub span: std::ops::Range<usize>,
}

/// Lexer error
#[derive(Debug, Clone)]
pub struct LexError {
    pub span: std::ops::Range<usize>,
    pub slice: String,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unexpected token '{}' at position {}",
            self.slice, self.span.start
        )
    }
}

impl std::error::Error for LexError {}

/// Tokenize source code into a vector of spanned tokens
pub fn tokenize(source: &str) -> Result<Vec<SpannedToken>, LexError> {
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => tokens.push(SpannedToken {
                token,
                span: lexer.span(),
            }),
            Err(_) => {
                return Err(LexError {
                    span: lexer.span(),
                    slice: source[lexer.span()].to_string(),
                });
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let tokens: Vec<_> = Token::lexer("package import class var func rule")
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(
            tokens,
            vec![
                Token::Package,
                Token::Import,
                Token::Class,
                Token::Var,
                Token::Func,
                Token::Rule,
            ]
        );
    }

    #[test]
    fn test_all_keywords() {
        let source = "abstract break case catch class continue default do else \
                      final finally for foreach func goto if import instanceof \
                      native operator package private protected public return \
                      rule self static super switch throw transient try var while NULL";
        let tokens: Vec<_> = Token::lexer(source).filter_map(|r| r.ok()).collect();
        assert_eq!(tokens.len(), 36); // 36 keywords total
        assert!(tokens.iter().all(|t| t.is_keyword()));
    }

    #[test]
    fn test_symbols() {
        let tokens: Vec<_> = Token::lexer("@Width @GWK_ANBAU @_private")
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(
            tokens,
            vec![
                Token::Symbol("Width".to_string()),
                Token::Symbol("GWK_ANBAU".to_string()),
                Token::Symbol("_private".to_string()),
            ]
        );
    }

    #[test]
    fn test_integers() {
        let tokens: Vec<_> = Token::lexer("123 0 0x1F 0777")
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(tokens[0].as_int(), Some(123));
        assert_eq!(tokens[1].as_int(), Some(0));
        assert_eq!(tokens[2].as_int(), Some(31)); // 0x1F
        assert_eq!(tokens[3].as_int(), Some(511)); // 0777 octal
    }

    #[test]
    fn test_floats() {
        let tokens: Vec<_> = Token::lexer("3.14 .5 1e10 2.5e-3")
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(tokens[0].as_float(), Some(3.14));
        assert_eq!(tokens[1].as_float(), Some(0.5));
        assert_eq!(tokens[2].as_float(), Some(1e10));
        assert_eq!(tokens[3].as_float(), Some(2.5e-3));
    }

    #[test]
    fn test_strings() {
        let tokens: Vec<_> = Token::lexer(r#""hello" "world\n" "tab\there""#)
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(tokens[0].as_string(), Some("hello"));
        assert_eq!(tokens[1].as_string(), Some("world\n"));
        assert_eq!(tokens[2].as_string(), Some("tab\there"));
    }

    #[test]
    fn test_operators() {
        let tokens: Vec<_> = Token::lexer("+ - * / % == != <= >= && || ++ -- << >>")
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(
            tokens,
            vec![
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::Percent,
                Token::EqEq,
                Token::BangEq,
                Token::LtEq,
                Token::GtEq,
                Token::AmpAmp,
                Token::PipePipe,
                Token::PlusPlus,
                Token::MinusMinus,
                Token::LtLt,
                Token::GtGt,
            ]
        );
    }

    #[test]
    fn test_class_decl() {
        let source = "public class WkPlGroup: OiObject { }";
        let tokens: Vec<_> = Token::lexer(source).filter_map(|r| r.ok()).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Public,
                Token::Class,
                Token::Ident("WkPlGroup".to_string()),
                Token::Colon,
                Token::Ident("OiObject".to_string()),
                Token::LBrace,
                Token::RBrace,
            ]
        );
    }

    #[test]
    fn test_qualified_name() {
        let source = "::vitra::workit::*";
        let tokens: Vec<_> = Token::lexer(source).filter_map(|r| r.ok()).collect();
        assert_eq!(
            tokens,
            vec![
                Token::ColonColon,
                Token::Ident("vitra".to_string()),
                Token::ColonColon,
                Token::Ident("workit".to_string()),
                Token::ColonColon,
                Token::Star,
            ]
        );
    }

    #[test]
    fn test_symbol_array() {
        let source = "@(item1, item2)";
        let tokens: Vec<_> = Token::lexer(source).filter_map(|r| r.ok()).collect();
        assert_eq!(
            tokens,
            vec![
                Token::AtLParen,
                Token::Ident("item1".to_string()),
                Token::Comma,
                Token::Ident("item2".to_string()),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_real_cls_snippet() {
        let source = r#"
            var sAddTables = Hash();
            sAddTables[@W140] = ["89209701", ""];
        "#;
        let tokens: Vec<_> = Token::lexer(source).filter_map(|r| r.ok()).collect();
        // Should parse without errors
        assert!(tokens.len() > 10);
    }

    #[test]
    fn test_comments() {
        let source = r#"
            // This is a comment
            var x = 1; // inline comment
            /* block
               comment */
            var y = 2;
        "#;
        let tokens: Vec<_> = Token::lexer(source).filter_map(|r| r.ok()).collect();
        // Comments should be skipped
        let idents: Vec<_> = tokens
            .iter()
            .filter_map(|t| {
                if let Token::Ident(s) = t {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(idents, vec!["x", "y"]);
    }
}
