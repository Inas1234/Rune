use crate::tokenizer::{self, Token};

#[derive(Debug)]
pub struct NodeStmtPush{
    pub value: i32,
}
#[derive(Debug)]
pub struct NodeStmtPrint{}

#[derive(Debug)]
pub struct NodeStmtPlus{}

#[derive(Debug)]
pub struct NodeStmtDup{}

#[derive(Debug)]
pub struct NodeStmtMinus{}

#[derive(Debug)]
pub enum NodeStmt{
    Push(NodeStmtPush),
    Print(NodeStmtPrint),
    Plus(NodeStmtPlus),
    Dup(NodeStmtDup),
    Minus(NodeStmtMinus),
}

pub struct Node{
    pub stmts: Vec<NodeStmt>,
}

impl Node {
    pub fn new() -> Node {
        Node{
            stmts: Vec::new(),
        }
    }
    pub fn add_stmt(&mut self, stmt: NodeStmt) {
        self.stmts.push(stmt);
    }
    
}

pub struct Parser{
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser{
            tokens,
            index: 0,
        }
    }

    fn parse_push(&mut self) -> Option<NodeStmt> {
        let token = self.consume().unwrap();
        let value = token.value.clone().unwrap().parse::<i32>().unwrap();
        Some(NodeStmt::Push(NodeStmtPush{
            value,
        }))
    }

    fn parse_plus(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Plus(NodeStmtPlus{}))
    }

    fn parse_print(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Print(NodeStmtPrint{}))
    }

    fn parse_dup(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Dup(NodeStmtDup{}))
    }

    fn parse_minus(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Minus(NodeStmtMinus{}))
    }
    
    pub fn parse(&mut self) -> Option<NodeStmt> {
        while let Some(token) = self.peek(0) {
            match token.token_type {
                tokenizer::TokenType::Number => {
                    return self.parse_push();
                },
                tokenizer::TokenType::Plus => {
                    return self.parse_plus();
                },
                tokenizer::TokenType::Dot => {
                    return self.parse_print();
                },
                tokenizer::TokenType::Dup => {
                    return self.parse_dup();
                },
                tokenizer::TokenType::Minus => {
                    return self.parse_minus();
                },
                _ => {
                    panic!("Unexpected token {:?}", token);
                }
            }
        }

        None
    }

    pub fn parse_program(&mut self) -> Option<Node> {
        let mut node = Node::new();
        while let Some(stmt) = self.parse() {
            node.add_stmt(stmt);
        }
        Some(node)
    }
    
    fn peek(&self, offset: usize) -> Option<&Token> {
        let index = self.index + offset;
        if index >= self.tokens.len() {
            None
        }
        else {
            Some(&self.tokens[index])
        }
    }
    
    fn consume(&mut self) -> Option<&Token> {
        if self.index < self.tokens.len() {
            let c = self.tokens.get(self.index);
            self.index += 1;
            c
        }
        else {
            None
        }
    }
    
}