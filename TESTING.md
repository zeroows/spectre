# Testing Guide for Spectre CLI

## Quick Start

### Run All Tests
```bash
cargo test
```

### Run Integration Tests with Output
```bash
cargo test --test integration_tests -- --nocapture
```

### Run CSV-Based Tests
```bash
cargo test --test csv_tests -- --nocapture
```

### Test a Single Password
```bash
./target/release/spectre-cli -S "test" -u "test" -q masterpasswordapp.com
```

## Test Structure

```
tests/
├── integration_tests.rs   # Programmatic tests
├── csv_tests.rs           # CSV-driven tests
├── test_vectors.csv       # Test data
└── README.md              # Detailed test documentation
```

## Adding Your Own Test Cases

### Method 1: Add to CSV File

1. Edit `tests/test_vectors.csv`:
   ```csv
   myname,mysecret,example.com,long,1,3,
   ```

2. Run tests to see output:
   ```bash
   cargo test --test csv_tests -- --nocapture
   ```

3. Verify at https://spectre.app and update CSV with expected value

### Method 2: Use the Helper Script

```bash
./scripts/test_helper.sh
```

This will test common cases and show you what to verify on the official site.

## Verifying Against Official Spectre

Always verify critical test cases against the official implementation:

1. Visit: https://spectre.app
2. Enter test data (user, secret, site)
3. Compare password output
4. Update test expectations

## Current Test Results

Running the CSV tests will show:
- ✓ **Passed**: Output matches expected value
- ✗ **Failed**: Output doesn't match expected  value
- ℹ **Manual**: No expected value, needs verification

Example output:
```
✓ Line 2: test @ masterpasswordapp.com = DoloZanoKipu3_
ℹ Line 7: Alice @ example.com = PopoKeseCeyu8( (no expected value)
```

## Test Coverage

### Password Types
- [x] Long (default)
- [x] Medium
- [x] Short
- [x] Basic
- [x] Maximum
- [x] PIN
- [x] Name
- [x] Phrase

### Features
- [x] User key derivation
- [x] Site-specific passwords
- [x] Counter support (password rotation)
- [x] Multiple algorithm versions (0-3)
- [x] Different key purposes (auth, ident, recovery)
- [x] Identicon generation

## Troubleshooting

### Test fails with "assertion failed"
The password doesn't match the expected value. Verify against https://spectre.app.

### "Failed to derive user key"
Check that scrypt is working properly. This is a cryptographic operation that takes several seconds.

### Passwords don't match official site
The character order in template classes might need adjustment. Check `src/types.rs`.

## Performance

Key derivation (scrypt) is intentionally slow for security:
- Each test case takes ~4-5 seconds
- Total test suite: ~1-2 minutes
- Use `--test-threads=1` to avoid overwhelming CPU

## Need Help?

See `tests/README.md` for detailed documentation on:
- Test file formats
- Adding test cases
- Symbol character sets
- Debugging tips

