	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0	sdk_version 12, 3
	.globl	_m                              ; -- Begin function m
	.p2align	2
_m:                                     ; @m
	.cfi_startproc
; %bb.0:
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	str	w0, [sp, #8]
	str	w1, [sp, #4]
	ldr	w8, [sp, #8]
	add	w8, w8, #100
	str	w8, [sp]
	ldr	w8, [sp]
	cmp	w8, #100
	b.ne	LBB0_4
; %bb.1:
	mov	w8, #10
	str	w8, [sp, #8]
	ldr	w8, [sp, #4]
	cmp	w8, #2000
	b.ne	LBB0_3
; %bb.2:
	mov	w8, #100
	str	w8, [sp]
	ldr	w8, [sp, #8]
	str	w8, [sp, #12]
	b	LBB0_6
LBB0_3:
	b	LBB0_5
LBB0_4:
	mov	w8, #20
	str	w8, [sp, #8]
LBB0_5:
	ldr	w8, [sp, #8]
	ldr	w9, [sp, #4]
	add	w8, w8, w9
	ldr	w9, [sp, #4]
	mov	w10, #1111
	orr	w11, wzr, #0x2
	madd	w9, w9, w10, w11
	add	w8, w8, w9
	str	w8, [sp, #12]
LBB0_6:
	ldr	w0, [sp, #12]
	add	sp, sp, #16
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
