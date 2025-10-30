# Spectre CLI Tests

This directory contains comprehensive tests for the Spectre CLI implementation.

## Test Files

### 1. `integration_tests.rs`
Integration tests covering various scenarios:
- Official Spectre examples
- Different password types (long, medium, short, PIN, name, phrase)
- Counter variation (password rotation)
- Site variation
- Identicon generation
- Key purpose variation (auth, ident, recovery)
- Algorithm version compatibility

Run with:
```bash
cargo test --test integration_tests -- --nocapture
```

### 2. `csv_tests.rs`
CSV-based test suite that reads test vectors from `test_vectors.csv`.

Run with:
```bash
cargo test --test csv_tests -- --nocapture
```

### 3. `test_vectors.csv`
CSV file containing test cases in the format:
```
username,secret,site,type,counter,algorithm,expected
```

- **username**: Full name of the user
- **secret**: Personal secret (master password)
- **site**: Site domain name
- **type**: Password type (long, medium, short, basic, maximum, pin, name, phrase)
- **counter**: Counter value (usually 1)
- **algorithm**: Algorithm version (0-3, current is 3)
- **expected**: Expected password output (leave empty to see generated output)

## How to Use

### Adding New Test Cases

1. **Add to CSV file**:
   Edit `test_vectors.csv` and add a new line:
   ```
   myname,mysecret,example.com,long,1,3,
   ```
   Leave the `expected` field empty initially.

2. **Run tests to see output**:
   ```bash
   cargo test --test csv_tests -- --nocapture
   ```
   
3. **Verify against official Spectre**:
   - Go to https://spectre.app
   - Enter the same username, secret, and site
   - Compare the output

4. **Update expected value**:
   Once verified, add the expected password to the CSV:
   ```
   myname,mysecret,example.com,long,1,3,YourExpectedPassword
   ```

### Testing a Single Password

Use the CLI directly:
```bash
./target/release/spectre-cli -S "secret" -u "username" -q sitename.com
```

Add `-v` for verbose output with identicon:
```bash
./target/release/spectre-cli -S "secret" -u "username" sitename.com
```

### Verifying Against Official Implementation

To ensure compatibility with the official Spectre implementation:

1. Visit https://spectre.app
2. Enter your test data:
   - Full Name: `test`
   - Personal Secret: `test`
   - Site Domain: `masterpasswordapp.com`
3. Compare the generated password with our output

## Current Status

### ✅ Working Features
- User key derivation (scrypt)
- Site key derivation (HMAC-SHA256)
- All password types generation
- Counter support
- Multiple algorithm versions
- Identicon generation
- CSV test framework

### ⚠️ Known Issues
- Some symbol characters may differ from official implementation
- Character order in symbol set needs verification against official C implementation

## Symbol Character Set

The official Spectre algorithm uses this symbol set for the 'o' template character:
```
@&%?,=[]_:-+*$#!'^~;()/.
```

Order matters! The exact sequence affects which symbol is selected for each position.

## Running All Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test file
cargo test --test integration_tests
cargo test --test csv_tests

# Run specific test
cargo test test_official_spectre_examples -- --nocapture
```

## Debugging Tips

1. **Enable verbose output**: Add `-- --nocapture` to see println! output
2. **Test one case**: Add `#[ignore]` to other tests
3. **Use CLI directly**: Faster iteration than running full test suite
4. **Compare with official**: Always verify critical test cases at https://spectre.app

## Contributing Test Cases

When adding test cases:
1. Use diverse combinations of usernames and secrets
2. Test different password types
3. Test different counter values
4. Include both common and edge case site names
5. Always verify against the official implementation before committing expected values

