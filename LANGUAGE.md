# MatrixStack-Code

## Basics

In a MatrixStack program, each program is a 2d matrix of instructions, where each 4x4 square of instructions has access to a stack. A pointer with a velocity and a register traverses the matrix and performs the instructions at its position. Its initial position is 0, 0 (top left) and its initial velocity is to the right.

The code is split into 3 types:
- Comments - Ignored text prefaced by a `#`
- Headers - control statements prefaced by a header character
- Body - code characters

## Comments

Lines whose first character is a `#` are comment lines, they are ignored entirely.

Example: `# Comment on what the program does`

Lines whose first character is not a `#` are not comment lines, but may contain comments, after which the rest of the line will be ignored.

Example: `>+p # Comment on what this line does`

## Headers

Headers set information to be used in the program.

### Stack Headers

To set a stack, use the `s` header character, then the x-coordinate and y-coordinate of the stack, then the stack entries in push order (last entry will be first to be popped) all space-separated.

Example: `s 2 3 15 -12 32`

## Body

The body contains the instructions. It is a 2d matrix. Any line that is not interpreted as another type (even a blank line) is, by default, a body line. A body line must only contain valid instructions.

## Instructions

There are 5 types of instructions:
- Space - do nothing
- Deflector - change velocity (direction)
- Operator - perform mathematical and stack operations
- Comparator - perform comparisons
- IO - input / output

### Space

The space instruction (` `) does nothing and the pointer will continue moving through it.

### Deflector

There are 2 types of deflectors:
- Arrows - set the velocity
- Mirror - change the velocity depending on the current velocity

#### Arrows

The arrows will set the velocity to the direction they point in:
- Right Arrow - `>` (greater than)
- Left Arrow - `<` (less than)
- Up Arrow - `^` (caret)
- Down Arrow - `v` (lowercase vee)

#### Mirrors

The mirrors will set the velocity depending on the incoming velocity and the angle of the mirror:
- Omnidirectional Mirror - `o` (lowercase oe) - swaps right and left, and up and down
- Forward Mirror - `/` (forward slash) - swaps right and up, and left and down
- Backward Mirror - `\` (backslash) - swaps right and down, and left and up

### Operator

It should be noted that the stack that the pointer is over at any time is $(\left\lfloor x \over 4 \right\rfloor, \left\lfloor y \over 4 \right\rfloor)$ (floor of coordinates divided by 4), where x and y are the pointer's coordinates.
Also, the register referred to in this section is the pointer's register.

There are 3 types of operators:
- Stack Operators - for modifying the underlying stack
- Numerical Operators - for performing numerical operations
- Bitwise Operators - for performing bitwise operations

#### Stack Operators

These operators modify the stack the pointer is over:
- Push - `,` (comma) - pushes the value of the register onto the stack
- Pop - `.` (dot).- pops the top value off the stack and sets the register to it (defaulting to 0 when empty)
- Duplicate - `d` - duplicates the top value on the stack (defaulting to 0 when empty)

#### Numerical Operators

All numerical operations are wrapping.

These operators perform numerical operations with the stack the pointer is over:
- Add - `+` - pops the top value off the stack and adds it to the register
- Subtract - `-` (dash) - pops the top value off the stack and subtracts it from the register
- Multiply - `*` - pops the top value off the stack and multiplies it with the register, which becomes the new register value
- Divide - `~` (tilde) - pops the top value off the stack and divides the register by it (unless 0), which becomes the new register value

#### Bitwise Operators

These operators perform bitwise operations with the stack the pointer is over:
- Not - `!` - sets the register to the bitwise not of itself
- Or - `|` (vertical bar / pipe) - pops the top value off the stack performs a bitwise or with the register, which becomes the new register value
- And - `&` - pops the top value off the stack performs a bitwise and with the register, which becomes the new register value
- Xor - `:` - pops the top value off the stack performs a bitwise xor with the register, which becomes the new register value

### Comparator

In comparators, the register is compared with another value. If the register is greater than the value, the pointer's velocity will be rotated 90 degrees clockwise; if the register is equal to the value, the pointer's velocity will not be changed; and if the register is greater than the value, the pointer's velocity will be rotated 90 degrees anti-clockwise.

Diagram for entering from the left:
```
    greater
      ^
      |
---->   ----> equal
      |
      v
     less
```

There are 2 types of comparator:
- Zero Comparator - `z` - compares the register to zero
- Stack Comparator - `c` - pops the top value off the stack and compares the register to it

### IO

There are 2 input / output operations:
- Print - `p` - outputs the value of the register
- Input - `i` - takes an input, which becomes the new register value

## Files

The recommended file extension is `.msc` and the recommended encoding is utf-8.

## Examples

There are example files in [examples](/examples).
