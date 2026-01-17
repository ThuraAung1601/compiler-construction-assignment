// Thura Aung
// 66011606

use std::env;
use std::fs;
use std::io::Write;

#[derive(Debug, Clone)]
enum TokenType {
    Int,
    Real,
    Var,
    List,
    Plus,
    Minus,
    Mult,
    Div,
    IntDiv,
    Pow,
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
    Assign,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Err,
}

impl TokenType {
    fn to_string(&self) -> &str {
        match self {
            TokenType::Int => "INT",
            TokenType::Real => "REAL",
            TokenType::Var => "VAR",
            TokenType::List => "list",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Mult => "*",
            TokenType::Div => "/",
            TokenType::IntDiv => "//",
            TokenType::Pow => "POW",
            TokenType::Gt => ">",
            TokenType::Gte => ">=",
            TokenType::Lt => "<",
            TokenType::Lte => "<=",
            TokenType::Eq => "==",
            TokenType::Neq => "!=",
            TokenType::Assign => "=",
            TokenType::LParen => "LPAREN",
            TokenType::RParen => "RPAREN",
            TokenType::LBracket => "LBRACKET",
            TokenType::RBracket => "RBRACKET",
            TokenType::Err => "ERR",
        }
    }
}

struct Token {
    lexeme: String,
    token_type: TokenType,
}

struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        if self.pos + offset < self.input.len() {
            Some(self.input[self.pos + offset])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn scan_number(&mut self) -> Token {
        let start = self.pos;
        let mut has_dot = false;
        let mut has_e = false;

        while let Some(ch) = self.current() {
            if ch.is_numeric() {
                self.advance();
            } else if ch == '.' && !has_dot && !has_e {
                has_dot = true;
                self.advance();
            } else if (ch == 'e' || ch == 'E') && !has_e {
                has_e = true;
                self.advance();
                if let Some(sign) = self.current() {
                    if sign == '+' || sign == '-' {
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }

        let lexeme: String = self.input[start..self.pos].iter().collect();
        let token_type = if has_dot || has_e {
            TokenType::Real
        } else {
            TokenType::Int
        };

        Token { lexeme, token_type }
    }

    fn scan_identifier(&mut self) -> Token {
        let start = self.pos;

        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme: String = self.input[start..self.pos].iter().collect();
        let token_type = if lexeme == "list" {
            TokenType::List
        } else {
            TokenType::Var
        };

        Token { lexeme, token_type }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if ch.is_numeric() {
                tokens.push(self.scan_number());
                continue;
            }

            if ch.is_alphabetic() || ch == '_' {
                tokens.push(self.scan_identifier());
                continue;
            }

            let token = match ch {
                '+' => {
                    self.advance();
                    Token {
                        lexeme: "+".to_string(),
                        token_type: TokenType::Plus,
                    }
                }
                '-' => {
                    self.advance();
                    Token {
                        lexeme: "-".to_string(),
                        token_type: TokenType::Minus,
                    }
                }
                '*' => {
                    self.advance();
                    Token {
                        lexeme: "*".to_string(),
                        token_type: TokenType::Mult,
                    }
                }
                '/' => {
                    self.advance();
                    if let Some('/') = self.current() {
                        self.advance();
                        Token {
                            lexeme: "//".to_string(),
                            token_type: TokenType::IntDiv,
                        }
                    } else {
                        Token {
                            lexeme: "/".to_string(),
                            token_type: TokenType::Div,
                        }
                    }
                }
                '^' => {
                    self.advance();
                    Token {
                        lexeme: "^".to_string(),
                        token_type: TokenType::Pow,
                    }
                }
                '>' => {
                    self.advance();
                    if let Some('=') = self.current() {
                        self.advance();
                        Token {
                            lexeme: ">=".to_string(),
                            token_type: TokenType::Gte,
                        }
                    } else {
                        Token {
                            lexeme: ">".to_string(),
                            token_type: TokenType::Gt,
                        }
                    }
                }
                '<' => {
                    self.advance();
                    if let Some('=') = self.current() {
                        self.advance();
                        Token {
                            lexeme: "<=".to_string(),
                            token_type: TokenType::Lte,
                        }
                    } else {
                        Token {
                            lexeme: "<".to_string(),
                            token_type: TokenType::Lt,
                        }
                    }
                }
                '=' => {
                    self.advance();
                    if let Some('=') = self.current() {
                        self.advance();
                        Token {
                            lexeme: "==".to_string(),
                            token_type: TokenType::Eq,
                        }
                    } else {
                        Token {
                            lexeme: "=".to_string(),
                            token_type: TokenType::Assign,
                        }
                    }
                }
                '!' => {
                    self.advance();
                    if let Some('=') = self.current() {
                        self.advance();
                        Token {
                            lexeme: "!=".to_string(),
                            token_type: TokenType::Neq,
                        }
                    } else {
                        Token {
                            lexeme: "!".to_string(),
                            token_type: TokenType::Err,
                        }
                    }
                }
                '(' => {
                    self.advance();
                    Token {
                        lexeme: "(".to_string(),
                        token_type: TokenType::LParen,
                    }
                }
                ')' => {
                    self.advance();
                    Token {
                        lexeme: ")".to_string(),
                        token_type: TokenType::RParen,
                    }
                }
                '[' => {
                    self.advance();
                    Token {
                        lexeme: "[".to_string(),
                        token_type: TokenType::LBracket,
                    }
                }
                ']' => {
                    self.advance();
                    Token {
                        lexeme: "]".to_string(),
                        token_type: TokenType::RBracket,
                    }
                }
                _ => {
                    let lexeme = ch.to_string();
                    self.advance();
                    Token {
                        lexeme,
                        token_type: TokenType::Err,
                    }
                }
            };

            tokens.push(token);
        }

        tokens
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let content = fs::read_to_string(input_file).expect("Failed to read input file");

    let mut output = Vec::new();

    for line in content.lines() {
        let mut lexer = Lexer::new(line.to_string());
        let tokens = lexer.scan_tokens();

        let mut token_strs = Vec::new();
        for token in tokens {
            token_strs.push(format!("{}/{}", token.lexeme, token.token_type.to_string()));
        }

        output.push(token_strs.join(" "));
    }

    let mut file = fs::File::create(output_file).expect("Error: failed to create output file");
    file.write_all(output.join("\n").as_bytes())
        .expect("Error: failed to write output");
}
