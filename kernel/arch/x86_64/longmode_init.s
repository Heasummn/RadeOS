global longmode_start

section .text
bits 64

longmode_start:
  mov rax, 0x2f472f4e2f4f2f4c
  mov qword [0xb8000], rax
  mov rax, 0x2f452f442f4f2f4d
  mov qword [0xb8008], rax
  mov word 	[0xb8010], 0x2f21

  hlt