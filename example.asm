  LDY #$01
  LDA #$03
  STA $01
  LDA #$07
  STA $02
  LDX #$0a
  STX $0704
  LDA ($010),Y
