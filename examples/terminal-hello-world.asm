@include "okto.h"

@macro PUSH_LITERAL(%char) \
    lchr $c, %char \    # C = literal character
    la PUSH_C \                                # load address of PUSH_C into B:A
    swpx \                                     # move B:A into X
    jmp                                        # jump to PUSH_C

.code
    li $sp, 255                    # initialize SP at the top of the stack

    li $c, 0                       # C = '\0' terminator
    la PUSH_C                      # load address of PUSH_C into B:A
    swpx                           # move B:A into X
    jmp                            # jump to PUSH_C

    PUSH_LITERAL('d')
    PUSH_LITERAL('l')
    PUSH_LITERAL('r')
    PUSH_LITERAL('o')
    PUSH_LITERAL('W')
    PUSH_LITERAL(' ')
    PUSH_LITERAL(',')
    PUSH_LITERAL('o')
    PUSH_LITERAL('l')
    PUSH_LITERAL('l')
    PUSH_LITERAL('e')
    PUSH_LITERAL('H')

PRINT_LOOP:
    mv $a, $sp                     # A = SP
    li $b, 1                       # B = 1
    add                            # A = SP + 1
    mv $sp, $a                     # SP = SP + 1

    ld $a, $sp                     # A = *SP
    mv $c, $a                      # C = current character

    la END                         # load address of END into B:A
    swpx                           # move B:A into X
    mv $a, $c                      # A = current character
    li $b, 0                       # B = '\0'
    jeq                            # if current character == 0, jump to END

    mv $a, $c                      # A = current character
    li $c, OKTO_PRINT_CHAR         # C = print char syscall
    call                           # print character

    la PRINT_LOOP                  # load address of PRINT_LOOP into B:A
    swpx                           # move B:A into X
    jmp                            # jump back to PRINT_LOOP

END:
    lchr $a, '\n'                  # A = newline
    li $c, OKTO_PRINT_CHAR         # C = print char syscall
    call                           # print newline

    li $c, OKTO_EXIT               # C = exit syscall
    call                           # terminate program

PUSH_C:
    mv $a, $c                      # A = C
    st $a, $sp                     # *SP = character
    mv $a, $sp                     # A = SP
    li $b, 1                       # B = 1
    sub                            # A = SP - 1
    mv $sp, $a                     # SP = SP - 1
    jmp                            # return to routine caller