# xorelang

`xorelang` is a simple programming language that supports the `log:` function.

## Features

- `log:` function: prints a message to the console.

## Usage

Create a `.xor` file with your code. For example:

// file.xor log: ‘Hello, world!’ log: 123 log: ‘Another message’


Then run the interpreter with the `.xor` file as input:

./main.exe file.xor


This will produce the following output:

Hello, world! 123 Another message


## Examples

Here are some examples of how to use the `log:` function:

```xorelang
// Print a string
log: 'Hello, world!'

// Print a number
log: 123

// Print multiple messages
log: 'First message'
log: 'Second message'
