extern rust_main
global longmode_start
section .text
bits 64

longmode_start:
  call rust_main

  hlt
