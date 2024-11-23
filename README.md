# DeskCalc
Random Rust calculator project for personal use. 
Designed for keyboard use - just type expressions and instantly see results.
# Features
- Assign custom variables to use in more calculations
- Familiar code-like syntax for function calls and operations
- Supports vector math (WIP)
- User-defined functions (WIP)
- Type commands using '/' to quickly perform calculator functions (WIP)
# Usage
## Basic Math
- Operators +, -, /, *, for basic operations and ^ for exponents
- Operators with no number to the right assume 1: 1+ = 2
- Variables and constants (e, pi, tau) can be used by their name
- Functions can be called using syntax function_name(param1,  *any others here...*)
- All whitespace is ignored
## Defining Variables and Functions
- Variables can be defined by writing #*var_name* = (expression)
- Any defined variables can then be used later or reassigned to
- Stored variables can be cleared with the /clearvars command or /clear to include other things
- functions wip
## Vectors
- Vectors can be created using square brackets (e.x. [1, 2, 3])
- Support operators +, -, /, *, ^, just like scalars, works with either vector or scalar to right
- Individual components can be accessed using . followed by x, y, z, or a number (vectors are 0-indexed)
- Built in functions include (WIP):
    - mag() - takes the magnitude of the vector
