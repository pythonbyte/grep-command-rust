# Rust Grep-like Tool

## Introduction

This project implements a simple grep-like tool in Rust. It provides basic regular expression matching functionality, allowing users to search for patterns in text input. This tool is designed as an educational project to demonstrate regex matching algorithms and Rust programming concepts.

## Features

- Basic regex pattern matching
- Support for the following regex features:
  - `^` for start of string anchor
  - `$` for end of string anchor
  - `.` for any character
  - `*` for zero or more repetitions of the previous character
- Command-line interface similar to grep

## Installation

To use this tool, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once Rust is installed, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-grep-tool.git
   cd rust-grep-tool
   ```

2. Build the project:
   ```
   cargo build --release
   ```

The executable will be available in the `target/release` directory.

## Usage

The basic usage of the tool is as follows:

```
./rust-grep-tool -E <pattern>
```

The tool reads input from stdin and prints whether the input matches the given pattern.

### Examples

1. Search for a simple string:
   ```
   echo "Hello, World" | ./rust-grep-tool -E "World"
   ```

2. Use a start-of-line anchor:
   ```
   echo "Hello, World" | ./rust-grep-tool -E "^Hello"
   ```

3. Use a wildcard and repetition:
   ```
   echo "aaabbb" | ./rust-grep-tool -E "a*b"
   ```

## Limitations

This is a basic implementation and does not support all features of a full-fledged regex engine. It's primarily intended for educational purposes and may not be suitable for complex pattern matching tasks.

## Contributing

Contributions to improve the tool or extend its functionality are welcome. Please feel free to submit issues or pull requests on the project's GitHub page.


## Acknowledgements

This project was inspired by the regex matching algorithm described in "Beautiful Code" by Brian Kernighan, which was originally based on Rob Pike's implementation.
