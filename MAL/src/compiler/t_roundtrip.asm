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

    mov rax, 99
    mov rbx, 10
    mov rcx, 0
    lea rdi, [num_buf + 31]
.nts_6:
    xor rdx, rdx
    div rbx
    add dl, '0'
    dec rdi
    mov [rdi], dl
    inc rcx
    test rax, rax
    jnz .nts_6
    push rdi
    push rcx
    mov rax, rcx
    add rax, 8
    mov rdi, rax
    call arena_alloc
    pop rcx
    mov [rax], rcx
    pop rsi
    push rax
    lea rdi, [rax + 8]
.ntc_6:
    test rcx, rcx
    jz .ntd_6
    mov al, [rsi]
    mov [rdi], al
    inc rsi
    inc rdi
    dec rcx
    jmp .ntc_6
.ntd_6:
    pop rax
    mov rcx, [rax]
    lea rsi, [rax + 8]
    mov rax, 0
    mov rbx, 10
.std_5:
    test rcx, rcx
    jz .sdd_5
    movzx rdx, byte [rsi]
    cmp dl, '0'
    jl .std_skip_5
    cmp dl, '9'
    jg .std_skip_5
    sub dl, '0'
    imul rax, rbx
    add rax, rdx
.std_skip_5:
    inc rsi
    dec rcx
    jmp .std_5
.sdd_5:
    call print_int

    mov rax, 60
    xor rdi, rdi
    syscall
