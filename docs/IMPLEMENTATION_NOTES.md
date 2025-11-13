# Spectre Implementation Notes

## Verified Against Official Implementation

This Rust implementation has been verified against the [official Spectre JavaScript implementation](https://gitlab.com/spectre.app/www/-/raw/main/assets/js/mpw-js/mpw.js?ref_type=heads) from the Spectre GitLab repository.

### Test Results

All test vectors from the official Spectre site (https://spectre.app) **pass** ✅:

| User | Secret | Site | Expected | Result |
|------|--------|------|----------|--------|
| test | test | masterpasswordapp.com | `DolsZanoKipu3_` | ✅ Pass |
| Robert Lee Mitchell | banana colored duckling | masterpasswordapp.com | `Jejr5[RepuSosp` | ✅ Pass |
| test | test | example.com | `JivrYeloQasg8[` | ✅ Pass |
| test | test | github.com | `NochJefi8+Jupl` | ✅ Pass |
| test | test | google.com | `PetsPibs8=Tuci` | ✅ Pass |

## Key Implementation Details

### Character Classes (from official implementation)

```javascript
V: "AEIOU"
C: "BCDFGHJKLMNPQRSTVWXYZ"
v: "aeiou"
c: "bcdfghjklmnpqrstvwxyz"
A: "AEIOUBCDFGHJKLMNPQRSTVWXYZ"
a: "AEIOUaeiouBCDFGHJKLMNPQRSTVWXYZbcdfghjklmnpqrstvwxyz"
n: "0123456789"
o: "@&%?,=[]_:-+*$#!'^~;()/."
x: "AEIOUaeiouBCDFGHJKLMNPQRSTVWXYZbcdfghjklmnpqrstvwxyz0123456789!@#$%^&*()"
' ': " "
```

**Important**: The order of characters matters! The symbol class `o` must be exactly: `@&%?,=[]_:-+*$#!'^~;()/.`

### Password Templates

#### Long Password (21 templates)
The long password type has **21 different templates**, not just 3. This was the key issue that was causing mismatches:

```rust
"CvcvnoCvcvCvcv",
"CvcvCvcvnoCvcv",
"CvcvCvcvCvcvno",
"CvccnoCvcvCvcv",
"CvccCvcvnoCvcv",
"CvccCvcvCvcvno",
"CvcvnoCvccCvcv",
"CvcvCvccnoCvcv",
"CvcvCvccCvcvno",
"CvcvnoCvcvCvcc",
"CvcvCvcvnoCvcc",
"CvcvCvcvCvccno",
"CvccnoCvccCvcv",
"CvccCvccnoCvcv",
"CvccCvccCvcvno",
"CvcvnoCvccCvcc",
"CvcvCvccnoCvcc",
"CvcvCvccCvccno",
"CvccnoCvcvCvcc",
"CvccCvcvnoCvcc",
"CvccCvcvCvccno",
```

The first byte of the site key (after HMAC-SHA256) selects which template to use: `template_index = siteKey[0] % templates.length`

### Algorithm Parameters

- **scrypt parameters**: N=32768, r=8, p=2, dkLen=64
- **HMAC**: SHA-256
- **Key scope for authentication**: `"com.lyndir.masterpassword"`
- **Key scope for identification**: `"com.lyndir.masterpassword.login"`
- **Key scope for recovery**: `"com.lyndir.masterpassword.answer"`

### Salt Construction

#### User Key Salt:
```
scope | #userName | userName
```
Where `#userName` is a 4-byte big-endian integer representing the byte length of userName.

**Note**: Versions 0-2 incorrectly used character length instead of byte length.

#### Site Key Salt:
```
keyScope | #siteName | siteName | keyCounter | [#keyContext | keyContext]
```

Where:
- `#siteName` is a 4-byte big-endian integer (byte length for v2+, character length for v0-1)
- `keyCounter` is a 4-byte big-endian signed integer
- `keyContext` is optional

## Compatibility

This implementation is compatible with:
- ✅ Spectre algorithm version 3 (current)
- ✅ Official Spectre web app (https://spectre.app)
- ✅ Official JavaScript implementation
- ✅ All password types (long, medium, short, basic, maximum, PIN, name, phrase)

## References

- Official Spectre Site: https://spectre.app
- GitLab Repository: https://gitlab.com/spectre.app/api
- JavaScript Implementation: https://gitlab.com/spectre.app/www/-/raw/main/assets/js/mpw-js/mpw.js
- Algorithm Documentation: Based on MasterPassword algorithm v3

## Testing

Run the test suite:

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific tests
cargo test --test integration_tests
cargo test --test csv_tests

# Against official test vectors
cargo test --test csv_tests -- --nocapture
```

All tests pass and match the official implementation! 🎉

