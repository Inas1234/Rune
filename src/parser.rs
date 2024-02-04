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
pub struct NodeStmtStore{
}

#[derive(Debug)]
pub struct NodeStmtLoad{
}

#[derive(Debug)]
pub struct NodeStmtMem{}

#[derive(Debug)]
pub struct NodeStmtSyscall3{}

#[derive(Debug)]
pub struct NodeStmtSyscall1{}

#[derive(Debug)]
pub struct NodeStmtEqual{}

#[derive(Debug)]
pub struct NodeStmtWhile{}

#[derive(Debug)]
pub struct NodeStmtEndWhile{}

#[derive(Debug)]
pub struct NodeStmtDo{}

#[derive(Debug)]
pub struct NodeStmtIf{}

#[derive(Debug)]
pub struct NodeStmtEndIf{}

#[derive(Debug)]
pub struct NodeStmtElse{}

#[derive(Debug)]
pub struct NodeStmtLesser{}

#[derive(Debug)]
pub struct NodeStmtGreater{}

#[derive(Debug)]
pub struct NodeStmt2Dup{}

#[derive(Debug)]
pub struct NodeStmtOver{}

#[derive(Debug)]
pub struct NodeStmtSwap{}

#[derive(Debug)]
pub struct NodeStmtDrop{}

#[derive(Debug)]
pub enum NodeStmt{
    Push(NodeStmtPush),
    Print(NodeStmtPrint),
    Plus(NodeStmtPlus),
    Dup(NodeStmtDup),
    Minus(NodeStmtMinus),
    Store(NodeStmtStore),
    Load(NodeStmtLoad),
    Mem(NodeStmtMem),
    Syscall3(NodeStmtSyscall3),
    Syscall1(NodeStmtSyscall1),
    Equal(NodeStmtEqual),
    If(NodeStmtIf),
    EndIf(NodeStmtEndIf),
    Else(NodeStmtElse),
    While(NodeStmtWhile),
    EndWhile(NodeStmtEndWhile),
    Do(NodeStmtDo),
    Lesser(NodeStmtLesser),
    Greater(NodeStmtGreater),
    Dup2(NodeStmt2Dup),
    Over(NodeStmtOver),
    Swap(NodeStmtSwap),
    Drop(NodeStmtDrop),
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
    
    fn parse_load(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Load(NodeStmtLoad{
        }))
    }

    fn parse_store(&mut self) -> Option<NodeStmt> {
        self.consume();

        Some(NodeStmt::Store(NodeStmtStore{
        }))
    }

    fn parse_mem(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Mem(NodeStmtMem{
        }))
    }

    fn parse_syscall3(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Syscall3(NodeStmtSyscall3{
        }))
    }

    fn parse_syscall1(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Syscall1(NodeStmtSyscall1{
        }))
    }

    fn parse_equal(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Equal(NodeStmtEqual{
        }))
    }

    fn parse_if(&mut self) -> Option<NodeStmt> {
        self.consume();
        // Check if end_if is missing
        Some(NodeStmt::If(NodeStmtIf{
        }))
    }

    fn parse_end_if(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::EndIf(NodeStmtEndIf{
        }))
    }

    fn parse_else(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Else(NodeStmtElse{
        }))
    }

    fn parse_while(&mut self) -> Option<NodeStmt> {
        self.consume();
        // Check if end_while is missing
        Some(NodeStmt::While(NodeStmtWhile{
        }))
    }

    fn parse_end_while(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::EndWhile(NodeStmtEndWhile{
        }))
    }

    fn parse_do(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Do(NodeStmtDo{
        }))
    }

    fn parse_lesser(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Lesser(NodeStmtLesser{
        }))
    }

    fn parse_greater(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Greater(NodeStmtGreater{
        }))
    }

    fn parse_dup2(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Dup2(NodeStmt2Dup{
        }))
    }

    fn parse_over(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Over(NodeStmtOver{
        }))
    }

    fn parse_swap(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Swap(NodeStmtSwap{
        }))
    }

    fn parse_drop(&mut self) -> Option<NodeStmt> {
        self.consume();
        Some(NodeStmt::Drop(NodeStmtDrop{
        }))
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
                tokenizer::TokenType::Load => {
                    return self.parse_load();
                },
                tokenizer::TokenType::Store => {
                    return self.parse_store();
                },
                tokenizer::TokenType::Mem => {
                    return self.parse_mem();
                },
                tokenizer::TokenType::Syscall3 => {
                    return self.parse_syscall3();
                },
                tokenizer::TokenType::Syscall1 => {
                    return self.parse_syscall1();
                },
                tokenizer::TokenType::Equal => {
                    return self.parse_equal();
                },
                tokenizer::TokenType::If => {
                    return self.parse_if();
                },
                tokenizer::TokenType::EndIf => {
                    return self.parse_end_if();
                },
                tokenizer::TokenType::Else => {
                    return self.parse_else();
                },
                tokenizer::TokenType::While => {
                    return self.parse_while();
                },
                tokenizer::TokenType::EndWhile => {
                    return self.parse_end_while();
                },
                tokenizer::TokenType::Do => {
                    return self.parse_do();
                },
                tokenizer::TokenType::Lesser => {
                    return self.parse_lesser();
                },
                tokenizer::TokenType::Greater => {
                    return self.parse_greater();
                },
                tokenizer::TokenType::Dup2 => {
                    return self.parse_dup2();
                },
                tokenizer::TokenType::Over => {
                    return self.parse_over();
                },
                tokenizer::TokenType::Swap => {
                    return self.parse_swap();
                },
                tokenizer::TokenType::Drop => {
                    return self.parse_drop();
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