; Copyright (C) 2025 Filip Chovanec
;
; This program is free software: you can redistribute it and/or modify
; it under the terms of the GNU General Public License as published by
; the Free Software Foundation, either version 3 of the License, or
; (at your option) any later version.
;
; This program is distributed in the hope that it will be useful,
; but WITHOUT ANY WARRANTY; without even the implied warranty of
; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
; GNU General Public License for more details.
;
; You should have received a copy of the GNU General Public License
; along with this program.  If not, see <https://www.gnu.org/licenses/>.

STACK_SIZE equ 1000 ; * 8
PRINT_SIZE equ 128  ; * 1
ITEM_SIZE equ 8
CHAR_SPACE equ 32

section .bss
stack:  resq STACK_SIZE
sptr:   resq 1
msgbuf: resb PRINT_SIZE

section .text

; input => rdi
; uses rax rdi
data_push:
    mov rax, [sptr]
    mov [rax], rdi
    add qword [sptr], ITEM_SIZE
    ret

%macro mpush 1
    mov rdi, %1
    call data_push
%endmacro

; rax => output
; uses rax
data_pop: 
    mov rax, [sptr]
    sub qword [sptr], ITEM_SIZE
    mov rax, [sptr]
    mov rax, [rax]
    ret

; rbx => value
; uses rax rbx rdi
data_copy:
    call data_pop
    mov rbx, rax
    mpush rax
    mpush rbx
    ret

; uses rax rbx rcx rdi
data_swap:
    call data_pop
    mov rbx, rax
    call data_pop
    mov rcx, rax
    mpush rbx
    mpush rcx
    ret

; tested
builtin_add:
    call data_pop
    mov rbx, rax
    call data_pop
    add rax, rbx
    mpush rax
    ret

; tested
builtin_sub:
    call data_pop
    mov rbx, rax
    call data_pop
    sub rax, rbx
    mpush rax
    ret

; tested
builtin_mul:
    call data_pop
    mov rbx, rax
    call data_pop
    imul rax, rbx
    mpush rax
    ret

; src: ChatGPT
; tested
builtin_div:
    call data_pop
    mov rbx, rax
    call data_pop
    cqo
    idiv rbx
    mpush rax
    ret

; src: ChatGPT
; tested
builtin_mod:
    call data_pop
    mov rbx, rax
    call data_pop
    cqo
    idiv rbx
    mpush rdx
    ret


SYSCALL_WRITE equ 1
SYSCALL_EXIT equ 60
CODE_STDOUT equ 1
EXIT_OK equ 0

%macro linux_syscall 4
    mov rax, %1
    mov rdi, %2
    mov rsi, %3
    mov rdx, %4
    syscall
%endmacro

; This function is not correct lmao
; uses rax rdi rsi rdx
builtin_print_digit:
    call data_pop
    add rax, 48 ; convert number to its ascii representation
    mov [msgbuf], rax

    ; add a trailing space for clarity
    mov rax, msgbuf
    add rax, 1
    mov rbx, CHAR_SPACE
    mov [rax], rbx

    linux_syscall SYSCALL_WRITE, CODE_STDOUT, msgbuf, 2
    ret

global _start

_start:
    ; stack init
    lea rax, [rel stack]
    mov [sptr], rax

    call fun_main

    linux_syscall SYSCALL_EXIT, EXIT_OK, 0, 0

