global start

section 32
bits 32
start:
  mov esp, stack_top ; Update the stack pointer
  
  mov al, "1"
  jmp error

  hlt


; Print a nice red on white ERROR: and it's argument
; Uses argument from al
error:
  ; 4f == RED on white
  mov dword [0xb8000], 0x4f524f45 ; 0x45 = E, 0x52 = R
  mov dword [0xb8004], 0x4f4f4f52 ; 0x52 = R, 0x4f = O
  mov dword [0xb8008], 0x4f3a4f52 ; 0x52 = R, 0x3a = :
  mov word  [0xb800c], 0x4f20     ; 0x20 = ' '
  mov byte  [0xb800e], al         ; Put parameter byte
  mov byte  [0xb800f], 0x4f
  hlt

section .bss
stack_bottom: ; The stack grows downwardsh, so the bottom is actually at the top
  resb 64
stack_top:

