# Building the parser.rs file
In order to generate a `parser.rs` file from the `parser.lalrpop` file, you can install lalrpop using cargo install:
```bash
cargo install lalrpop
```

then run the following command, or use the `make-parser.sh` script:
```bash
lalrpop src/parser.lalrpop
```
