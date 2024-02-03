
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType{
    Number,
    Plus,
    Dot,
    Dup,
    Minus,
    Load,
    Store,
    Mem,
    Syscall3,
    Syscall1,
    While,
    EndWhile,
    Do,
    Equal,
    If,
    EndIf,
    Else
}

#[derive(Debug, Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub value: Option<String>,
}

pub struct Tokenizer{
    contents: String,
    index: usize,  
}

impl Tokenizer {
    pub fn new(contents: String) -> Tokenizer {
        Tokenizer{
            contents,
            index: 0,
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut buffer = String::new();
        while let Some(c) = self.peek(0) {
            if c.is_numeric() {
                buffer.push(self.consume());
                while let Some(c) = self.peek(0) {
                    if c.is_numeric(){
                        buffer.push(self.consume());
                    } else {
                        break;
                    }
                    
                }
                tokens.push(Token{
                    token_type: TokenType::Number,
                    value: Some(buffer.clone()),
                });
                buffer.clear();

            }
            else if c.is_alphabetic() {
                buffer.push(self.consume());
                while let Some(c) = self.peek(0) {
                    if c.is_alphanumeric(){
                        buffer.push(self.consume());
                    } else {
                        break;
                    }
                    
                }
                if buffer == "dup" {
                    tokens.push(Token{
                        token_type: TokenType::Dup,
                        value: None,
                    });
                }
                else if buffer == "mem"{
                    tokens.push(Token{
                        token_type: TokenType::Mem,
                        value: None,
                    });
                }
                else if buffer == "syscall3"{
                    tokens.push(Token{
                        token_type: TokenType::Syscall3,
                        value: None,
                    });
                }
                else if buffer == "syscall1"{
                    tokens.push(Token{
                        token_type: TokenType::Syscall1,
                        value: None,
                    });
                }
                else if buffer == "while"{
                    tokens.push(Token{
                        token_type: TokenType::While,
                        value: None,
                    });
                }
                else if buffer == "end_while"{
                    tokens.push(Token{
                        token_type: TokenType::EndWhile,
                        value: None,
                    });
                }
                else if buffer == "do"{
                    tokens.push(Token{
                        token_type: TokenType::Do,
                        value: None,
                    });
                }
                else if buffer == "if"{
                    tokens.push(Token{
                        token_type: TokenType::If,
                        value: None,
                    });
                }
                else if buffer == "end"{
                    tokens.push(Token{
                        token_type: TokenType::EndIf,
                        value: None,
                    });
                }
                else if buffer == "else"{
                    tokens.push(Token{
                        token_type: TokenType::Else,
                        value: None,
                    });
                }
                buffer.clear();
            }
            else if c == '+' {
                self.consume();
                tokens.push(Token{
                    token_type: TokenType::Plus,
                    value: None,
                });
            }
            else if c == '-' {
                self.consume();
                tokens.push(Token{
                    token_type: TokenType::Minus,
                    value: None,
                });
            }
            else if c == '.' {
                self.consume();
                tokens.push(Token{
                    token_type: TokenType::Dot,
                    value: None,
                });
            }
            else if c == '@' {
                self.consume();
                tokens.push(Token{
                    token_type: TokenType::Load,
                    value: None,
                });
            }
            else if c == '!' {
                self.consume();
                tokens.push(Token{
                    token_type: TokenType::Store,
                    value: None,
                });
            }
            else if c == '=' {
                self.consume();
                tokens.push(Token{
                    token_type: TokenType::Equal,
                    value: None,
                });
            }
            else if c.is_whitespace() {
                self.consume();
            }
            else {
                self.consume();
            }
            
            
        }
        tokens
    }
   

    fn peek(&self, ahead: usize) -> Option<char> {
        let index = self.index + ahead;
        if index < self.contents.len() {
            self.contents.chars().nth(index)
        } else {
            None
        }
    }

    fn consume(&mut self) -> char {
        if self.index < self.contents.len(){
            let c = self.contents[self.index..].chars().next().unwrap();
            self.index += 1;
            c
        }
        else {
            '\0'
        }
    }
}