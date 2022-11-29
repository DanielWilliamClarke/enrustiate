# Enrustiate

Just a quick challenge to myself, to see if I could implement a numbers to words CLI tool in Rust in an evening.

## Challenge aims

- Robust input validation
- Render numbers up to a quadrillion in words
- Implement the algorithm in the most elegant way possible
- Tests!!

```bash
# Build
cargo build

# Run 
# pass a value up to one quadrillion as the first cli argument
cargo run -- 999999999999999
# Output: nine hundred and ninety nine trillion nine hundred and ninety nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine

# Test
cargo test
```
