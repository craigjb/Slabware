.section .text
.globl _start
_start:
    li x5, 0x1
    li x28, 0x80
    li x6, 0xf0000000
    
outer:
    li x7, 0x80000
loop0:
    sub x7, x7, x5
    bne x7, x0, loop0
    sw x28, 0(x6)
    srli x28, x28, 1
    beq x28, x0, reset
    j outer
reset:
    li x28, 0x80
    j outer
