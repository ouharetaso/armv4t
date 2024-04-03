.section .text

.global _start

_start:
    mov     r0, #0x0000d4 
    orr     r0, r0, #0x00a100
    orr     r0, r0, #0x010000
    mov     r1, #0x0000ac
    orr     r1, r1, #0x007000
    orr     r1, r1, #0x010000

    cmp     r0, r1              // pc = 24 (32)
    subgt   r0, r0, r1          // pc = 28 (36)
    sublt   r1, r1, r0          // pc = 32 (40)
    subne   r15, r15, #20       // pc = 36 (44)


    sub     r15, r15, #8        // pc = 40 (48)
    mov     r1, r1              // pc = 44 (52)
    mov     r2, r2              // pc = 48 (56)
    nop                         // pc = 52 (60)
    nop                         // pc = 56
    nop                         
    nop                         
    swi     #1919


