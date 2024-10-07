@pattern
M=0
(LOOP)
    @KBD
    D=M

    // if D = 0 clear screen
    @CLEAR
    D;JEQ
    @pattern
    M=-1
    @FILL
    0;JMP

    (CLEAR)
        @pattern
        M=0

    (FILL)
        @8096
        D=A
        @i
        M=D
        @offset

        (PRINT)
            @i
            D=M
            @LOOP
            D;JLT

            @SCREEN
            D=D+A
            @offset
            M=D

            @pattern
            D=M

            @offset
            A=M
            M=D

            @i
            M=M-1

            @PRINT
            0;JMP
            

    // those are never executed, just for safety 
    @LOOP
    0;JMP


