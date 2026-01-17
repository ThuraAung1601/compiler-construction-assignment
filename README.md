# Scanner and Parser

## Stuent Information
- **Name:** Thura Aung  
- **Student ID:** 66011606  
- **Email:** 66011606@kmitl.ac.th  

## Description
This program is developed for **Phase 1 of the Programming Assignment** in the **Compiler Construction** course.

The scanner reads expressions from an input file and generates a tokenized output file.  
Each line is processed independently, and tokens are identified according to the defined lexical grammar.

The project is implemented in **Rust**, as Rust was taught during the first-year, first-semester course **“Elementary System Programming”**, and this project serves as an opportunity to apply the acquired knowledge.

## Prerequisites

### Linux / macOS
Install Rust using:
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
````

### Windows

Install Rust from:

```
https://rust-lang.github.io/rustup/installation/windows-msvc.html
```

## Compilation

1. Navigate to the project directory
2. Build the project in release mode:

```bash
cargo build --release
```

## Execution

Run the program using:

```bash
cargo run --release <input_file> <output_file>
```

### Example

```bash
cargo run --release input.txt output.tok
```

## References

1. Dave MacLeod, *Easy Rust*, GitHub Pages
   [https://dhghomon.github.io/easy_rust](https://dhghomon.github.io/easy_rust)
2. Ken Youens-Clark, *Command-Line Rust*, O’Reilly, 2022
   ISBN: 9781098109417
3. Clement Tsang, *A (toy) C compiler written in Rust with no dependencies*
   [https://github.com/ClementTsang/rustcc](https://github.com/ClementTsang/rustcc)

```
