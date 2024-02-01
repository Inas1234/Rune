mod tokenizer;
mod parser;
mod generator;
use std::fs;
use std::process::Command;


fn main() {
    let contents = fs::read_to_string("test.rn")
        .expect("Something went wrong reading the file");

    let mut tokenizer = tokenizer::Tokenizer::new(contents);
    let tokens = tokenizer.tokenize();
    let mut parser = parser::Parser::new(tokens);
    let node = parser.parse_program();
    let mut generator = generator::Generator::new(node.unwrap());
    let output = generator.generate();
    
    fs::write("test.asm", output).expect("Unable to write file");

    let _output = Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("test.asm")
        .output()
        .expect("failed to execute process");

    let output = Command::new("ld")
        .arg("-o")
        .arg("test")
        .arg("test.o")
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));


}
