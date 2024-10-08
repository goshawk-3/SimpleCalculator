# Simple Calculator
A simple library for evaluating arithmetic expressions.


### Description

The library (libcalc) exposes a function `try_eval` that does the following:

- Convert infix notation to Reverse Polish Notation by using Shunting Yard algorithm.
- Perform stack-based evaluation for supported binary operators.

Supported operators: `+, - , (, ), *, /`

Supported operands: signed long integer
 
### Testing
List of unit tests ensures all valid and edge cases are addressed properly.

### Run
``make run_cli`` - Runs a binary that uses library crate to evaluate expressions from stdin

``make test`` - Run all unit tests

### How to support decimal operands

To enable Decimal operands, the follow tasks should be implemented:
- Support decimal point in `yarn::to_rpn` parsing.
- Set `f64` for `type OperandType = i32;`
- Use `println!("{:.4}", result);` to print result with precision of 4

### How to support HEX operands

To enable HEX operands, the follow tasks should be implemented:
- Use `i32::from_str_radix(value, 16)` instead of `parse<>` in `yarn::to_rpn` to convert HEX operand into decimal
- Use `format!("{:x}", result)` to print the result of evaluation in HEX format


