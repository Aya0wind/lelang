	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
; %bb.0:                                ; %common.ret
	cmp	w1, #2000
	ccmp	w0, #0, #0, eq
	mov	w8, #1111
	add	w9, w0, w1
	madd	w8, w1, w8, w9
	add	w8, w8, #2
	csel	w0, w0, w8, eq
	ret
                                        ; -- End function
.subsections_via_symbols
