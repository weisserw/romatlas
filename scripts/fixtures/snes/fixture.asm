; Original synthetic hardware probe (MIT). No external ROM/assets.
; The Python driver substitutes @MAPPING@, @BASE@, @HEADER@, @END@, @SIZE@.
@MAPPING@
org @BASE@
reset:
    sei
    clc
    xce
    rep #$10
    sep #$20
    ldx.w #$1fff
    txs
    stz.w $4200
    lda.b #$0f
    sta.w $2100
    stz.w $2121
    lda.b #$1f
    sta.w $2122
    stz.w $2122
    ; Native 16-bit immediate and explicit long CPU alias encodings.
    rep #$20
    lda.w #$1234
    sta.l $7e0100
    sep #$20
    lda.l reset
    sta.l $7e0102
    lda.b #$5a
    sta.l @SRAM@
    ; IPL standard upload handshake, with 8-bit counter and 16-bit X.
wait_ipl:
    lda.w $2140
    cmp.b #$aa
    bne wait_ipl
    ldx.w #$0200
    stx.w $2142
    lda.b #$01
    sta.w $2141
    lda.b #$cc
    sta.w $2140
wait_ready:
    cmp.w $2140
    bne wait_ready
    ldx.w #$0000
upload:
    lda.l payload,x
    sta.w $2141
    txa
    sta.w $2140
wait_byte:
    cmp.w $2140
    bne wait_byte
    inx
    cpx.w #payload_end-payload
    bne upload
    stz.w $2141
    ldx.w #$0200
    stx.w $2142
    clc
    adc.b #$02
    sta.w $2140
wait_start:
    cmp.w $2140
    bne wait_start
    ; Poll B through the real serial controller registers.
loop:
    lda.b #$01
    sta.w $4016
    stz.w $4016
    lda.w $4016
    and.b #$01
    sta.l $7e0110
    inc.w $0111
    bra loop

payload:
; SPC execution address differs from ROM storage. No assumed upload envelope.
arch spc700
base $0200
    mov $f2,#$6c
    mov $f3,#$20       ; unmute, disable echo
    mov $f2,#$0c
    mov $f3,#$7f       ; main left
    mov $f2,#$1c
    mov $f3,#$7f       ; main right
    mov $f2,#$00
    mov $f3,#$7f       ; voice left
    mov $f2,#$01
    mov $f3,#$7f       ; voice right
    mov $f2,#$05
    mov $f3,#$00       ; gain mode
    mov $f2,#$07
    mov $f3,#$7f       ; direct gain
    mov $f2,#$3d
    mov $f3,#$01       ; noise voice 0
    mov $f2,#$6c
    mov $f3,#$3f       ; noise frequency, echo disabled
    mov $f2,#$4c
    mov $f3,#$01       ; key on
spc_loop:
    inc $20
    bra spc_loop
base off
arch 65816
payload_end:

org @HEADER@
    db "ROMATLAS SYNTHETIC   " ; 21 bytes
    db @MODE@,$02,@SIZE@,$01,$01,$00,$00
    dw $edcb,$1234 ; intentionally invalid checksum, must survive reconstruction
org @VECTOR@
    dw reset
org @END@
    db $5a
