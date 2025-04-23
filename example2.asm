.ORG $FFFC
.BYTES $00 $06

.ORG $0010
.BYTES $20

.ORG $0600
lda $10
adc #$55
sta $F0
