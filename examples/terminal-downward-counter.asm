@include "okto.h"

@macro INITIAL_VALUE 30

.code
    li $a, INITIAL_VALUE            # $a = INITIAL_VALUE
    st $a, $sp                      # $a = *$sp

LOOP:
    ld $a, $sp                      # $a = *$sp
    li $c, OKTO_PRINTLN_UNSIGNED    # prints $a
    call                            # syscall

    la END                          # (b:a) = END
    swpx                            # x = (b:a)

    ld $a, $sp                      # $a = *$sp
    li $b, 1                        # b = 1
    sub                             # $a = $a - $b

    st $a, $sp                      # *$sp = $a

    swpf                            # b = f

    li $a, 1                        # a = 1

    jeq                             # if $a == $b, goto END

    la LOOP                         # (b:a) = LOOP
    swpx                            # x = (b:a)
    jmp                             # goto x (which is loop)

END:
    li $c, OKTO_EXIT                # c = OKTO_EXIT
    call                            # syscall