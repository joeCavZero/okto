@include "okto.h"

@macro INITIAL_VALUE 100    # initial counter value

.code
    li $sp, INITIAL_VALUE       # SP = initial counter value

LOOP:
    mv $a, $sp                      # A = SP (copy counter to A)
    li $c, OKTO_PRINTLN_UNSIGNED    # select print unsigned syscall
    call                            # print A

    mv $a, $sp                 # A = SP (load counter again)
    li $b, 1                   # B = 1 (decrement value)
    sub                        # A = A - B (A = SP - 1)
    mv $sp, $a                 # SP = A (update counter)

    la END                     # load address of END into B:A
    swpx                       # move B:A into X (prepare jump target)

    swpf                       # swap A <-> F (F now holds result flags)
    li $b, 1                   # B = 1 (compare value)
    jeq                        # if F == B, jump to END (counter reached 0)

    la LOOP                    # load address of LOOP into B:A
    swpx                       # move B:A into X
    jmp                        # jump to LOOP

END:
    li $c, OKTO_EXIT           # select exit syscall
    call                       # terminate program