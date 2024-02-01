
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType{
    Number,
    Plus,
    Dot,
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
            else if c == '+' {
                self.consume();
                tokens.push(Token{
                    token_type: TokenType::Plus,
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