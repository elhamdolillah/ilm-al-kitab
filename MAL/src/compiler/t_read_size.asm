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

    xor rcx, rcx
.rd_loop_2:
    lea rsi, [read_buf + rcx]
    xor rdi, rdi
    mov rdx, 1
    xor rax, rax
    push rcx
    syscall
    pop rcx
    test rax, rax
    jz .rd_end_2
    mov al, [read_buf + rcx]
    cmp al, 10
    je .rd_end_2
    inc rcx
    jmp .rd_loop_2
.rd_end_2:
    mov rax, rcx
    add rax, 8
    mov rdi, rax
    call arena_alloc
    mov [rax], rcx
    push rax
    push rcx
    lea rsi, [read_buf]
    lea rdi, [rax + 8]
.rd_copy_2:
    test rcx, rcx
    jz .rd_cdone_2
    mov al, [rsi]
    mov [rdi], al
    inc rsi
    inc rdi
    dec rcx
    jmp .rd_copy_2
.rd_cdone_2:
    pop rcx
    pop rax
    mov [vars + 0], rax
    mov rax, [vars + 0]
    mov rax, [rax]
    call print_int

    mov rax, 60
    xor rdi, rdi
    syscall
