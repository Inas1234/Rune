use crate::parser::{self, NodeStmt, Node};
use std::fmt::Write;
pub struct Generator{
    node: Node,
}

impl Generator {
    pub fn new(node: Node) -> Generator {
        Generator{
            node,
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
                    write!(stream, "    push {}\n", node.value).expect("Error writing push to stream");
                },
                NodeStmt::Print(_node) => {
                    write!(stream, "    ;; -- print --\n").expect("Error");
                    write!(stream, "    pop rdi\n").expect("Error writing print to stream");
                    write!(stream, "    call dump\n").expect("Error writing call to stream");
                },
                NodeStmt::Plus(_node) => {
                    write!(stream, "    ;; -- plus --\n").expect("Error");
                    write!(stream, "    pop rax\n").expect("Error writing pop to stream");
                    write!(stream, "    pop rbx\n").expect("Error writing pop to stream");
                    write!(stream, "    add rax, rbx\n").expect("Error writing add to stream");
                    write!(stream, "    push rax\n").expect("Error writing push to stream");
                },
            }
        }
        write!(stream, "    ;; -- exit --\n").expect("Error");
        write!(stream, "    mov rax, 60\n").expect("Error");
        write!(stream, "    mov rdi, 0\n").expect("Error");
        write!(stream, "    syscall\n").expect("Error");
        stream
    }
}