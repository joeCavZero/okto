# Processors

Processors are assembly-time commands handled before parsing code and data
sections. They do not produce VM instructions directly. Instead, they rewrite
the token stream used by the assembler.

OKTO currently supports three processors:

| Processor  | Syntax                    | Purpose                                      |
|:-----------|:--------------------------|:---------------------------------------------|
| `@include` | `@include "path"`         | Inserts another assembly file at that point. |
| `@macro`   | `@macro NAME body`        | Defines a token replacement macro.           |
| `@once`    | `@once`                   | Prevents the rest of a file from being included more than once. |

---

## Include

The `@include` processor loads another file and inserts its processed tokens
where the include appears.

```python
@include "okto.h"
```

Relative include paths are resolved from the directory of the file that contains
the `@include`.

For example, if `examples/game.asm` contains:

```python
@include "okto.h"
```

the assembler reads `examples/okto.h`.

Include cycles are rejected. If `a.asm` includes `b.asm` and `b.asm` includes
`a.asm`, compilation fails instead of expanding forever.

---

## Once

The `@once` processor is used by include files that should only contribute their
contents once.

```python
@once

@macro OKTO_EXIT 200
@macro OKTO_PRINT_CHAR 203
```

Place `@once` at the top of a header-style file. The first time that file is
processed, `@once` is removed and the remaining contents are kept. If the same
file is included again, everything from `@once` onward is skipped.

This makes shared macro files safe to include from multiple places.

---

## Macro

The `@macro` processor defines a name that expands to one or more tokens.

```python
@macro SCREEN_WIDTH 255
```

After this definition, every later use of `SCREEN_WIDTH` expands to `255`.

```python
li $a, SCREEN_WIDTH
```

is processed as:

```python
li $a, 255
```

Macro definitions are removed from the final token stream. They only affect
tokens that appear after the definition has been processed.

## Macro Arguments

Macros can take arguments. Arguments are written with `%` in the definition and
are passed in parentheses at the call site.

```python
@macro PUSH_REG(%reg) \
    decsp \
    st %reg, $sp
```

Example call:

```python
PUSH_REG($a)
```

**Expands to:**

```python
decsp
st $a, $sp
```

A macro call must provide exactly the number of arguments declared by the macro.

## Multi-Line Macros

By default, a macro body ends at the end of its line.

```python
@macro ZERO_A li $a, 0
```

Use a trailing backslash to continue the macro body on the next line.

```python
@macro PUSH_VALUE(%value) \
    decsp \
    li $c, %value \
    st $c, $sp
```

The backslash itself is not part of the expansion.

## Processor Order

Processors are resolved while the assembler walks through the token stream:

1. `@include` inserts the included file's processed tokens.
2. `@macro` records a macro and removes the definition.
3. `@once` controls whether the current file should keep being processed.
4. Macro calls expand when their identifier is encountered.

This means shared macro definitions usually belong before the code that uses
them, either directly in the same file or through an `@include`.

You can read more about code expanded after processing in the
[pseudo-instructions documentation](/docs/pseudo-instructions.md), and about
section directives in the [directives documentation](/docs/directives.md).
