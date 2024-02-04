# Rune
## Description
Rune is a simple stack based programming language written in Rust. It is inspired by Forth and PostScript.  
## Syntax
Example if statement:
``` pascal
10 0 while dupp > do

    dup 0 while dupp > do
        "*" 1 1 syscall3
        1 +
    endWhile
    
    drop drop 
    "\n" 1 1 syscall3
    1 +

endWhile
```
## Features
- [x] Writting to memory
- [x] Reading from memory
- [x] Arithmetic operations
- [x] Conditional statements
- [x] Comments
- [x] Loops
- [x] Stings
- [ ] Standard library
- [ ] Macros
