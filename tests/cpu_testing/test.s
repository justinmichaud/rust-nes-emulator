.segment "ZEROPAGE"


;no variables yet


.segment "HEADER"

    .byte $4e,$45,$53,$1a
	.byte 01
	.byte 01
	.byte 00
	.byte 00
	.res 8,0



.segment "STARTUP"

start:
	adc test_var
	jmp start

nmi:
irq:
    rti

.segment "RODATA"

test_var: .word $BEEF

.segment "VECTORS"

    .word nmi	;$fffa vblank nmi
    .word start	;$fffc reset
   	.word irq	;$fffe irq / brk


.segment "CHARS"

	.incbin "Alpha.chr"
