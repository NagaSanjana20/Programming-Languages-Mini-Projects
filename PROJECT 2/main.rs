use std::fmt;
use std::io;

// Representing different token types in the input expression
#[derive(Debug, Clone)]
enum TokenType {
    Int(i32),
    Add,
    Sub,
    Mul,
    LParen,
    RParen,
}

// Representing different nodes in AST
#[derive(Debug)]
enum ASTNode {
    Sub(Box<ASTNode>, Box<ASTNode>), 
    Mul(Box<ASTNode>, Box<ASTNode>),
    Add(Box<ASTNode>, Box<ASTNode>),
    Number(i32),
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Sub(left, right) => {
                write!(f, "Sub(\n  {},\n  {}\n)", 
                    format!("{}", left).replace('\n', "\n  "),
                    format!("{}", right).replace('\n', "\n  "))
            }
            ASTNode::Mul(left, right) => {
                write!(f, "Mul(\n  {},\n  {}\n)", 
                    format!("{}", left).replace('\n', "\n  "),
                    format!("{}", right).replace('\n', "\n  "))
            }
            ASTNode::Add(left, right) => {
                write!(f, "Add(\n  {},\n  {}\n)", 
                    format!("{}", left).replace('\n', "\n  "),
                    format!("{}", right).replace('\n', "\n  "))
            }
            ASTNode::Number(val) => {
                write!(f, "Int {}", val)
            }
        }
    }
}

// LEXER - converts input string to tokens
struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_token(&mut self) -> Option<TokenType> {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }

        if self.position >= self.input.len() {
            return None;
        }

        let current = self.input[self.position];
        match current {
            '0'..='9' => {
                let mut num_str = String::new();
                while self.position < self.input.len() && self.input[self.position].is_digit(10) {
                    num_str.push(self.input[self.position]);
                    self.position += 1;
                }
                Some(TokenType::Int(num_str.parse().unwrap()))
            }
            '+' => {
                self.position += 1;
                Some(TokenType::Add)
            }
            '-' => {
                self.position += 1;
                Some(TokenType::Sub)
            }
            '*' => {
                self.position += 1;
                Some(TokenType::Mul)
            }
            '(' => {
                self.position += 1;
                Some(TokenType::LParen)
            }
            ')' => {
                self.position += 1;
                Some(TokenType::RParen)
            }
            _ => {
                if current == '−' || current == '–' { 
                    self.position += 1;
                    Some(TokenType::Sub)
                } else {
                    eprintln!("Unexpected character: {}", current);
                    self.position += 1;
                    None
                }
            }
        }
    }
}

// PARSER - converts tokens into AST(Abstract Syntax Tree)
struct Parser {
    lexer: Lexer,
    current_token: Option<TokenType>,
}

impl Parser {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn consume(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> Result<ASTNode, String> {
        self.parse_sub().ok_or_else(|| "Parsing failed".to_string())
    }

    fn parse_sub(&mut self) -> Option<ASTNode> {
        let mut left = self.parse_add()?;

        while let Some(TokenType::Sub) = self.current_token {
            self.consume();
            let right = self.parse_add()?;
            left = ASTNode::Sub(Box::new(left), Box::new(right));
        }

        Some(left)
    }

    fn parse_add(&mut self) -> Option<ASTNode> {
        let mut left = self.parse_mul()?;

        while let Some(TokenType::Add) = self.current_token {
            self.consume();
            let right = self.parse_mul()?;
            left = ASTNode::Add(Box::new(left), Box::new(right));
        }

        Some(left)
    }

    fn parse_mul(&mut self) -> Option<ASTNode> {
        let mut left = self.parse_primary()?;

        while let Some(TokenType::Mul) = self.current_token {
            self.consume();
            let right = self.parse_primary()?;
            left = ASTNode::Mul(Box::new(left), Box::new(right));
        }

        Some(left)
    }

    fn parse_primary(&mut self) -> Option<ASTNode> {
        match &self.current_token {
            Some(TokenType::Int(val)) => {
                let node = ASTNode::Number(*val);
                self.consume();
                Some(node)
            }
            Some(TokenType::LParen) => {
                self.consume();
                let expr = self.parse_sub();
                if let Some(TokenType::RParen) = self.current_token {
                    self.consume();
                    expr
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

// input the testcases as needed and print the output
fn main() {
    println!("Enter an arithmetic expression:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let input = input.trim();
    
    let mut parser = Parser::new(input);
    match parser.parse() {
        Ok(ast) => println!("{}", ast),
        Err(e) => {
            eprintln!("Parsing error: {}", e);
            std::process::exit(1);
        }
    }
}
