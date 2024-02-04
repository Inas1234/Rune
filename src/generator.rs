use crate::parser::{ NodeStmt, Node};
use std::fmt::Write;
use std::collections::VecDeque;

pub struct Generator{
    node: Node,
    ip: i32,
    if_count: i32,
    if_stack: VecDeque<i32>,    
    while_count: i32,
    while_stack: VecDeque<i32>,
}

impl Generator {
    pub fn new(node: Node) -> Generator {
        Generator{
            node,
            ip: 0,
            if_count: 0,
            if_stack: VecDeque::new(),
            while_count: 0,
            while_stack: VecDeque::new(),

        }
    }

    pub fn generate(&mut self) -> String {
        let mut stream = String::new();

        write!(stream, "segment .text\n").expect("Error");
        write!(stream,"        dump:\n").expect("Error");
        write!(stream,"    push    rbp\n").expect("Error");
        write!(stream,"    mov     rbp, rsp\n").expect("Error");
        write!(stream,"    sub     rsp, 64\n").expect("Error");
        write!(stream,"    mov     QWORD  [rbp-56], rdi\n").expect("Error");
        write!(stream,"    mov     QWORD  [rbp-8], 1\n").expect("Error");
        write!(stream,"    mov     eax, 32\n").expect("Error");
        write!(stream,"    sub     rax, QWORD  [rbp-8]\n").expect("Error");
        write!(stream,"    mov     BYTE  [rbp-48+rax], 10\n").expect("Error");
        write!(stream,".L2:\n").expect("Error");
        write!(stream,"    mov     rcx, QWORD  [rbp-56]\n").expect("Error");
        write!(stream,"    mov  rdx, -3689348814741910323\n").expect("Error");
        write!(stream,"    mov     rax, rcx\n").expect("Error");
        write!(stream,"    mul     rdx\n").expect("Error");
        write!(stream,"    shr     rdx, 3\n").expect("Error");
        write!(stream,"    mov     rax, rdx\n").expect("Error");
        write!(stream,"    sal     rax, 2\n").expect("Error");
        write!(stream,"    add     rax, rdx\n").expect("Error");
        write!(stream,"    add     rax, rax\n").expect("Error");
        write!(stream,"    sub     rcx, rax\n").expect("Error");
        write!(stream,"    mov     rdx, rcx\n").expect("Error");
        write!(stream,"    mov     eax, edx\n").expect("Error");
        write!(stream,"    lea     edx, [rax+48]\n").expect("Error");
        write!(stream,"    mov     eax, 31\n").expect("Error");
        write!(stream,"    sub     rax, QWORD  [rbp-8]\n").expect("Error");
        write!(stream,"    mov     BYTE  [rbp-48+rax], dl\n").expect("Error");
        write!(stream,"    add     QWORD  [rbp-8], 1\n").expect("Error");
        write!(stream,"    mov     rax, QWORD  [rbp-56]\n").expect("Error");
        write!(stream,"    mov     rdx, -3689348814741910323\n").expect("Error");
        write!(stream,"    mul     rdx\n").expect("Error");
        write!(stream,"    mov     rax, rdx\n").expect("Error");
        write!(stream,"    shr     rax, 3\n").expect("Error");
        write!(stream,"    mov     QWORD  [rbp-56], rax\n").expect("Error");
        write!(stream,"    cmp     QWORD  [rbp-56], 0\n").expect("Error");
        write!(stream,"    jne     .L2\n").expect("Error");
        write!(stream,"    mov     eax, 32\n").expect("Error");
        write!(stream,"    sub     rax, QWORD  [rbp-8]\n").expect("Error");
        write!(stream,"    lea     rdx, [rbp-48]\n").expect("Error");
        write!(stream,"    lea     rcx, [rdx+rax]\n").expect("Error");
        write!(stream,"    mov     rax, QWORD  [rbp-8]\n").expect("Error");
        write!(stream,"    mov     rdx, rax\n").expect("Error");
        write!(stream,"    mov     rsi, rcx\n").expect("Error");
        write!(stream,"    mov     edi, 1\n").expect("Error");
        write!(stream,"    mov     rax, 1\n").expect("Error");
        write!(stream,"    syscall\n").expect("Error");
        write!(stream,"    nop\n").expect("Error");
        write!(stream,"    leave\n").expect("Error");
        write!(stream,"    ret\n").expect("Error");

        write!(stream, "global _start\n").expect("Error");
        write!(stream, "_start:\n").expect("Error");

        for stmt in &self.node.stmts {
            match stmt {
                NodeStmt::Push(node) => {
                    write!(stream, "    ;; -- push --\n").expect("Error");
                    write!(stream, "    mov rax, {}\n", node.value).expect("Error");
                    write!(stream, "    push rax\n").expect("Error writing push to stream");
                    self.ip += 1;
                },
                NodeStmt::Print(_node) => {
                    write!(stream, "    ;; -- print --\n").expect("Error");
                    write!(stream, "    pop rdi\n").expect("Error writing print to stream");
                    write!(stream, "    call dump\n").expect("Error writing call to stream");
                    self.ip += 1;
                },
                NodeStmt::Plus(_node) => {
                    write!(stream, "    ;; -- plus --\n").expect("Error");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rbx\n").expect("Error writing pop to stream");
                    write!(stream, "    add rax, rbx\n").expect("Error writing add to stream");
                    write!(stream, "    push rax\n").expect("Error writing push to stream");
                    self.ip += 1;
                },
                NodeStmt::Dup(_node) => {
                    write!(stream, "    ;; -- dup --\n").expect("Error");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    push rax\n").expect("Error writing push to stream");
                    write!(stream, "    push rax\n").expect("Error writing push to stream");
                    self.ip += 1;
                },
                NodeStmt::Minus(_node) => {
                    write!(stream, "    ;; -- minus --\n").expect("Error");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rbx\n").expect("Error writing pop to stream");
                    write!(stream, "    sub rbx, rax\n").expect("Error writing sub to stream");
                    write!(stream, "    push rbx\n").expect("Error writing push to stream");
                    self.ip += 1;
                },
                NodeStmt::Load(_node) => {
                    write!(stream, "    ;; -- load --\n").expect("Error");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    xor rbx, rbx\n").expect("Error writing mov to stream");
                    write!(stream, "    mov bl, [rax]\n").expect("Error writing push to stream");
                    write!(stream, "    push rbx\n").expect("Error writing push to stream");
                    self.ip += 1;
                },
                NodeStmt::Store(_node) => {
                    write!(stream, "    ;; -- store --\n").expect("Error");
                    write!(stream, "    pop rbx\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    mov [rax], bl\n").expect("Error writing mov to stream");
                    self.ip += 1;
                },
                NodeStmt::Mem(_node) => {
                    write!(stream, "    ;; -- mem --\n").expect("Error");
                    write!(stream, "    push mem\n").expect("Error writing mov to stream");
                    self.ip += 1;
                },
                NodeStmt::Syscall3(_node) => {
                    write!(stream, "    ;; -- syscall3 --\n").expect("Error");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rdi\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rsi\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rdx\n").expect("Error writing mov to stream");
                    write!(stream, "    syscall\n").expect("Error writing syscall to stream");
                    self.ip += 1;
                },
                NodeStmt::Syscall1(_node) => {
                    write!(stream, "    ;; -- syscall1 --\n").expect("Error");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rdi\n").expect("Error writing pop to stream");
                    write!(stream, "    syscall\n").expect("Error writing syscall to stream");
                    self.ip += 1;
                },
                NodeStmt::Equal(_node) => {
                    write!(stream, "    ;; -- equal --\n").expect("Error");
                    write!(stream, "    mov rcx, 0\n").expect("Error writing pop to stream");
                    write!(stream, "    mov rdx, 1\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rbx\n").expect("Error writing pop to stream");
                    write!(stream, "    cmp rax, rbx\n").expect("Error writing cmp to stream");
                    write!(stream, "    cmove rcx, rdx\n").expect("Error writing je to stream");
                    write!(stream, "    push rcx\n").expect("Error writing push to stream");
                    self.ip += 1;
                },
                NodeStmt::Lesser(_node) => {
                    write!(stream, "    ;; -- lesser --\n").expect("Error");
                    write!(stream, "    mov rcx, 0\n").expect("Error writing pop to stream");
                    write!(stream, "    mov rdx, 1\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rbx\n").expect("Error writing pop to stream");
                    write!(stream, "    cmp rax, rbx\n").expect("Error writing cmp to stream");
                    write!(stream, "    cmovl rcx, rdx\n").expect("Error writing je to stream");
                    write!(stream, "    push rcx\n").expect("Error writing push to stream");
                    self.ip += 1;
                },
                NodeStmt::Greater(_node) => {
                    write!(stream, "    ;; -- greater --\n").expect("Error");
                    write!(stream, "    mov rcx, 0\n").expect("Error writing pop to stream");
                    write!(stream, "    mov rdx, 1\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rbx\n").expect("Error writing pop to stream");
                    write!(stream, "    cmp rax, rbx\n").expect("Error writing cmp to stream");
                    write!(stream, "    cmovg rcx, rdx\n").expect("Error writing je to stream");
                    write!(stream, "    push rcx\n").expect("Error writing push to stream");
                    self.ip += 1;
                },
                NodeStmt::If(_node) => {
                    let label = self.if_count;
                    self.if_stack.push_back(label); // Remember this if for a potential else
                    self.if_count += 1; // Prepare next label
                    
                    write!(stream, "    ;; -- if --\n").expect("Error");
                    write!(stream, "    pop rax\n").expect("Error");
                    write!(stream, "    test rax, rax\n").expect("Error");
                    write!(stream, "    jz .address_num_{}\n", label).expect("Error"); // Jump if false
                },
                NodeStmt::Else(_node) => {
                    let if_label = self.if_stack.pop_back().expect("No matching 'if' for 'else'");
                    let else_label = self.if_count; // Use the next label for the jump over the else
                    self.if_count += 1; // Increment for the next use
                    
                    write!(stream, "    jmp .address_num_{}\n", else_label).expect("Error"); // Jump over else
                    write!(stream, ".address_num_{}:\n", if_label).expect("Error"); // Start else block
                    self.if_stack.push_back(else_label); // Push else label for endif
                },
                NodeStmt::EndIf(_node) => {
                    if let Some(label) = self.if_stack.pop_back() {
                        write!(stream, ".address_num_{}:\n", label).expect("Error"); // Mark the end
                    } else {
                        panic!("Syntax error: 'endif' without matching 'if' or 'else'");
                    }
                },
                NodeStmt::While(_node) => {
                    let label = self.while_count;
                    self.while_stack.push_back(label);
                    self.while_count += 1;
                    
                    write!(stream, "    ;; -- while --\n").expect("Error");
                    write!(stream, ".L_while_start_{}:\n", label).expect("Error");
                },
                NodeStmt::Do(_node) => {
                    if let Some(label) = self.while_stack.back() {
                        write!(stream, "    ;; -- do --\n").expect("Error");
                        write!(stream, "    pop rax\n").expect("Error");
                        write!(stream, "    test rax, rax\n").expect("Error");
                        write!(stream, "    jz .L_while_end_{}\n", label).expect("Error"); 
                    } else {
                        panic!("Syntax error: 'do' without matching 'while'");
                    }
                },
                NodeStmt::EndWhile(_node) => {
                    if let Some(label) = self.while_stack.pop_back() {
                        write!(stream, "    jmp .L_while_start_{}\n", label).expect("Error"); 
                        write!(stream, ".L_while_end_{}:\n", label).expect("Error"); 
                    } else {
                        panic!("Syntax error: 'endwhile' without matching 'while'");
                    }
                },
                

            }
        }
        write!(stream, "    ;; -- exit --\n").expect("Error");
        write!(stream, "    mov rax, 60\n").expect("Error");
        write!(stream, "    mov rdi, 0\n").expect("Error");
        write!(stream, "    syscall\n").expect("Error");

        write!(stream, "section .bss\n").expect("Error");
        write!(stream, "    mem resb 640000\n").expect("Error");
        stream
    }
}