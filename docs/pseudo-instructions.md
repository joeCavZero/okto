# Pseudo-Instructions

Pseudo-instructions are higher-level assembly commands that make programming
easier and more expressive. They are not directly supported by the virtual
machine, but are expanded by the assembler into one or more native instructions
before code generation.

---

## How Pseudo-Instructions Are Resolved

During compilation, the resolver first reserves the amount of instruction space
needed by each pseudo-instruction. This allows labels to point to the correct
final instruction addresses even when a pseudo-instruction expands into multiple
native instructions.

After the symbol table is built, each pseudo-instruction is replaced by the
native instruction sequence that implements the same behavior.

For example, the pseudo-instruction `li $a, 72` is replaced by `lli` and `lai`,
because native OKTO instructions can only load 4 immediate bits at a time.

---

## Pseudo-Instructions Reference

### `nope`

Performs no operation.

```python
nope
```

**Expands to:**

```python
mv $a, $a
```

---

### `li`

Loads the lower 8 bits of a numeric literal into a register.

```python
li $r, imm
```

**Expands to:**

```python
lli $r, imm<3...0>
lai $r, imm<7...4>
```

The lower byte of the immediate is split into its lower and upper nibbles. The
lower nibble is loaded first with `lli`, and the upper nibble is loaded after
that with `lai`.

Example:

```python
li $a, 72
```

**Expands to:**

```python
lli $a, 8
lai $a, 4
```

---

### `lchr`

Loads an 8-bit character literal into a register.

```python
lchr $r, 'c'
```

**Expands to:**

```python
lli $r, char<3...0>
lai $r, char<7...4>
```

The character must fit in one byte.

Example:

```python
lchr $a, 'A'
```

**Expands to:**

```python
lli $a, 1
lai $a, 4
```

---

### `lla`

Loads the lower byte of a label address into a register.

```python
lla $r, label
```

**Expands to:**

```python
lli $r, label<3...0>
lai $r, label<7...4>
```

This is useful when only the low byte of an address is needed, such as when
working with an 8-bit stack offset or a low address value.

---

### `laa`

Loads the upper byte of a label address into a register.

```python
laa $r, label
```

**Expands to:**

```python
lli $r, label<11...8>
lai $r, label<15...12>
```

This is useful when the high byte of a 16-bit address needs to be loaded
separately.

---

### `la`

Loads the 16-bit address of a label into `$b:$a`.

```python
la label
```

**Expands to:**

```python
lli $a, label<3...0>
lai $a, label<7...4>
lli $b, label<11...8>
lai $b, label<15...12>
```

The low byte is loaded into `$a`, and the high byte is loaded into `$b`. To jump
to the loaded address, move `$b:$a` into `$x` with `swpx`, then use `jmp`,
`jeq`, or `jgt`.

Example:

```python
la LOOP
swpx
jmp
```

This loads the address of `LOOP`, swaps it into `$x`, and jumps to it.

---

## Notes on Addressing

Code labels resolve to instruction addresses. Since each OKTO instruction is one
byte, a code label address is also the byte offset of that instruction in the
`.code` section.

Data labels resolve to byte offsets inside their data section. These labels can
also be used with address-loading pseudo-instructions when the resulting offset
fits the intended use.

You can read more about native instructions in the
[instructions documentation](/docs/instructions.md) and about binary layouts in
the [formats documentation](/docs/formats.md).
