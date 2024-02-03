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
    mov rax, 34
    push rax
    ;; -- push --
    mov rax, 35
    push rax
    ;; -- plus --
    pop rax
    pop rbx
    add rax, rbx
    push rax
    ;; -- push --
    mov rax, 69
    push rax
    ;; -- equal --
    mov rcx, 0
    mov rdx, 1
    pop rax
    pop rbx
    cmp rax, rbx
    cmove rcx, rdx
    push rcx
    ;; -- if --
    pop rax
    test rax, rax
    je .address_num_0
    ;; -- push --
    mov rax, 0
    push rax
    ;; -- if --
    pop rax
    test rax, rax
    je .address_num_1
    ;; -- push --
    mov rax, 44
    push rax
    ;; -- print --
    pop rdi
    call dump
    jmp .address_num_2
.address_num_1:
    ;; -- push --
    mov rax, 55
    push rax
    ;; -- print --
    pop rdi
    call dump
.address_num_2:
    jmp .address_num_3
.address_num_0:
    ;; -- push --
    mov rax, 66
    push rax
    ;; -- print --
    pop rdi
    call dump
.address_num_3:
    ;; -- exit --
    mov rax, 60
    mov rdi, 0
    syscall
section .bss
    mem resb 640000
