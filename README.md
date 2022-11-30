# Enrustiate

Just a quick challenge to myself, to see if I could implement a numbers to words CLI tool in Rust in an evening.

## Challenge aims

- Robust input validation
- Render numbers up to a quadrillion in words
- Implement the algorithm in the most elegant way possible
- Tests!!

```bash
# I've provided a makefile to simplify building and test for your convenience

# Compile
make build [debug=yes]

# Test
make test

# Bench
make bench

# Generate binary
make install [debug=yes]

# Run 
# pass a value up to one quadrillion as the first cli argument
./bin/numbers_to_words 999999999999999
# Output: nine hundred and ninety nine trillion nine hundred and ninety nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine

# if you want to use cargo directly use:
cargo run -- 999999999999999
# Output: nine hundred and ninety nine trillion nine hundred and ninety nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine
```
