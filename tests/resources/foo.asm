; z80dasm 1.1.6
; command line: z80dasm -l -g 0x100 -o foo.asm zexdoc.com

	org	00100h

l0100h:
	jp l0113h
l0103h:
	nop	
l0104h:
	nop	
l0105h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l0111h:
	nop	
	nop	
l0113h:
	ld hl,(00006h)
	ld sp,hl	
	ld de,l1ddah
	ld c,009h
	call sub_1dceh
	ld hl,l013ah
l0122h:
	ld a,(hl)	
	inc hl	
	or (hl)	
	jp z,l012fh
	dec hl	
	call sub_1ae2h
	jp l0122h
l012fh:
	ld de,l1df6h
	ld c,009h
	call sub_1dceh
	jp 00000h
l013ah:
	jp nz,l2201h
	ld (bc),a	
	add a,d	
	ld (bc),a	
	jp po,04202h
	inc bc	
	and d	
	inc bc	
	ld (bc),a	
	inc b	
	ld h,d	
	inc b	
	jp nz,l2204h
l014dh:
	dec b	
	add a,d	
	dec b	
	jp po,04205h
	ld b,0a2h
	ld b,002h
l0157h:
	rlca	
	ld h,d	
	rlca	
	jp nz,l2207h
	ex af,af'	
	add a,d	
	ex af,af'	
	jp po,04208h
	add hl,bc	
	and d	
	add hl,bc	
	ld (bc),a	
	ld a,(bc)	
	ld h,d	
	ld a,(bc)	
	jp nz,l220ah
	dec bc	
	add a,d	
	dec bc	
	jp po,0420bh
	inc c	
	and d	
	inc c	
	ld (bc),a	
	dec c	
	ld h,d	
	dec c	
	jp nz,0220dh
	ld c,082h
	ld c,0e2h
	ld c,042h
	rrca	
	and d	
	rrca	
	ld (bc),a	
	djnz l01ebh
	djnz l014dh
	djnz l01afh
	ld de,l1182h
	jp po,04211h
	ld (de),a	
	and d	
	ld (de),a	
	ld (bc),a	
	inc de	
	ld h,d	
	inc de	
	jp nz,l2213h
	inc d	
	add a,d	
	inc d	
	jp po,04214h
	dec d	
	and d	
	dec d	
	ld (bc),a	
	ld d,062h
	ld d,0c2h
	ld d,022h
	rla	
	add a,d	
l01afh:
	rla	
	jp po,04217h
	jr l0157h
	jr l01b9h
	add hl,de	
	ld h,d	
l01b9h:
	add hl,de	
	jp nz,02219h
	ld a,(de)	
	add a,d	
	ld a,(de)	
	nop	
	nop	
	rst 0	
	sbc hl,bc
	nop	
	nop	
	inc l	
	add a,e	
	adc a,b	
	ld c,a	
	dec hl	
	jp p,0b339h
	rra	
	ld a,(hl)	
	ld h,e	
	dec d	
	out (089h),a
	ld e,(hl)	
	ld b,(hl)	
	nop	
	jr c,l01dah
l01dah:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld hl,000f8h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l01ebh:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 10h	
	nop	
	rst 38h	
	rst 38h	
	ret m	
	or h	
	jp pe,03ca9h
	ld h,c	
	ld h,h	
	ld h,e	
	inc l	
	ld (hl),e	
	ld h,d	
	ld h,e	
	ld a,020h
	ld l,b	
	ld l,h	
	inc l	
	inc a	
	ld h,d	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	inc l	
	ld l,b	
	ld l,h	
	inc l	
	ld (hl),e	
	ld (hl),b	
	ld a,02eh
	ld l,02eh
	ld l,024h
	rst 0	
	add hl,bc	
	nop	
	nop	
	nop	
	and l	
	call nz,0c4c7h
	ld h,0d2h
	ld d,b	
	and b	
	jp pe,06658h
	add a,l	
	add a,0deh
	ret	
	sbc a,e	
	jr nc,l0239h
l0239h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld hl,000f8h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
sub_0253h:
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 10h	
	nop	
	rst 38h	
	rst 38h	
	adc a,c	
	or (iy+035h)
	ld h,c	
	ld h,h	
	ld h,h	
	jr nz,l02d0h
	ld l,h	
	inc l	
	inc a	
	ld h,d	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	inc l	
	ld l,b	
	ld l,h	
	inc l	
	ld (hl),e	
	ld (hl),b	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 0	
	add ix,bc
	nop	
	nop	
	xor h	
	defb 0ddh,094h	;sub ixh
	jp nz,0635bh
l028dh:
	out (033h),a
	halt	
	ld l,d	
	jr nz,l028dh
	sub h	
	ld l,b	
	push af	
	ld (hl),000h
	jr nc,l029ah
l029ah:
	nop	
	nop	
	nop	
	nop	
	nop	
	ld hl,000f8h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 10h	
	nop	
	rst 38h	
	rst 38h	
	pop bc	
	inc sp	
	ld a,c	
	dec bc	
	ld h,c	
	ld h,h	
	ld h,h	
	jr nz,l0331h
	ld a,b	
	inc l	
	inc a	
	ld h,d	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
l02d0h:
	inc l	
	ld l,c	
	ld a,b	
	inc l	
	ld (hl),e	
	ld (hl),b	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 0	
	add iy,bc
	nop	
	nop	
	jp nz,l07c7h
	call p,051c1h
	sub (hl)	
	ld a,0f4h
	dec bc	
	rrca	
	ld d,c	
	sub d	
	ld e,0eah
	ld (hl),c	
	nop	
	jr nc,l02fah
l02fah:
	nop	
	nop	
	nop	
	ld hl,000f8h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 10h	
	nop	
	rst 38h	
	rst 38h	
	ret pe	
l0320h:
	add a,c	
	ld a,e	
	sbc a,(hl)	
	ld h,c	
	ld h,h	
	ld h,h	
	jr nz,l0391h
	ld a,c	
	inc l	
	inc a	
	ld h,d	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	inc l	
l0331h:
	ld l,c	
	ld a,c	
	inc l	
	ld (hl),e	
	ld (hl),b	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	add a,000h
	nop	
	nop	
	ld b,b	
	sub c	
	inc a	
	ld a,(hl)	
	ld h,a	
	ld a,d	
	ld l,l	
	rst 18h	
	ld h,c	
	ld e,e	
	add hl,hl	
	dec bc	
	djnz l03bbh
	or d	
	add a,l	
	jr c,l0359h
l0359h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
sub_037ah:
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ld c,b	
	ld a,c	
	sub e	
	ld h,b	
	ld h,c	
	ld l,h	
	ld (hl),l	
	ld l,a	
	ld (hl),b	
	jr nz,l03ebh
	inc l	
	ld l,(hl)	
	ld l,(hl)	
	ld l,02eh
	ld l,02eh
l0391h:
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	add a,b	
	nop	
	nop	
	nop	
	ld a,0c5h
	ld a,(04d57h)
	ld c,h	
	inc bc	
	ld bc,0e309h
	ld h,(hl)	
	and (hl)	
	ret nc	
	dec sp	
	cp e	
	xor l	
	ccf	
	nop	
	nop	
	nop	
l03bbh:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 10h	
	nop	
	nop	
	nop	
	cp 043h
	or b	
	ld d,061h
	ld l,h	
	ld (hl),l	
	ld l,a	
	ld (hl),b	
	jr nz,l044bh
	inc l	
l03ebh:
	inc a	
	ld h,d	
l03edh:
	inc l	
	ld h,e	
	inc l	
	ld h,h	
	inc l	
	ld h,l	
	inc l	
	ld l,b	
	inc l	
	ld l,h	
	inc l	
	jr z,l0462h
	ld l,h	
	add hl,hl	
	inc l	
	ld h,c	
	ld a,02eh
l0400h:
	ld l,024h
	rst 10h	
	defb 0ddh,084h	;add a,ixh
	nop	
	nop	
	rst 30h	
	sub 06eh
	rst 0	
	rst 8	
	xor h	
	ld b,a	
	jr z,l03edh
	ld (0c035h),hl
	push bc	
	jr c,l0461h
	inc hl	
	jr nz,l0452h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 10h	
	nop	
	nop	
	nop	
	and h	
	ld (bc),a	
	ld l,l	
	ld e,d	
	ld h,c	
	ld l,h	
	ld (hl),l	
	ld l,a	
	ld (hl),b	
	jr nz,l04abh
	inc l	
l044bh:
	inc a	
	ld l,c	
	ld a,b	
	ld l,b	
	inc l	
	ld l,c	
	ld a,b	
l0452h:
	ld l,h	
	inc l	
	ld l,c	
	ld a,c	
	ld l,b	
	inc l	
	ld l,c	
	ld a,c	
	ld l,h	
	ld a,02eh
	ld l,02eh
	ld l,02eh
l0461h:
	inc h	
l0462h:
	rst 10h	
	add a,(ix+001h)
	nop	
	or a	
	sub b	
	ld (bc),a	
	ld bc,l0100h+2
	defb 0fdh,032h,06eh	;illegal sequence
	ld b,b	
	call c,045c1h
	ld l,(hl)	
	jp m,l20e5h
	jr c,l047ah
l047ah:
	nop	
	nop	
	nop	
	ld bc,l0100h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ret pe	
	ld c,c	
	ld h,a	
	ld l,(hl)	
	ld h,c	
	ld l,h	
	ld (hl),l	
	ld l,a	
	ld (hl),b	
	jr nz,l050bh
	inc l	
l04abh:
	jr z,l04e9h
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02e29h
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	ld d,e	
	bit 0,(ix+001h)
	ld (hl),l	
	jr nz,$+4
	ld bc,l0100h+2
	call m,09a3ch
	and a	
	ld (hl),h	
	dec a	
	ld d,c	
	daa	
	inc d	
	jp z,00020h
	nop	
	jr c,l04dch
l04dch:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld d,e	
	nop	
l04e9h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	xor b	
	xor 008h
	ld h,a	
	ld h,d	
	ld l,c	
	ld (hl),h	
	jr nz,l0576h
	inc l	
	jr z,l0547h
l050bh:
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02e29h
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	ld d,e	
	bit 0,b
	nop	
	nop	
	pop af	
	ld a,0fch
	sbc a,l	
	call z,sub_037ah
	ld bc,0be61h
	add a,(hl)	
	ld a,d	
	ld d,b	
	inc h	
	sbc a,b	
	add hl,de	
	nop	
	ccf	
l0539h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l0547h:
	ld d,e	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	nop	
	rst 38h	
	nop	
	nop	
	ld a,e	
	ld d,l	
	and 0c8h
	ld h,d	
	ld l,c	
	ld (hl),h	
	jr nz,l05d6h
	inc l	
	inc a	
	ld h,d	
	inc l	
	ld h,e	
	inc l	
	ld h,h	
	inc l	
	ld h,l	
	inc l	
	ld l,b	
	inc l	
	ld l,h	
	inc l	
l0576h:
	jr z,l05e0h
	ld l,h	
	add hl,hl	
	inc l	
	ld h,c	
	ld a,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	cpd
	nop	
	nop	
	or (hl)	
	rst 0	
	or h	
	ld (hl),d	
	or 018h
	inc d	
	ld bc,08dbdh
	ld bc,0c000h
	jr nc,l0539h
	sub h	
	nop	
	djnz l059ah
l059ah:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld a,(bc)	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	xor b	
	ld a,(hl)	
	ld l,h	
	jp m,07063h
	ld h,h	
	inc a	
	ld (hl),d	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
l05d6h:
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
l05e0h:
	ld l,024h
	rst 10h	
	cpi
	nop	
	nop	
	ld c,b	
	ld c,l	
	ld c,d	
	xor a	
	ld l,e	
	sub b	
	inc bc	
	ld bc,04e71h
	ld bc,09300h
	ld l,d	
	ld a,h	
	sub b	
	nop	
	djnz l05fah
l05fah:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld a,(bc)	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
sub_0611h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ld b,0deh
	or e	
	ld d,(hl)	
	ld h,e	
	ld (hl),b	
	ld l,c	
	inc a	
	ld (hl),d	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	daa	
	nop	
	nop	
	nop	
	ld b,c	
	ld hl,009fah
	ld h,b	
	dec e	
	ld e,c	
	and l	
	ld e,e	
	adc a,l	
	ld a,c	
	sub b	
	inc b	
	adc a,(hl)	
	sbc a,l	
	add hl,hl	
	jr l0659h
l0659h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	sbc a,e	
	ld c,e	
	and (hl)	
	ld (hl),l	
	inc a	
	ld h,h	
	ld h,c	
	ld h,c	
l0687h:
	inc l	
	ld h,e	
	ld (hl),b	
	ld l,h	
	inc l	
	ld (hl),e	
	ld h,e	
	ld h,(hl)	
	inc l	
	ld h,e	
	ld h,e	
	ld h,(hl)	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	inc a	
	nop	
	nop	
	nop	
	rst 18h	
	ld c,d	
	ret c	
	push de	
	sbc a,b	
	push hl	
	dec hl	
	adc a,d	
	or b	
	and a	
	dec de	
	ld b,e	
	ld b,h	
	ld e,d	
	jr nc,l0687h
	ld bc,00000h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
sub_06d7h:
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	pop de	
	adc a,b	
	dec d	
	and h	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld h,c	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	inc b	
	nop	
	nop	
	nop	
	inc hl	
	sub 02dh
	ld b,e	
	ld h,c	
	ld a,d	
	add a,b	
	add a,c	
	add a,(hl)	
	ld e,d	
	add a,l	
	ld e,086h
	ld e,b	
	cp e	
	sbc a,e	
	ld bc,00000h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ld e,a	
	ld l,b	
	ld (03c64h),hl
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld h,d	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	inc bc	
	nop	
	nop	
	nop	
	sub a	
	call 044abh
	ret	
	adc a,l	
	ex (sp),hl	
	ex (sp),hl	
	call z,0a411h
	ret pe	
	ld (bc),a	
	ld c,c	
	ld c,l	
	ld hl,(00008h)
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld hl,000f8h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	jp nc,03baeh
	call pe,0693ch
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld h,d	
	ld h,e	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	inc c	
	nop	
	nop	
	nop	
l07c7h:
	adc a,c	
	rst 10h	
	dec (hl)	
	add hl,bc	
	ld e,e	
	dec b	
	add a,l	
	sbc a,a	
	daa	
	adc a,e	
	ex af,af'	
	jp nc,00595h
	ld h,b	
	ld b,001h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	jp nz,05584h
	ld c,h	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld h,e	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	inc d	
	nop	
	nop	
	nop	
	jp pe,0baa0h
	ld e,a	
	ei	
	ld h,l	
	inc e	
	sbc a,b	
	call z,0bc38h
	sbc a,043h
	ld e,h	
	cp l	
	inc bc	
	ld bc,00000h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l0858h:
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ld b,l	
	inc hl	
	sbc a,010h
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld h,h	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	inc de	
	nop	
	nop	
	nop	
	ld l,034h
	dec e	
	inc de	
	ret	
	jr z,l0858h
	ld a,(bc)	
	ld h,a	
	sbc a,c	
	ld l,03ah
	sub d	
	or 054h
	sbc a,l	
	ex af,af'	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld hl,000f8h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	xor (hl)	
	add a,0d4h
	inc l	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld h,h	
	ld h,l	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	inc e	
	nop	
	nop	
	nop	
	cpl	
	ld h,b	
	dec c	
	ld c,h	
	ld (bc),a	
	inc h	
	push af	
	jp po,0a0f4h
	ld a,(bc)	
	and c	
	inc de	
	ld (05925h),a
	ld bc,00000h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	pop hl	
	ld (hl),l	
	xor a	
	call z,0693ch
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld h,l	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	inc h	
	nop	
	nop	
	nop	
	ld b,015h
	ex de,hl	
	jp p,0e8ddh
	dec hl	
	ld h,0a6h
	ld de,0bc1ah
	rla	
	ld b,018h
	jr z,l0959h
	nop	
l0959h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	inc e	
	defb 0edh;next byte illegal after ed
	add a,h	
	ld a,l	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld l,b	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	inc hl	
	nop	
	nop	
	nop	
	call p,0a5c3h
	rlca	
	ld l,l	
	dec de	
	inc b	
	ld c,a	
	jp nz,02ae2h
	add a,d	
	ld d,a	
	ret po	
	pop hl	
	jp 00008h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld hl,000f8h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	call m,06d0dh
	ld c,d	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld l,b	
	ld l,h	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	inc ix
	nop	
	nop	
	inc a	
	cp h	
	sbc a,e	
	dec c	
	add a,c	
	ret po	
	defb 0fdh,0adh	;xor iyl
	ld a,a	
	sbc a,d	
	push hl	
	sub (hl)	
	inc de	
	add a,l	
	jp po,0000bh
	ex af,af'	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld hl,000f8h
	nop	
	nop	
l0a24h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	and l	
	ld c,l	
	cp (hl)	
	ld sp,0693ch
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld l,c	
	ld a,b	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	inc iy
	nop	
	nop	
	ld (bc),a	
	sub h	
	ld a,d	
	ld h,e	
	add a,d	
	ld sp,0c65ah
	jp (hl)	
	or d	
	or h	
	xor e	
	ld d,0f2h
	dec b	
	ld l,l	
	nop	
	ex af,af'	
	nop	
	nop	
	nop	
	nop	
	ld hl,000f8h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ld d,b	
	ld e,l	
	ld d,c	
	and e	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld l,c	
	ld a,c	
l0aafh:
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	inc l	
	nop	
	nop	
	nop	
	ld sp,l2080h
	and l	
	ld d,(hl)	
	ld b,e	
	add hl,bc	
	or h	
	pop bc	
	call p,0dfa2h
	pop de	
	inc a	
	and d	
	ld a,001h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ld d,(hl)	
	call 0f306h
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld l,h	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	inc (hl)	
	nop	
	nop	
	nop	
	ld d,(hl)	
	cp b	
	ld a,h	
	inc c	
	ld a,0e5h
	inc bc	
	ld bc,0877eh
	ld e,b	
	jp c,05c15h
	scf	
	rra	
	ld bc,00000h
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	cp b	
	ld a,(0efdch)
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	jr z,l0bd7h
	ld l,h	
	add hl,hl	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	inc sp	
	nop	
	nop	
	nop	
	ld l,a	
	inc (hl)	
	add a,d	
	call nc,0d169h
	or (hl)	
	sbc a,094h
	and h	
	halt	
	call p,sub_0253h
	ld e,e	
	add a,l	
	ex af,af'	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld hl,000f8h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ld e,l	
	xor h	
	push de	
	daa	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld (hl),e	
	ld (hl),b	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
l0bd7h:
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	inc (ix+001h)
	nop	
	ld l,(hl)	
	jp m,l0100h+2
	ld (bc),a	
	ld bc,02c28h
	sub h	
	adc a,b	
	ld d,a	
	ld d,b	
	ld d,033h
	ld l,a	
	jr z,l0c18h
	ld bc,00000h
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
l0c01h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l0c18h:
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	jr nz,l0c79h
	inc d	
	ld (hl),b	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	jr z,l0c6bh
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02e29h
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	defb 0ddh,024h	;inc ixh
	nop	
	nop	
	jr c,l0c01h
	ld l,h	
	ld sp,0c6d4h
	ld bc,0583eh
	add a,e	
	or h	
	dec d	
l0c53h:
	add a,c	
	sbc a,059h
	ld b,d	
	nop	
	ld bc,00000h
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l0c6bh:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l0c79h:
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ld l,a	
	ld b,(hl)	
	ld (hl),062h
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld l,c	
	ld a,b	
	ld l,b	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	defb 0ddh,02ch	;inc ixl
	nop	
	nop	
	inc d	
	ld c,l	
	ld h,b	
	ld (hl),h	
	call nc,0e776h
	ld b,0a2h
	ld (l213ch),a
	sub 0d7h
	and l	
	sbc a,c	
	nop	
	ld bc,00000h
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ld (bc),a	
	ld a,e	
	rst 28h	
	inc l	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld l,c	
	ld a,b	
	ld l,h	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	defb 0ddh,024h	;inc ixh
	nop	
	nop	
	ld (hl),028h
	ld l,a	
	sbc a,a	
	ld d,091h
	cp c	
	ld h,c	
	res 0,d
	add hl,de	
	jp po,07392h
	adc a,h	
	xor c	
	nop	
	ld bc,00000h
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	dec l	
	sub (hl)	
	ld l,h	
	di	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld l,c	
	ld a,c	
	ld l,b	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	defb 0ddh,02ch	;inc ixl
	nop	
	nop	
	add a,0d7h
	push de	
	ld h,d	
	sbc a,(hl)	
	and b	
	add hl,sp	
	ld (hl),b	
	ld a,(hl)	
	ld a,012h
	sbc a,a	
	sub b	
	exx	
	rrca	
	ld (l0100h),hl
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ei	
	res 7,d
	sub l	
	inc a	
	ld l,c	
	ld l,(hl)	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld h,e	
	ld a,020h
	ld l,c	
	ld a,c	
	ld l,h	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld bc,(l0103h)
	xor b	
	ld sp,hl	
	ld e,c	
	push af	
	and h	
	sub e	
	defb 0edh;next byte illegal after ed
	push af	
	sub (hl)	
	ld l,a	
	ld l,b	
	exx	
	add a,(hl)	
	and 0d8h
sub_0dd6h:
	ld c,e	
	nop	
	djnz l0ddah
l0ddah:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld c,l	
	ld b,l	
	xor c	
	xor h	
	ld l,h	
	ld h,h	
	jr nz,l0e43h
	ld h,d	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld a,02ch
	jr z,$+112
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	add hl,hl	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld hl,(l0103h)
	nop	
	ld h,e	
	sbc a,b	
	jr nc,l0ea3h
	ld (hl),a	
l0e2ch:
	jr nz,l0e2ch
	or c	
	jp m,0b8b9h
	xor e	
	inc b	
	ld b,015h
	ld h,b	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l0e43h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld e,a	
	sub a	
	inc h	
	add a,a	
	ld l,h	
	ld h,h	
	jr nz,l0ecfh
	ld l,h	
	inc l	
	jr z,l0ed9h
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	add hl,hl	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ld sp,(l0103h)
	call m,0d78dh
	ld d,a	
	ld h,c	
	ld hl,0ca18h
	add a,l	
	pop bc	
	jp c,08327h
	ld e,060h
	call p,00000h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l0ea3h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld a,d	
	adc a,0a1h
	dec de	
	ld l,h	
	ld h,h	
	jr nz,l0f3ah
	ld (hl),b	
	inc l	
	jr z,$+112
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	add hl,hl	
l0ecfh:
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
l0ed9h:
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ld ix,(l0103h)
	rst 10h	
	sbc a,0fah
	and (hl)	
	add a,b	
	rst 30h	
	ld c,h	
	inc h	
	sbc a,087h
	jp nz,016bch
	ld h,e	
	sub (hl)	
	ld c,h	
	jr nz,l0ef9h
l0ef9h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	add a,l	
	adc a,e	
	pop af	
	ld l,l	
	ld l,h	
	ld h,h	
	jr nz,l0f63h
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02ch
	jr z,$+112
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	add hl,hl	
	ld l,02eh
	ld l,02eh
	ld l,02eh
l0f3ah:
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld (l0103h),bc
	sbc a,b	
	rra	
	ld c,l	
	add a,h	
	xor h	
	ret pe	
	defb 0edh;next byte illegal after ed
	ret	
	ld e,l	
	ret	
	ld h,c	
	adc a,a	
	add a,b	
	ccf	
	cp a	
	rst 0	
	nop	
	djnz l0f5ah
l0f5ah:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l0f63h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	ld h,h	
	ld e,087h
	dec d	
	ld l,h	
	ld h,h	
	jr nz,l0fafh
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	add hl,hl	
	inc l	
	inc a	
	ld h,d	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ld (l0103h),hl
	nop	
	inc bc	
	ret nc	
	ld (hl),d	
	ld (hl),a	
	ld d,e	
	ld a,a	
	ld (hl),d	
	ccf	
l0fafh:
	jp pe,08064h
	pop hl	
	djnz l0fe2h
	jp (hl)	
	dec (hl)	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	and e	
	ld h,b	
	adc a,e	
l0fe2h:
	ld b,a	
	ld l,h	
	ld h,h	
	jr nz,l100fh
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	add hl,hl	
	inc l	
	ld l,b	
	ld l,h	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ld (l0103h),sp
	call c,0d6c0h
	pop de	
	ld e,d	
	im 1
	di	
l100fh:
	jp c,0a7afh
	ld l,h	
	ld b,h	
	sbc a,a	
	ld a,(bc)	
	ccf	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	ld d,058h
	ld e,a	
	rst 10h	
	ld l,h	
	ld h,h	
	jr nz,l106fh
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	add hl,hl	
	inc l	
	ld (hl),e	
	ld (hl),b	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ld (l0103h),ix
	jp 0916ch
	dec c	
	nop	
	ld l,c	
	ret m	
	adc a,(hl)	
l106fh:
	sub 0e3h
	rst 30h	
	jp 0d9c6h
	rst 18h	
	jp nz,00020h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	cp d	
	djnz l10cch
	ld l,e	
	ld l,h	
	ld h,h	
	jr nz,l10cfh
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	add hl,hl	
	inc l	
	inc a	
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ld bc,00000h
	nop	
	inc e	
	ld e,h	
	ld b,(hl)	
	dec l	
	cp c	
l10cch:
	adc a,(hl)	
	ld a,b	
	ld h,b	
l10cfh:
	or c	
	ld (hl),h	
	ld c,0b3h
	ld b,(hl)	
	pop de	
	call z,03030h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	sbc a,039h
	add hl,de	
	ld l,c	
	ld l,h	
	ld h,h	
	jr nz,l1143h
	ld h,d	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	inc l	
	ld l,b	
	ld l,h	
	inc l	
	ld (hl),e	
	ld (hl),b	
	ld a,02ch
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld ix,00000h
	ret pe	
	add a,a	
	ld b,020h
	ld (de),a	
	cp l	
	sbc a,e	
	or (hl)	
	ld d,e	
	ld (hl),d	
	push hl	
	and c	
	ld d,c	
	inc de	
	cp l	
	pop af	
	jr nz,l1139h
l1139h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l1143h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld (0d57dh),hl
	dec h	
	ld l,h	
	ld h,h	
	jr nz,l11a3h
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02ch
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
l1182h:
	rst 10h	
	ld a,(bc)	
	nop	
	nop	
	nop	
	xor b	
	or e	
	ld hl,(08e1dh)
	ld a,a	
	xor h	
	ld b,d	
	inc bc	
	ld bc,l0103h
	add a,0b1h
	adc a,(hl)	
	rst 28h	
	djnz l1199h
l1199h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l11a3h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	rst 38h	
	nop	
	nop	
	or b	
	add a,c	
	adc a,c	
	dec (hl)	
	ld l,h	
	ld h,h	
	jr nz,l1228h
	inc l	
	inc a	
	jr z,l122dh
	ld h,e	
	add hl,hl	
	inc l	
	jr z,$+102
	ld h,l	
	add hl,hl	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld b,000h
	nop	
	nop	
	rlca	
	call nz,0f49dh
	dec a	
	pop de	
	add hl,sp	
	inc bc	
	adc a,c	
	sbc a,055h
	ld (hl),h	
	ld d,e	
	ret nz	
	add hl,bc	
	ld d,l	
	jr c,l11f9h
l11f9h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	pop af	
	jp c,056b5h
	ld l,h	
	ld h,h	
	jr nz,l1263h
	ld h,d	
l1228h:
	inc l	
	ld h,e	
	inc l	
	ld h,h	
	inc l	
l122dh:
	ld h,l	
	inc l	
	ld l,b	
	inc l	
	ld l,h	
	inc l	
	jr z,$+106
	ld l,h	
	add hl,hl	
	inc l	
	ld h,c	
	ld a,02ch
	ld l,(hl)	
	ld l,(hl)	
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ld (ix+001h),000h
	ld b,l	
	dec de	
	ld (bc),a	
	ld bc,l0100h+2
	pop bc	
	push de	
	rst 0	
	ld h,c	
	call nz,0c0bdh
	add a,l	
	ld d,0cdh
	jr nz,l1259h
l1259h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l1263h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	ld h,0dbh
	ld b,a	
	ld a,(hl)	
	ld l,h	
	ld h,h	
	jr nz,l12afh
	inc a	
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02c29h
	ld l,(hl)	
	ld l,(hl)	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld b,(ix+001h)
	nop	
	ld d,0d0h
	ld (bc),a	
	ld bc,l0100h+2
	ld h,b	
	ld b,d	
l12afh:
	add hl,sp	
	ld a,a	
	inc b	
	inc b	
	sub a	
	ld c,d	
	add a,l	
	ret nc	
	jr nz,l12d1h
	nop	
	nop	
	nop	
	nop	
	ld bc,l0100h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
l12d1h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	call z,sub_0611h
	xor b	
	ld l,h	
	ld h,h	
	jr nz,l1323h
	ld h,d	
	inc l	
	ld h,e	
	inc l	
	ld h,h	
	inc l	
	ld h,l	
	ld a,02ch
	jr z,l132eh
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02e29h
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld h,(ix+001h)
	nop	
	ret po	
	add a,h	
	ld (bc),a	
	ld bc,l0100h+2
	ld d,d	
	sbc a,h	
	sbc a,c	
	and a	
	or (hl)	
	ld c,c	
	sub e	
	nop	
	xor l	
	xor 020h
	ex af,af'	
	nop	
	nop	
	nop	
	nop	
	ld bc,l0100h
	nop	
	nop	
	nop	
l1323h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l132eh:
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	jp m,04d2ah
	inc bc	
	ld l,h	
	ld h,h	
	jr nz,l1383h
	ld l,b	
	inc l	
	ld l,h	
	ld a,02ch
	jr z,l138ah
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02e29h
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld a,(ix+001h)
	nop	
	or (hl)	
	ret c	
	ld (bc),a	
	ld bc,l0100h+2
	ld (de),a	
	add a,007h
	rst 18h	
	ret nc	
	sbc a,h	
	ld b,e	
	and (hl)	
	push hl	
	and b	
	jr nz,l1379h
l1379h:
	nop	
	nop	
	nop	
	nop	
	ld bc,l0100h
	nop	
	nop	
	nop	
l1383h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l138ah:
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
l1395h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	and l	
	jp (hl)	
	xor h	
	ld h,h	
	ld l,h	
	ld h,h	
	jr nz,l1408h
	inc l	
	jr z,l13e6h
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02e29h
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	defb 0ddh,026h,000h	;ld ixh,000h
	nop	
	ld d,e	
	inc a	
	ld b,b	
	ld b,(hl)	
	ld a,c	
	pop hl	
	ld de,00777h
	pop bc	
	jp m,0811ah
	xor l	
	sbc a,e	
	ld e,l	
	jr nz,l13e1h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l13e1h:
	nop	
	nop	
	nop	
	nop	
	nop	
l13e6h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	inc h	
	ret pe	
	add a,d	
	adc a,e	
	ld l,h	
	ld h,h	
	jr nz,l1443h
	ld l,c	
l1408h:
	ld a,b	
	ld l,b	
	inc l	
	ld l,c	
	ld a,b	
	ld l,h	
	inc l	
	ld l,c	
	ld a,c	
	ld l,b	
	inc l	
	ld l,c	
	ld a,c	
	ld l,h	
	ld a,02ch
	ld l,(hl)	
	ld l,(hl)	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld b,b	
	nop	
	nop	
	nop	
	and h	
	ld (hl),d	
	inc h	
	and b	
	xor h	
	ld h,c	
	inc bc	
	ld bc,082c7h
	adc a,a	
	ld (hl),c	
	sub a	
	adc a,a	
	adc a,(hl)	
	rst 28h	
	ccf	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l1443h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 10h	
	rst 38h	
	nop	
	nop	
	ld (hl),h	
	ld c,e	
	ld bc,06c18h
	ld h,h	
	jr nz,l14a3h
	ld h,d	
	ld h,e	
	ld h,h	
	ld h,l	
	ld l,b	
	ld l,h	
	ld h,c	
	ld a,02ch
	inc a	
	ld h,d	
	ld h,e	
	ld h,h	
	ld h,l	
	ld l,b	
	ld l,h	
	ld h,c	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	defb 0ddh,040h,000h	;illegal sequence
	nop	
	push bc	
	cp h	
	inc bc	
	ld bc,l0103h
	inc bc	
	ld bc,02fc2h
	ret nz	
	sbc a,b	
	add a,e	
	rra	
	call sub_203bh
	ccf	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l14a3h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 10h	
	rst 38h	
	nop	
	nop	
	ld b,a	
	adc a,e	
	and e	
	ld l,e	
	ld l,h	
	ld h,h	
	jr nz,l1503h
	ld h,d	
	ld h,e	
	ld h,h	
	ld h,l	
	ld a,b	
	ld a,c	
	ld h,c	
	ld a,02ch
	inc a	
	ld h,d	
	ld h,e	
	ld h,h	
	ld h,l	
	ld a,b	
	ld a,c	
	ld h,c	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld (l0103h),a
	nop	
	ld l,b	
	defb 0fdh,0ech,0f4h	;illegal sequence
	and b	
	ld b,h	
	ld b,e	
	or l	
	ld d,e	
	ld b,0bah
	call 04fd2h
	ret c	
	rra	
	ex af,af'	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l1503h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	rst 38h	
	nop	
	nop	
	ret	
	ld h,02dh
	push hl	
	ld l,h	
	ld h,h	
	jr nz,l1588h
	inc l	
	jr z,$+112
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	add hl,hl	
	jr nz,l155fh
	jr nz,$+110
	ld h,h	
	jr nz,l155dh
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	ld l,(hl)	
	add hl,hl	
	inc l	
	ld h,c	
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ldd
	nop	
	nop	
	ld d,d	
	sbc a,b	
	jp m,0a168h
	ld h,(hl)	
	ld b,001h
	inc b	
	ld bc,00001h
	pop bc	
	ld l,b	
	or a	
	jr nz,l1558h
l1558h:
	djnz l155ah
l155ah:
	nop	
	nop	
	nop	
l155dh:
	nop	
	nop	
l155fh:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	sub h	
	call p,06927h
	ld l,h	
	ld h,h	
	ld h,h	
	inc a	
	ld (hl),d	
l1588h:
	ld a,020h
	jr z,l15bdh
	add hl,hl	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ldd
	nop	
	nop	
	ld l,0f1h
	ld hl,(0baebh)
	push de	
	ld b,001h
	inc b	
	ld bc,00002h
	ld b,a	
	rst 38h	
	call po,000fbh
	djnz l15bah
l15bah:
	nop	
	nop	
	nop	
l15bdh:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ld e,d	
	sub b	
	ld a,(hl)	
	call nc,0646ch
	ld h,h	
	inc a	
	ld (hl),d	
	ld a,020h
	jr z,l161eh
	add hl,hl	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
l1602h:
	rst 10h	
	ldi
	nop	
	nop	
l1607h:
	jr nc,l1607h
	call 05803h
	ld h,b	
	dec b	
	ld bc,l0103h
	ld bc,l0400h
	ld h,b	
	adc a,b	
	ld h,000h
	djnz l161ah
l161ah:
	nop	
	nop	
	nop	
	nop	
l161eh:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	sbc a,d	
	cp l	
	or 0b5h
	ld l,h	
	ld h,h	
	ld l,c	
	inc a	
	ld (hl),d	
	ld a,020h
	jr z,l167dh
	add hl,hl	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ldi
	nop	
	nop	
	adc a,04ah
	ld l,(hl)	
	jp nz,0b188h
	dec b	
	ld bc,l0103h
	ld (bc),a	
	nop	
	inc d	
	dec l	
	sbc a,a	
	and e	
	nop	
	djnz l167ah
l167ah:
	nop	
	nop	
	nop	
l167dh:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	ex de,hl	
	ld e,c	
	adc a,c	
	dec de	
	ld l,h	
	ld h,h	
	ld l,c	
	inc a	
	ld (hl),d	
	ld a,020h
	jr z,l16deh
	add hl,hl	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	neg
	nop	
	nop	
	and d	
	jr c,$+109
	ld e,a	
	inc (hl)	
	exx	
	call po,0d657h
	jp nc,04642h
	ld b,e	
	ld e,d	
	call z,00009h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l16deh:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld l,d	
	inc a	
	dec sp	
	cp l	
	ld l,(hl)	
	ld h,l	
	ld h,a	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	rrd
	nop	
	nop	
	res 2,c
	adc a,e	
	call nz,0fa62h
	inc bc	
	ld bc,0e720h
	ld a,c	
	or h	
	ld b,b	
	ld b,0e2h
	adc a,d	
	nop	
	ex af,af'	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	rst 38h	
	nop	
	nop	
	sub l	
	ld e,e	
	and e	
	ld h,03ch
	ld (hl),d	
	ld (hl),d	
	ld h,h	
	inc l	
	ld (hl),d	
	ld l,h	
	ld h,h	
	ld a,02eh
	ld l,02eh
	ld l,02eh
l1771h:
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	rlca	
	nop	
	nop	
	nop	
	sub d	
	bit 0,e
	ld l,l	
	sub b	
	ld a,(bc)	
	add a,h	
	jp nz,l0c53h
	ld c,0f5h
	sub c	
	ex de,hl	
	call m,01840h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	dec h	
	inc de	
	jr nc,l1771h
	inc a	
	ld (hl),d	
	ld l,h	
	ld h,e	
	ld h,c	
	inc l	
	ld (hl),d	
	ld (hl),d	
	ld h,e	
	ld h,c	
	inc l	
	ld (hl),d	
	ld l,h	
	ld h,c	
	inc l	
	ld (hl),d	
	ld (hl),d	
	ld h,c	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	rlc (ix+001h)
	xor a	
	defb 0ddh,002h,001h	;illegal sequence
	ld (bc),a	
	ld bc,0ff3ch
	or 0dbh
	call p,08294h
	add a,b	
	exx	
	ld h,c	
	jr nz,l17f9h
l17f9h:
	nop	
	jr c,l17fch
l17fch:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	add a,b	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld d,a	
	nop	
	nop	
	nop	
	ld (hl),c	
	ld a,(081cdh)
	ld (hl),e	
	ld l,b	
	ld h,(hl)	
	cpl	
	ld (hl),d	
	ld l,a	
	ld (hl),h	
	jr nz,l1854h
	inc a	
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02e29h
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	rlc b
	nop	
	nop	
	ex de,hl	
	call z,05d4ah
	rlca	
	ret po	
	inc bc	
	ld bc,l1395h
	xor 030h
	ld b,e	
l1854h:
	ld a,b	
	xor l	
	dec a	
	nop	
	ccf	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	add a,b	
	nop	
	nop	
	nop	
	nop	
l186ch:
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	ld d,a	
	rst 38h	
	nop	
	nop	
	ex de,hl	
	ld h,b	
	ld c,l	
	ld e,b	
	ld (hl),e	
	ld l,b	
	ld h,(hl)	
	cpl	
	ld (hl),d	
	ld l,a	
	ld (hl),h	
	jr nz,l18c8h
	ld h,d	
	inc l	
	ld h,e	
	inc l	
	ld h,h	
	inc l	
	ld h,l	
	inc l	
	ld l,b	
	inc l	
	ld l,h	
	inc l	
	jr z,l1902h
	ld l,h	
	add hl,hl	
	inc l	
	ld h,c	
	ld a,02eh
	ld l,024h
	rst 10h	
	res 0,b
	nop	
	nop	
	push de	
	inc l	
	xor e	
	sub a	
	rst 38h	
	add hl,sp	
	inc bc	
	ld bc,0d14bh
	or d	
	ld l,d	
	ld d,e	
	daa	
	jr c,l186ch
	nop	
	ld a,a	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l18c8h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 10h	
	rst 38h	
	nop	
	nop	
	adc a,e	
	ld d,a	
	ret p	
	ex af,af'	
	inc a	
	ld (hl),e	
	ld h,l	
	ld (hl),h	
	inc l	
	ld (hl),d	
	ld h,l	
	ld (hl),e	
	ld a,020h
	ld l,(hl)	
	inc l	
	inc a	
	ld h,d	
	ld h,e	
	ld h,h	
	ld h,l	
	ld l,b	
	ld l,h	
	jr z,$+106
	ld l,h	
	add hl,hl	
	ld h,c	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	inc h	
l1902h:
	rst 10h	
	res 0,(ix+001h)
	ld b,h	
	ei	
	ld (bc),a	
	ld bc,l0100h+2
	add hl,bc	
	cp d	
	cp (hl)	
	ld l,b	
	ret c	
	ld (05e10h),a
	ld h,a	
	xor b	
	jr nz,l1919h
l1919h:
	nop	
	ld a,b	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 10h	
	nop	
	nop	
	nop	
	call z,0f963h
	adc a,d	
	inc a	
	ld (hl),e	
	ld h,l	
	ld (hl),h	
	inc l	
	ld (hl),d	
	ld h,l	
	ld (hl),e	
	ld a,020h
	ld l,(hl)	
	inc l	
	jr z,l198dh
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02e29h
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ld (ix+001h),b
	nop	
	dec c	
	daa	
	ld (bc),a	
	ld bc,l0100h+2
	ld a,(07bb7h)
	adc a,b	
	xor 099h
	add a,(hl)	
	ld (hl),b	
	rlca	
	jp z,l0320h
	nop	
	nop	
	nop	
	nop	
	ld bc,l0100h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l198dh:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	inc b	
	ld h,d	
	ld l,d	
	cp a	
	ld l,h	
	ld h,h	
	jr nz,l19cfh
	inc a	
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02c29h
	inc a	
	ld h,d	
	inc l	
	ld h,e	
	inc l	
	ld h,h	
	inc l	
	ld h,l	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld (ix+001h),h
	nop	
	ld h,h	
	or (hl)	
	ld (bc),a	
	ld bc,l0100h+2
	xor h	
	ret pe	
l19cfh:
	push af	
	or l	
	cp 0aah
	ld (de),a	
	djnz l1a3ch
	sub l	
	jr nz,l19dah
	nop	
l19dah:
	nop	
	nop	
	nop	
	ld bc,l0100h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	ld l,d	
	ld a,(de)	
	adc a,b	
	ld sp,0646ch
	jr nz,l1a2fh
	inc a	
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02c29h
	inc a	
	ld l,b	
	inc l	
	ld l,h	
	ld a,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,024h
	rst 10h	
	ld (ix+001h),a
	nop	
	xor a	
	ld h,a	
	ld (bc),a	
	ld bc,l0100h+2
	inc de	
	ld c,a	
l1a2fh:
	ld b,h	
	ld b,0d7h
	cp h	
	ld d,b	
	xor h	
	xor a	
	ld e,a	
	jr nz,l1a39h
l1a39h:
	nop	
	nop	
	nop	
l1a3ch:
	nop	
	ld bc,l0100h
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	call z,05abeh
	sub (hl)	
	ld l,h	
	ld h,h	
	jr nz,l1a8fh
	inc a	
	ld l,c	
	ld a,b	
	inc l	
	ld l,c	
	ld a,c	
	ld a,02bh
	ld sp,02c29h
	ld h,c	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
	rst 10h	
	ld (bc),a	
	nop	
	nop	
	nop	
	dec sp	
	inc c	
	sub d	
	or l	
	rst 38h	
	ld l,h	
	sbc a,(hl)	
	sub l	
l1a8fh:
	inc bc	
	ld bc,l0104h
	pop bc	
	ld hl,0bde7h
	jr l1a99h
l1a99h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	rst 38h	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	rst 38h	
	nop	
	nop	
	ld a,d	
	ld c,h	
	ld de,06c4fh
	ld h,h	
	jr nz,$+42
	inc a	
	ld h,d	
	ld h,e	
	inc l	
	ld h,h	
	ld h,l	
	ld a,029h
	inc l	
	ld h,c	
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	ld l,02eh
	inc h	
sub_1ae2h:
	push hl	
	ld a,(hl)	
	inc hl	
	ld h,(hl)	
	ld l,a	
	ld a,(hl)	
	ld (01d65h),a
	inc hl	
	push hl	
	ld de,00014h
	add hl,de	
	ld de,l1cdah
	call sub_1c49h
	pop hl	
	push hl	
	ld de,00028h
	add hl,de	
	ld de,l1d02h
	call sub_1c49h
	ld hl,l1d02h
	ld (hl),001h
	pop hl	
	push hl	
	ld de,l1d42h
	ld bc,00004h
	ldir
	ld de,l0103h
	ld bc,00010h
	ldir
	ld de,0002ch
	add hl,de	
	ex de,hl	
	ld c,009h
	call sub_1dceh
	call sub_1e71h
l1b27h:
	ld a,(l1d42h)
	cp 076h
	jp z,l1b3eh
	and 0dfh
	cp 0ddh
	jp nz,l1b3bh
	ld a,(l1d43h)
	cp 076h
l1b3bh:
	call nz,sub_1d2ah
l1b3eh:
	call sub_1c89h
	call nz,sub_1cadh
	pop hl	
	jp z,l1b7ah
	ld de,0003ch
	add hl,de	
	call sub_1e32h
	ld de,l1e05h
	jp z,l1b71h
	ld de,l1e0ch
	ld c,009h
	call sub_1dceh
	call sub_1d99h
	ld de,01e27h
	ld c,009h
	call sub_1dceh
	ld hl,l1e85h
	call sub_1d99h
	ld de,01e2fh
l1b71h:
	ld c,009h
	call sub_1dceh
	pop hl	
	inc hl	
	inc hl	
	ret	
l1b7ah:
	push hl	
	ld a,001h
	ld (l1bf0h),a
	ld (l1c14h),a
	ld hl,l1cdah
	ld (l1bf1h),hl
	ld hl,l1d02h
	ld (l1c15h),hl
	ld b,004h
	pop hl	
	push hl	
	ld de,l1d42h
	call sub_1ba4h
	ld b,010h
	ld de,l0103h
	call sub_1ba4h
	jp l1b27h
sub_1ba4h:
	call sub_1badh
	inc hl	
	dec b	
	jp nz,sub_1ba4h
	ret	
sub_1badh:
	push bc	
	push de	
	push hl	
	ld c,(hl)	
	ld de,00014h
	add hl,de	
	ld a,(hl)	
	cp 000h
	jp z,l1bceh
	ld b,008h
l1bbdh:
	rrca	
	push af	
	ld a,000h
	call c,sub_1bf3h
	xor c	
	rrca	
	ld c,a	
	pop af	
	dec b	
	jp nz,l1bbdh
	ld b,008h
l1bceh:
	ld de,00014h
	add hl,de	
	ld a,(hl)	
	cp 000h
	jp z,l1be9h
	ld b,008h
l1bdah:
	rrca	
	push af	
	ld a,000h
	call c,sub_1c17h
	xor c	
	rrca	
	ld c,a	
	pop af	
	dec b	
	jp nz,l1bdah
l1be9h:
	pop hl	
	pop de	
	ld a,c	
	ld (de),a	
	inc de	
	pop bc	
	ret	
l1bf0h:
	nop	
l1bf1h:
	nop	
	nop	
sub_1bf3h:
	push bc	
	push hl	
	ld hl,(l1bf1h)
	ld b,(hl)	
	ld hl,l1bf0h
	ld a,(hl)	
	ld c,a	
	rlca	
	ld (hl),a	
	cp 001h
	jp nz,l1c0ch
	ld hl,(l1bf1h)
	inc hl	
	ld (l1bf1h),hl
l1c0ch:
	ld a,b	
	and c	
	pop hl	
	pop bc	
	ret z	
	ld a,001h
	ret	
l1c14h:
	nop	
l1c15h:
	nop	
	nop	
sub_1c17h:
	push bc	
	push hl	
	ld hl,(l1c15h)
	ld b,(hl)	
	ld hl,l1c14h
	ld a,(hl)	
	ld c,a	
	rlca	
	ld (hl),a	
	cp 001h
	jp nz,l1c30h
	ld hl,(l1c15h)
	inc hl	
	ld (l1c15h),hl
l1c30h:
	ld a,b	
	and c	
	pop hl	
	pop bc	
	ret z	
	ld a,001h
	ret	
sub_1c38h:
	push af	
	push bc	
	push de	
	push hl	
	ld (hl),000h
	ld d,h	
	ld e,l	
	inc de	
	dec bc	
	ldir
	pop hl	
	pop de	
	pop bc	
	pop af	
	ret	
sub_1c49h:
	push de	
	ex de,hl	
	ld bc,00028h
	call sub_1c38h
	ex de,hl	
	ld b,014h
	ld c,001h
	ld d,000h
l1c58h:
	ld e,(hl)	
l1c59h:
	ld a,e	
	and c	
	jp z,l1c5fh
	inc d	
l1c5fh:
	ld a,c	
	rlca	
	ld c,a	
	cp 001h
	jp nz,l1c59h
	inc hl	
	dec b	
	jp nz,l1c58h
	ld a,d	
	and 0f8h
	rrca	
	rrca	
	rrca	
	ld l,a	
	ld h,000h
	ld a,d	
	and 007h
	inc a	
	ld b,a	
	ld a,080h
l1c7ch:
	rlca	
	dec b	
	jp nz,l1c7ch
	pop de	
	add hl,de	
	ld de,00014h
	add hl,de	
	ld (hl),a	
	ret	
sub_1c89h:
	push bc	
	push de	
	push hl	
	ld hl,l1cdah
	ld de,00014h
	ex de,hl	
	add hl,de	
	ex de,hl	
l1c95h:
	inc (hl)	
	ld a,(hl)	
	cp 000h
	jp z,l1ca8h
	ld b,a	
	ld a,(de)	
	and b	
	jp z,l1ca4h
	ld (hl),000h
l1ca4h:
	pop bc	
	pop de	
	pop hl	
	ret	
l1ca8h:
	inc hl	
	inc de	
	jp l1c95h
sub_1cadh:
	push bc	
	push de	
	push hl	
	ld hl,l1d02h
	ld de,00014h
	ex de,hl	
	add hl,de	
	ex de,hl	
l1cb9h:
	ld a,(hl)	
	or a	
	jp z,l1cd5h
	ld b,a	
	ld a,(de)	
	and b	
	jp nz,l1cd1h
	ld a,b	
	rlca	
	cp 001h
	jp nz,l1ccfh
	ld (hl),000h
	inc hl	
	inc de	
l1ccfh:
	ld (hl),a	
	xor a	
l1cd1h:
	pop hl	
	pop de	
	pop bc	
	ret	
l1cd5h:
	inc hl	
	inc de	
	jp l1cb9h
l1cdah:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l1d02h:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
sub_1d2ah:
	push af	
	push bc	
	push de	
	push hl	
	di	
	ld (l1d8dh),sp
	ld sp,l0105h
	pop iy
	pop ix
	pop hl	
	pop de	
	pop bc	
	pop af	
	ld sp,(l0111h)
l1d42h:
	nop	
l1d43h:
	nop	
	nop	
	nop	
	ld (l1d8bh),sp
	ld sp,l1d8bh
	push af	
	push bc	
	push de	
	push hl	
	push ix
	push iy
	ld sp,(l1d8dh)
	ei	
	ld hl,(l0103h)
	ld (l1d7dh),hl
	ld hl,l1d89h
	ld a,(hl)	
	and 0d7h
	ld (hl),a	
	ld b,010h
	ld de,l1d7dh
	ld hl,l1e85h
l1d6fh:
	ld a,(de)	
	inc de	
	call sub_1e49h
	dec b	
	jp nz,l1d6fh
	pop hl	
	pop de	
	pop bc	
	pop af	
	ret	
l1d7dh:
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
	nop	
l1d89h:
	nop	
	nop	
l1d8bh:
	nop	
	nop	
l1d8dh:
	nop	
	nop	
l1d8fh:
	ld a,(hl)	
	call sub_1dabh
	inc hl	
	dec b	
	jp nz,l1d8fh
	ret	
sub_1d99h:
	push af	
	push bc	
	push hl	
	ld b,004h
l1d9eh:
	ld a,(hl)	
	call sub_1dabh
	inc hl	
	dec b	
	jp nz,l1d9eh
	pop hl	
	pop bc	
	pop af	
	ret	
sub_1dabh:
	push af	
	rrca	
	rrca	
	rrca	
	rrca	
	call sub_1db4h
	pop af	
sub_1db4h:
	push af	
	push bc	
	push de	
	push hl	
	and 00fh
	cp 00ah
	jp c,l1dc1h
	add a,027h
l1dc1h:
	add a,030h
	ld e,a	
	ld c,002h
	call sub_1dceh
	pop hl	
	pop de	
	pop bc	
	pop af	
	ret	
sub_1dceh:
	push af	
	push bc	
	push de	
	push hl	
	call 00005h
	pop hl	
	pop de	
	pop bc	
	pop af	
	ret	
l1ddah:
	ld e,d	
	jr c,$+50
	jr nz,l1e48h
	ld l,(hl)	
	ld (hl),e	
	ld (hl),h	
	ld (hl),d	
	ld (hl),l	
	ld h,e	
	ld (hl),h	
	ld l,c	
	ld l,a	
	ld l,(hl)	
	jr nz,$+103
	ld a,b	
	ld h,l	
	ld (hl),d	
	ld h,e	
	ld l,c	
	ld (hl),e	
	ld h,l	
	ld (hl),d	
	ld a,(bc)	
	dec c	
	inc h	
l1df6h:
	ld d,h	
	ld h,l	
	ld (hl),e	
	ld (hl),h	
	ld (hl),e	
	jr nz,$+101
	ld l,a	
	ld l,l	
	ld (hl),b	
	ld l,h	
	ld h,l	
	ld (hl),h	
	ld h,l	
	inc h	
l1e05h:
	jr nz,$+34
	ld c,a	
	ld c,e	
	ld a,(bc)	
	dec c	
	inc h	
l1e0ch:
	jr nz,$+34
	ld b,l	
	ld d,d	
	ld d,d	
	ld c,a	
	ld d,d	
	jr nz,l1e3fh
	ld hl,(02a2ah)
	jr nz,l1e7dh
	ld (hl),d	
	ld h,e	
	jr nz,l1e83h
	ld a,b	
	ld (hl),b	
	ld h,l	
	ld h,e	
	ld (hl),h	
	ld h,l	
	ld h,h	
	ld a,(02024h)
	ld h,(hl)	
	ld l,a	
	ld (hl),l	
	ld l,(hl)	
	ld h,h	
	ld a,(l0a24h)
	dec c	
	inc h	
sub_1e32h:
	push bc	
	push de	
	push hl	
	ld de,l1e85h
	ld b,004h
l1e3ah:
	ld a,(de)	
	cp (hl)	
	jp nz,l1e45h
l1e3fh:
	inc hl	
	inc de	
	dec b	
	jp nz,l1e3ah
l1e45h:
	pop hl	
	pop de	
	pop bc	
l1e48h:
	ret	
sub_1e49h:
	push af	
	push bc	
	push de	
	push hl	
	push hl	
	ld de,00003h
	add hl,de	
	xor (hl)	
	ld l,a	
	ld h,000h
	add hl,hl	
	add hl,hl	
	ex de,hl	
	ld hl,l1e89h
	add hl,de	
	ex de,hl	
	pop hl	
	ld bc,00004h
l1e62h:
	ld a,(de)	
	xor b	
	ld b,(hl)	
	ld (hl),a	
	inc de	
	inc hl	
	dec c	
	jp nz,l1e62h
	pop hl	
	pop de	
	pop bc	
	pop af	
	ret	
sub_1e71h:
	push af	
	push bc	
	push hl	
	ld hl,l1e85h
	ld a,0ffh
	ld b,004h
l1e7bh:
	ld (hl),a	
	inc hl	
l1e7dh:
	dec b	
	jp nz,l1e7bh
	pop hl	
	pop bc	
l1e83h:
	pop af	
	ret	
l1e85h:
	nop	
	nop	
	nop	
	nop	
l1e89h:
	nop	
	nop	
	nop	
	nop	
	ld (hl),a	
	rlca	
	jr nc,$-104
	xor 00eh
	ld h,c	
	inc l	
	sbc a,c	
	add hl,bc	
	ld d,c	
	cp d	
	rlca	
	ld l,l	
	call nz,07019h
	ld l,d	
	call p,0e98fh
	ld h,e	
	and l	
	dec (hl)	
	sbc a,(hl)	
	ld h,h	
	sub l	
	and e	
	ld c,0dbh
	adc a,b	
	ld (0dc79h),a
	cp b	
	and h	
	ret po	
	push de	
	jp (hl)	
	ld e,097h
	jp nc,088d9h
	add hl,bc	
	or (hl)	
	ld c,h	
	dec hl	
	ld a,(hl)	
	or c	
	ld a,h	
	cp l	
	rst 20h	
	cp b	
l1ec3h:
	dec l	
	rlca	
	sub b	
	cp a	
	dec e	
	sub c	
	dec e	
	or a	
	djnz l1f31h
	ld l,d	
	or b	
	jr nz,l1ec3h
	di	
	cp c	
	ld (hl),c	
	ld c,b	
l1ed5h:
	add a,h	
	cp (hl)	
	ld b,c	
	sbc a,01ah
	jp c,07dd4h
	ld l,l	
	defb 0ddh,0e4h,0ebh	;illegal sequence
	call p,0b5d4h
	ld d,c	
	add a,e	
	out (085h),a
	rst 0	
	inc de	
	ld l,h	
	sbc a,b	
	ld d,(hl)	
	ld h,h	
	ld l,e	
	xor b	
	ret nz	
	defb 0fdh,062h	;ld iyh,d
	ld sp,hl	
	ld a,d	
	adc a,d	
	ld h,l	
	ret	
	call pe,l0113h+1
	ld e,h	
	ld c,a	
	ld h,e	
	ld b,06ch
	exx	
	jp m,03d0fh
	ld h,e	
	adc a,l	
	ex af,af'	
	dec c	
	push af	
	dec sp	
	ld l,(hl)	
	jr nz,l1ed5h
	ld c,h	
	ld l,c	
	djnz l1f6fh
	push de	
	ld h,b	
	ld b,c	
	call po,067a2h
	ld (hl),c	
	ld (hl),d	
	inc a	
	inc bc	
	call po,04bd1h
	inc b	
	call nc,0d247h
	dec c	
	add a,l	
	defb 0fdh,0a5h	;and iyl
	ld a,(bc)	
	or l	
	ld l,e	
	dec (hl)	
	or l	
	xor b	
	jp m,0b242h
	sbc a,b	
	ld l,h	
l1f31h:
	in a,(0bbh)
	ret	
	sub 0ach
	cp h	
	ld sp,hl	
	ld b,b	
	ld (06cd8h),a
	ex (sp),hl	
	ld b,l	
	rst 18h	
	ld e,h	
	ld (hl),l	
	call c,sub_0dd6h
	rst 8	
	xor e	
	pop de	
	dec a	
	ld e,c	
	ld h,0d9h
	jr nc,$-82
	ld d,c	
	sbc a,000h
	ld a,(0d7c8h)
	ld d,c	
	add a,b	
	cp a	
	ret nc	
	ld h,c	
	ld d,021h
	or h	
	call p,056b5h
	or e	
	call nz,0cf23h
	cp d	
	sub l	
	sbc a,c	
	cp b	
	cp l	
	and l	
	rrca	
	jr z,l1f6dh
	cp b	
	sbc a,(hl)	
l1f6dh:
	ld e,a	
	dec b	
l1f6fh:
	adc a,b	
	ex af,af'	
	add a,00ch
	exx	
	or d	
	or c	
	dec bc	
	jp (hl)	
	inc h	
	cpl	
	ld l,a	
	ld a,h	
	add a,a	
	ld e,b	
	ld l,b	
	ld c,h	
	ld de,061c1h
	dec e	
	xor e	
	or (hl)	
	ld h,(hl)	
	dec l	
	dec a	
	halt	
	call c,09041h
	ld bc,071dbh
	ld b,098h
	jp nc,0bc20h
	rst 28h	
	push de	
	djnz l1fc3h
	ld (hl),c	
	or c	
	add a,l	
	adc a,c	
	ld b,0b6h
	or l	
	rra	
	sbc a,a	
	cp a	
	call po,0e8a5h
	cp b	
	call nc,07833h
	rlca	
	ret	
	and d	
l1fadh:
	rrca	
	nop	
	ld sp,hl	
	inc (hl)	
	sub (hl)	
	add hl,bc	
	xor b	
	adc a,(hl)	
	pop hl	
	ld c,098h
	jr l2039h
	ld l,d	
	dec c	
	cp e	
	ex af,af'	
	ld l,l	
	dec a	
	dec l	
	sub c	
	ld h,h	
l1fc3h:
	ld l,h	
	sub a	
	and 063h
	ld e,h	
	ld bc,06b6bh
	ld d,c	
	call p,06c1ch
	ld h,c	
	ld h,d	
	add a,l	
	ld h,l	
	jr nc,l1fadh
	jp p,00062h
	ld c,(hl)	
	ld l,h	
	ld b,095h
	defb 0edh;next byte illegal after ed
	dec de	
	ld bc,07ba5h
	add a,d	
	ex af,af'	
	call p,0f5c1h
	rrca	
	call nz,06557h
	or b	
	exx	
	add a,012h
	or a	
	jp (hl)	
	ld d,b	
	adc a,e	
	cp (hl)	
	cp b	
	jp pe,0b9fch
	adc a,b	
	ld a,h	
	ld h,d	
	defb 0ddh,01dh,0dfh	;illegal sequence
	dec d	
	jp c,0492dh
	adc a,h	
	out (07ch),a
	di	
	ei	
	call nc,0654ch
	ld c,l	
	or d	
	ld h,c	
	ld e,b	
	ld a,(051b5h)
	adc a,0a3h
	cp h	
	nop	
	ld (hl),h	
	call nc,030bbh
	jp po,0df4ah
	and l	
	ld b,c	
	dec a	
	ret c	
	sub l	
	rst 10h	
	and h	
	pop de	
	call nz,0d36dh
	sub 0f4h
l2028h:
	ei	
	ld b,e	
	ld l,c	
	jp (hl)	
	ld l,d	
	inc (hl)	
	ld l,(hl)	
	exx	
	call m,067adh
	adc a,b	
	ld b,(hl)	
	jp c,0b860h
	ret nc	
l2039h:
	ld b,h	
	inc b	
sub_203bh:
	dec l	
	ld (hl),e	
	inc sp	
	inc bc	
	dec e	
	push hl	
	xor d	
	ld a,(bc)	
	ld c,h	
	ld e,a	
	defb 0ddh,00dh,07ch	;illegal sequence
	ret	
	ld d,b	
	dec b	
	ld (hl),c	
	inc a	
	daa	
	ld (bc),a	
	ld b,c	
	xor d	
	cp (hl)	
	dec bc	
	djnz $+18
	ret	
	inc c	
	jr nz,$-120
	ld d,a	
	ld l,b	
	or l	
	dec h	
	jr nz,l20ceh
	add a,l	
	or e	
	cp c	
	ld h,(hl)	
	call nc,0ce09h
	ld h,c	
	call po,05e9fh
	sbc a,0f9h
	ld c,029h
	exx	
	ret	
	sbc a,b	
	or b	
	ret nc	
	sbc a,b	
	ld (0d7c7h),hl
	xor b	
	or h	
	ld e,c	
	or e	
	dec a	
	rla	
	ld l,0b4h
	dec c	
l2080h:
	add a,c	
	or a	
	cp l	
	ld e,h	
	dec sp	
	ret nz	
	cp d	
	ld l,h	
	xor l	
	lddr
	add a,e	
	jr nz,l2028h
	cp a	
	or e	
	or (hl)	
	inc bc	
	or (hl)	
	jp po,0740ch
	or c	
	jp nc,0ea9ah
	push de	
	ld b,a	
	add hl,sp	
	sbc a,l	
	jp nc,0af77h
	inc b	
	in a,(026h)
	dec d	
	ld (hl),e	
	call c,08316h
	ex (sp),hl	
	ld h,e	
	dec bc	
	ld (de),a	
	sub h	
	ld h,h	
	dec sp	
	add a,h	
	dec c	
	ld l,l	
	ld l,d	
	ld a,07ah
	ld l,d	
	ld e,d	
	xor b	
	call po,0cf0eh
	dec bc	
	sub e	
	add hl,bc	
	rst 38h	
	sbc a,l	
	ld a,(bc)	
	nop	
	xor (hl)	
	daa	
	ld a,l	
	rlca	
	sbc a,(hl)	
	or c	
	ret p	
	rrca	
	sub e	
	ld b,h	
l20cdh:
	add a,a	
l20ceh:
	ex af,af'	
	and e	
	jp nc,0011eh
	jp p,06968h
	ld b,0c2h
	cp 0f7h
	ld h,d	
	ld d,a	
	ld e,l	
	add a,b	
	ld h,l	
	ld h,a	
	rr c
	ld l,h	
	ld (hl),071h
l20e5h:
	ld l,(hl)	
	ld l,e	
	ld b,0e7h
	cp 0d4h
l20ebh:
	dec de	
	halt	
	adc a,c	
	out (02bh),a
	ret po	
	djnz l20cdh
	ld a,d	
	ld e,d	
	ld h,a	
	defb 0ddh,04ah,0cch	;illegal sequence
	ld sp,hl	
	cp c	
	rst 18h	
	ld l,a	
	adc a,(hl)	
	cp (hl)	
	rst 28h	
	ld sp,hl	
	rla	
	or a	
	cp (hl)	
	ld b,e	
	ld h,b	
	or b	
	adc a,(hl)	
	push de	
	sub 0d6h
	and e	
	ret pe	
	and c	
	pop de	
	sub e	
	ld a,(hl)	
	jr c,l20ebh
	jp nz,04fc4h
	rst 18h	
	jp p,0d152h
	cp e	
	ld h,a	
	pop af	
	and (hl)	
	cp h	
	ld d,a	
	ld h,a	
	ccf	
	or l	
	ld b,0ddh
	ld c,b	
	or d	
	ld (hl),04bh
	ret c	
	dec c	
	dec hl	
	jp c,l0aafh
	dec de	
	ld c,h	
	ld (hl),003h
	ld c,d	
	or 041h
	inc b	
	ld a,d	
	ld h,b	
	rst 18h	
	ld h,b	
	rst 28h	
l213ch:
	jp 067a8h
	rst 18h	
	ld d,l	
	ld sp,08e6eh
	rst 28h	
	ld b,(hl)	
	ld l,c	
	cp (hl)	
	ld a,c	
	bit 4,c
	or e	
l214ch:
	adc a,h	
	cp h	
	ld h,(hl)	
	add a,e	
	ld a,(de)	
	dec h	
	ld l,a	
	jp nc,052a0h
	ld l,b	
	jp po,0cc36h
	inc c	
	ld (hl),a	
	sub l	
	cp e	
	dec bc	
	ld b,a	
	inc bc	
	ld (l1602h),hl
	cp c	
	ld d,l	
	dec b	
	ld h,02fh
	push bc	
	cp d	
	dec sp	
	cp (hl)	
	or d	
	cp l	
	dec bc	
	jr z,l219dh
	or h	
	ld e,d	
	sub d	
	ld e,h	
	or e	
	ld l,d	
	inc b	
	jp nz,0ffd7h
	and a	
	or l	
	ret nc	
	rst 8	
	ld sp,0d92ch
	sbc a,(hl)	
	adc a,e	
	ld e,e	
	sbc a,0aeh
	dec e	
	sbc a,e	
	ld h,h	
	jp nz,0ecb0h
	ld h,e	
	jp p,07526h
	ld l,d	
	and e	
	sbc a,h	
	ld (bc),a	
	ld l,l	
	sub e	
	ld a,(bc)	
	sbc a,h	
	add hl,bc	
	ld b,0a9h
l219dh:
	ex de,hl	
l219eh:
	ld c,036h
	ccf	
	ld (hl),d	
	rlca	
	ld h,a	
	add a,l	
	dec b	
	nop	
	ld d,a	
	inc de	
	sub l	
	cp a	
	ld c,d	
	add a,d	
	jp po,07ab8h
	inc d	
	ld a,e	
	or c	
	dec hl	
	xor (hl)	
	inc c	
	or (hl)	
	dec de	
	jr c,l214ch
	jp nc,09b8eh
	push hl	
	push de	
	cp (hl)	
	dec c	
	ld a,h	
	call c,0b7efh
	dec bc	
	in a,(0dfh)
	ld hl,0d386h
	jp nc,0f1d4h
	call nc,042e2h
	ld l,b	
	defb 0ddh,0b3h,0f8h	;illegal sequence
	rra	
	jp c,06e83h
	add a,c	
	cp (hl)	
	ld d,0cdh
	or 0b9h
	ld h,05bh
	ld l,a	
	or b	
	ld (hl),a	
	pop hl	
	jr l219eh
	ld b,a	
	ld (hl),a	
	adc a,b	
	ex af,af'	
	ld e,d	
	and 0ffh
	rrca	
	ld l,d	
	ld (hl),b	
	ld h,(hl)	
	ld b,03bh
	jp z,l0111h
	dec bc	
	ld e,h	
	adc a,a	
	ld h,l	
	sbc a,(hl)	
l21fch:
	rst 38h	
	ret m	
	ld h,d	
	xor (hl)	
	ld l,c	
l2201h:
	ld h,c	
	ld l,e	
	rst 38h	
l2204h:
	out (016h),a
	ld l,h	
l2207h:
	rst 8	
	ld b,l	
	and b	
l220ah:
	ld a,(bc)	
	jp po,0d778h
	dec c	
	jp nc,04eeeh
	inc b	
l2213h:
	add a,e	
	ld d,h	
	add hl,sp	
	inc bc	
	or e	
	jp nz,067a7h
	ld h,061h
	ret nc	
	ld h,b	
	ld d,0f7h
	ld c,c	
	ld l,c	
	ld b,a	
	ld c,l	
	ld a,06eh
	ld (hl),a	
	in a,(0aeh)
	pop de	
	ld l,d	
	ld c,d	
	exx	
	sub 05ah
	call c,0df40h
	dec bc	
	ld h,(hl)	
	scf	
	ret c	
	dec sp	
	ret p	
	xor c	
	cp h	
	xor (hl)	
	ld d,e	
	sbc a,0bbh
	sbc a,(hl)	
	push bc	
	ld b,a	
	or d	
	rst 8	
	ld a,a	
	jr nc,l21fch
	rst 38h	
	jp (hl)	
	cp l	
	cp l	
	jp p,0ca1ch
	cp d	
	jp nz,0538ah
	or e	
	sub e	
	jr nc,l227ah
	or h	
	and e	
	and (hl)	
	cp d	
	ret nc	
	ld (hl),005h
	call sub_06d7h
	sub e	
	ld d,h	
	sbc a,057h
	add hl,hl	
	inc hl	
	exx	
	ld h,a	
	cp a	
	or e	
	ld h,(hl)	
	ld a,d	
	ld l,0c4h
	ld h,c	
	ld c,d	
	cp b	
	ld e,l	
	ld l,b	
	dec de	
	ld (bc),a	
	ld hl,(02b6fh)
	sub h	
	or h	
l227ah:
	dec bc	
	cp (hl)	
	scf	
	jp 08e0ch
	and c	
	ld e,d	
	dec b	
	rst 18h	
	dec de	
	dec l	
	ld (bc),a	
	rst 28h	
	adc a,l	
