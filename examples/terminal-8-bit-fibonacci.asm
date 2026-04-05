@include "okto.h"

@macro CURRENT 255   # current fibonacci value
@macro NEXT 254      # next fibonacci value
@macro TEMP 253      # temporary storage

.code
    li $sp, CURRENT      # point SP to CURRENT slot
    li $a, 0             # A = 0 (first fibonacci number)
    st $a, $sp           # store A into CURRENT

    li $sp, NEXT         # point SP to NEXT slot
    li $a, 1             # A = 1 (second fibonacci number)
    st $a, $sp           # store A into NEXT

LOOP:
    li $sp, CURRENT      # SP -> CURRENT
    ld $a, $sp           # A = current

    li $sp, TEMP         # SP -> TEMP
    st $a, $sp           # TEMP = current (save for later)

    la END               # load address of END into B:A
    swpx                 # move B:A into X (jump target)

    li $sp, TEMP         # SP -> TEMP
    ld $a, $sp           # A = current
    li $b, 100           # B = 100 (limit)
    jgt                  # if A > B, jump to END

    li $c, OKTO_PRINTLN_UNSIGNED  # select print unsigned syscall
    call                 # print A

    li $sp, TEMP         # SP -> TEMP
    st $a, $sp           # TEMP = current (preserve value)

    li $sp, NEXT         # SP -> NEXT
    ld $a, $sp           # A = next

    li $sp, CURRENT      # SP -> CURRENT
    st $a, $sp           # CURRENT = next

    li $sp, TEMP         # SP -> TEMP
    ld $b, $sp           # B = old current

    add                  # A = A + B (new fibonacci value)

    li $sp, NEXT         # SP -> NEXT
    st $a, $sp           # NEXT = new value

    la LOOP              # load address of LOOP into B:A
    swpx                 # move B:A into X
    jmp                  # jump to LOOP

END:
    li $c, OKTO_EXIT     # select exit syscall
    call                 # terminate program