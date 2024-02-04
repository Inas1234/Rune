segment .text
        dump:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 64
    mov     QWORD  [rbp-56], rdi
    mov     QWORD  [rbp-8], 1
    mov     eax, 32
    sub     rax, QWORD  [rbp-8]
    mov     BYTE  [rbp-48+rax], 10
.L2:
    mov     rcx, QWORD  [rbp-56]
    mov  rdx, -3689348814741910323
    mov     rax, rcx
    mul     rdx
    shr     rdx, 3
    mov     rax, rdx
    sal     rax, 2
    add     rax, rdx
    add     rax, rax
    sub     rcx, rax
    mov     rdx, rcx
    mov     eax, edx
    lea     edx, [rax+48]
    mov     eax, 31
    sub     rax, QWORD  [rbp-8]
    mov     BYTE  [rbp-48+rax], dl
    add     QWORD  [rbp-8], 1
    mov     rax, QWORD  [rbp-56]
    mov     rdx, -3689348814741910323
    mul     rdx
    mov     rax, rdx
    shr     rax, 3
    mov     QWORD  [rbp-56], rax
    cmp     QWORD  [rbp-56], 0
    jne     .L2
    mov     eax, 32
    sub     rax, QWORD  [rbp-8]
    lea     rdx, [rbp-48]
    lea     rcx, [rdx+rax]
    mov     rax, QWORD  [rbp-8]
    mov     rdx, rax
    mov     rsi, rcx
    mov     edi, 1
    mov     rax, 1
    syscall
    nop
    leave
    ret
global _start
_start:
    ;; -- push --
    mov rax, 10
    push rax
    ;; -- push --
    mov rax, 0
    push rax
    ;; -- while --
.L_while_start_0:
    ;; -- dup2 --
    pop rax
    pop rbx
    push rbx
    push rax
    push rbx
    push rax
    ;; -- greater --
    mov rcx, 0
    mov rdx, 1
    pop rbx
    pop rax
    cmp rax, rbx
    cmovg rcx, rdx
    push rcx
    ;; -- do --
    pop rax
    test rax, rax
    jz .L_while_end_0
    ;; -- dup --
    pop rax
    push rax
    push rax
    ;; -- push --
    mov rax, 0
    push rax
    ;; -- while --
.L_while_start_1:
    ;; -- dup2 --
    pop rax
    pop rbx
    push rbx
    push rax
    push rbx
    push rax
    ;; -- greater --
    mov rcx, 0
    mov rdx, 1
    pop rbx
    pop rax
    cmp rax, rbx
    cmovg rcx, rdx
    push rcx
    ;; -- do --
    pop rax
    test rax, rax
    jz .L_while_end_1
    ;; -- mem --
    push mem
    ;; -- push --
    mov rax, 0
    push rax
    ;; -- plus --
    pop rax
    pop rbx
    add rax, rbx
    push rax
    ;; -- push --
    mov rax, 42
    push rax
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
    ;; -- push --
    mov rax, 1
    push rax
    ;; -- mem --
    push mem
    ;; -- push --
    mov rax, 1
    push rax
    ;; -- push --
    mov rax, 1
    push rax
    ;; -- syscall3 --
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
    ;; -- push --
    mov rax, 1
    push rax
    ;; -- plus --
    pop rax
    pop rbx
    add rax, rbx
    push rax
    jmp .L_while_start_1
.L_while_end_1:
    ;; -- drop --
    pop rax
    ;; -- drop --
    pop rax
    ;; -- mem --
    push mem
    ;; -- push --
    mov rax, 1
    push rax
    ;; -- plus --
    pop rax
    pop rbx
    add rax, rbx
    push rax
    ;; -- push --
    mov rax, 10
    push rax
    ;; -- store --
    pop rbx
    pop rax
    mov [rax], bl
    ;; -- push --
    mov rax, 1
    push rax
    ;; -- mem --
    push mem
    ;; -- push --
    mov rax, 1
    push rax
    ;; -- plus --
    pop rax
    pop rbx
    add rax, rbx
    push rax
    ;; -- push --
    mov rax, 1
    push rax
    ;; -- push --
    mov rax, 1
    push rax
    ;; -- syscall3 --
    pop rax
    pop rdi
    pop rsi
    pop rdx
    syscall
    ;; -- push --
    mov rax, 1
    push rax
    ;; -- plus --
    pop rax
    pop rbx
    add rax, rbx
    push rax
    jmp .L_while_start_0
.L_while_end_0:
    ;; -- exit --
    mov rax, 60
    mov rdi, 0
    syscall
section .bss
    mem resb 640000
