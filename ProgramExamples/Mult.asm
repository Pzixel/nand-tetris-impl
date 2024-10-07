// Multiplication: in the Hack computer, the top 16 RAM words (RAM[0]...RAM[15]) are also referred to as R0...R15. 

// With this terminology in mind, this program computes the value R0*R1 and stores the result in R2.

// The program assumes that R0>=0, R1>=0, and R0*R1<32768. Your program need not test these conditions, but rather assume that they hold.

@R1
D=M
@i
M=D
@R2
M=0
(LOOP)
  // if i == 0 goto END

  @i
  D=M
  @END
  D;JEQ

  // R2 = R2 + R0
  @R0
  D=M
  @R2
  M=D+M

  @i
  M=M-1

  @LOOP
  0;JMP

(END)
  @END
  0;JMP