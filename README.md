# calc
A simple library for evaluating arithmetic expressions.


### Description

The library (libcalc) exposes a function `try_eval` that does the following:

- Convert infix notation to Reverse Polish Notation
- Perform stack-based evaluation for supported binary operator by using.

Supported operators: `+, - , (, ), *, /`

Supported operands: signed long integer
 
### Testing
List of unit tests ensures all valid and edge cases are addressed properly.


### Run
``make run_cli`` - Runs a binary that uses library crate to evaluate expressions from stdin

