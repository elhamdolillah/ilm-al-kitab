global _start
section .bss
    vars resq 256
    num_buf resb 32
    read_buf resb 256
    arena_ptr resq 1
    arena_mem resb 262144

section .text
arena_alloc:
    mov rax, [arena_ptr]
    add rdi, 15
    and rdi, -16
    add [arena_ptr], rdi
    ret

print_int:
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    mov rbx, 10
    mov rcx, 0
    lea rdi, [num_buf + 31]
.piloop:
    xor rdx, rdx
    div rbx
    add dl, '0'
    dec rdi
    mov [rdi], dl
    inc rcx
    test rax, rax
    jnz .piloop
    mov rsi, rdi
    mov byte [rsi + rcx], 10
    inc rcx
    mov rdi, 1
    mov rax, 1
    mov rdx, rcx
    syscall
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    ret

print_str:
    push rax
    push rdx
    push rsi
    push rdi
    mov rsi, rax
    add rsi, 8
    mov rdx, [rax]
    mov rdi, 1
    mov rax, 1
    syscall
    mov rsi, nl_ptr
    mov rdx, 1
    mov rdi, 1
    mov rax, 1
    syscall
    pop rdi
    pop rsi
    pop rdx
    pop rax
    ret

section .data
nl_ptr: db 10
section .text

_start:
    lea rax, [arena_mem]
    mov [arena_ptr], rax

    mov rax, 0
    mov [vars + 0], rax
    mov rax, 1
    mov [vars + 8], rax
.while_2:
    mov rax, [vars + 0]
    push rax
    mov rax, 5
    pop rbx
    cmp rbx, rax
    mov rax, 0
    setl al
    cmp rax, 0
    je .wend_2
    mov rax, [vars + 8]
    push rax
    mov rax, 2
    pop rbx
    imul rax, rbx
    mov [vars + 8], rax
    mov rax, [vars + 0]
    push rax
    mov rax, 1
    pop rbx
    add rax, rbx
    mov [vars + 0], rax
    jmp .while_2
.wend_2:
    mov rax, [vars + 8]
    call print_int

    mov rax, 60
    xor rdi, rdi
    syscall
