# OKTO Instructions

This table presents all native instructions of the **OKTO Virtual Machine**,
including their formats, descriptions, and syntax.

| Instruction | Format | Syntax          | Description                                                                 |
|:------------|:------:|:----------------|:----------------------------------------------------------------------------|
| **lli**     | Alpha  | `lli $r, imm4`  | Loads `imm4` into the lower nibble of `$r`; the upper nibble is preserved.  |
| **lai**     | Alpha  | `lai $r, imm4`  | Loads `imm4` into the upper nibble of `$r`; the lower nibble is preserved.  |
| **lxi**     | Alpha  | `lxi $r, imm4`  | Sign-extends `imm4` to 8 bits and stores the result in `$r`.                |
| **mv**      | Beta   | `mv $rd, $rs`   | Copies the value of `$rs` into `$rd`.                                       |
| **ld**      | Beta   | `ld $rd, $rs`   | Loads the stack byte at offset `$rs` into `$rd`.                            |
| **st**      | Beta   | `st $rs, $rd`   | Stores the value of `$rs` into the stack byte at offset `$rd`.              |
| **add**     | Gamma  | `add`           | Adds `$a` and `$b`, storing the result in `$a`; carry is stored in `$f`.     |
| **sub**     | Gamma  | `sub`           | Subtracts `$b` from `$a`, storing the result in `$a`; borrow is stored in `$f`. |
| **and**     | Gamma  | `and`           | Performs bitwise AND between `$a` and `$b`, storing the result in `$a`.     |
| **or**      | Gamma  | `or`            | Performs bitwise OR between `$a` and `$b`, storing the result in `$a`.      |
| **xor**     | Gamma  | `xor`           | Performs bitwise XOR between `$a` and `$b`, storing the result in `$a`.     |
| **not**     | Gamma  | `not`           | Performs bitwise NOT on `$a`, storing the result in `$a`.                   |
| **shr**     | Gamma  | `shr`           | Shifts `$a` right by `$b`, storing the result in `$a` and shifted bits in `$f`. |
| **shl**     | Gamma  | `shl`           | Shifts `$a` left by `$b`, storing the result in `$a` and shifted bits in `$f`. |
| **jmp**     | Gamma  | `jmp`           | Jumps to the address in `$x`, then stores the next instruction address in `$x`. |
| **jeq**     | Gamma  | `jeq`           | If `$a` equals `$b`, jumps through `$x` like `jmp`.                         |
| **jgt**     | Gamma  | `jgt`           | If `$a` is greater than `$b`, jumps through `$x` like `jmp`.                |
| **incsp**   | Gamma  | `incsp`         | Increments `$sp` with wrapping arithmetic.                                  |
| **decsp**   | Gamma  | `decsp`         | Decrements `$sp` with wrapping arithmetic.                                  |
| **swpf**    | Gamma  | `swpf`          | Swaps `$f` and `$b`.                                                        |
| **swpx**    | Gamma  | `swpx`          | Swaps `$x` with the 16-bit value formed by `$b:$a`.                         |
| **call**    | Gamma  | `call`          | Calls the attached VM interface; the operation is usually selected by `$c`. |

The **OKTO** assembler turns these instructions into a binary format that can
be executed by the VM. Each instruction is encoded as a single byte based on its
format, which determines how operands are represented and how the instruction is
interpreted.

## Register Operands

Native instructions can use the general registers `$a`, `$b`, `$c`, and `$sp`.
The special registers `$f`, `$x`, `$pc`, and `$ir` are used implicitly by some
instructions, but they are not valid explicit operands in assembly code.

Stack access instructions use an 8-bit stack offset. The real memory address is
computed by adding the offset to `0xFF00`.

## Binary Formats

Binary formats are the specific encoding of instructions in **OKTO**. Each
instruction is represented by an 8-bit binary code. The following table shows
the binary structure of each native instruction:

| Instruction | 7...4 | 3...2 | 1...0 |
|:------------|:-----:|:-----:|:-----:|
| **lli**     | iiii  | rr    | 00    |
| **lai**     | iiii  | rr    | 01    |
| **lxi**     | iiii  | rr    | 10    |

| Instruction | 7...6 | 5...4 | 3...2 | 1...0 |
|:------------|:-----:|:-----:|:-----:|:-----:|
| **mv**      | ss    | dd    | 00    | 11    |
| **ld**      | ss    | dd    | 01    | 11    |
| **st**      | ss    | dd    | 10    | 11    |

| Instruction | 7...4 | 3...0 |
|:------------|:-----:|:-----:|
| **add**     | 0000  | 1111  |
| **sub**     | 0001  | 1111  |
| **and**     | 0010  | 1111  |
| **or**      | 0011  | 1111  |
| **xor**     | 0100  | 1111  |
| **not**     | 0101  | 1111  |
| **shr**     | 0110  | 1111  |
| **shl**     | 0111  | 1111  |
| **jmp**     | 1000  | 1111  |
| **jeq**     | 1001  | 1111  |
| **jgt**     | 1010  | 1111  |
| **incsp**   | 1011  | 1111  |
| **decsp**   | 1100  | 1111  |
| **swpf**    | 1101  | 1111  |
| **swpx**    | 1110  | 1111  |
| **call**    | 1111  | 1111  |

In these tables, `iiii`, `rr`, `ss`, and `dd` represent encoded fields:

| Field | Meaning                  |
|:-----:|:-------------------------|
| `iiii` | 4-bit immediate value    |
| `rr`   | Register operand         |
| `ss`   | Source register          |
| `dd`   | Destination register     |

The register codes are:

| Register | Code |
|:--------:|:----:|
| `$a`     | `00` |
| `$b`     | `01` |
| `$c`     | `10` |
| `$sp`    | `11` |

Formats categorize instructions by their structure and usage. See the
[formats documentation](/docs/formats.md) for more details.

Pseudo-instructions can be found in the
[pseudo-instructions documentation](/docs/pseudo-instructions.md).
