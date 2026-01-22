# Spectre Password Manager - Rust Implementation

A Rust implementation of the Spectre password manager CLI, providing stateless password generation based on the Spectre algorithm.

**This is a derivative work based on the original Spectre algorithm by Maarten Billemont.**

- Project: https://spectrs.app

This Rust implementation is also licensed under GPL-3.0 in compliance with the original license terms.

## Overview

Spectre is a stateless password management solution that generates site-specific passwords from:
- Your full name
- Your personal secret (master password)
- The site name

The same inputs always produce the same password, so you never need to store passwords - just remember your personal secret!

## Features

- **Stateless**: No database needed, passwords are generated on-demand
- **Secure**: Uses scrypt for key derivation with strong parameters
- **Flexible**: Multiple password templates (long, medium, short, PIN, phrase, etc.)
- **Portable**: Save user preferences to JSON format
- **Cross-platform**: Works on macOS, Linux, and other Unix-like systems

## Installation

### From Source

```bash
cargo build --release
```

The binary will be at `target/release/spectre-cli`.

## Usage

### Basic Usage

Generate a password for a site:

```bash
spectre-cli -u "Your Full Name" example.com
```

You'll be prompted for your personal secret, and the password will be generated.

### Command-Line Options

```
USAGE:
  spectre-cli [OPTIONS] [SITE_NAME]

ARGUMENTS:
  [SITE_NAME]  Site name for which to generate a password

OPTIONS:
  -u, --user-name <USER_NAME>
          User name (or -U to allow updating personal secret)
          Environment: SPECTRE_USERNAME

  -U, --user-name-update <USER_NAME_UPDATE>
          Allow updating personal secret

  -s, --secret-fd <SECRET_FD>
          Read personal secret from file descriptor

  -S, --secret <SECRET>
          Personal secret (UNSAFE - for testing only)

  -t, --result-type <RESULT_TYPE>
          Password template type [default: long]
          Options:
            x, max, maximum  | 20 characters, contains symbols
            l, long          | 14 characters, symbols (default)
            m, medium        | 8 characters, symbols
            b, basic         | 8 characters, no symbols
            s, short         | 4 characters, no symbols
            i, pin           | 4 numbers
            n, name          | 9 letter name
            p, phrase        | 20 character sentence
            K, key           | encryption key
            P, personal      | saved personal password

  -P, --result-param <RESULT_PARAM>
          Parameter value (login name, key bits, or personal password)

  -c, --counter <COUNTER>
          Counter value [default: 1]

  -a, --algorithm <ALGORITHM>
          Algorithm version (0-3, default: 3)
          Environment: SPECTRE_ALGORITHM

  -p, --purpose <PURPOSE>
          Key purpose [default: auth]
          Options:
            a, auth   | Authentication (password)
            i, ident  | Identification (username)
            r, rec    | Recovery (security answer)

  -C, --context <CONTEXT>
          Purpose-specific context (e.g., security question)

  -f, --format <FORMAT>
          File format (with fallback)
          Environment: SPECTRE_FORMAT
          Options:
            n, none   | No file
            f, flat   | Flat format
            j, json   | JSON format (default)

  -F, --format-fixed <FORMAT_FIXED>
          File format (fixed, no fallback)

  -R, --redacted <REDACTED>
          Save file in redacted format [default: 1]

  -v, --verbose...
          Increase verbosity (can be repeated)

  -q, --quiet...
          Decrease verbosity (can be repeated)

  -n, --no-newline
          Omit trailing newline

  -h, --help
          Print help

  -V, --version
          Print version
```

### Examples

#### Generate a password

```bash
spectre-cli -u "John Doe" example.com
```

#### Generate a shorter password

```bash
spectre-cli -u "John Doe" -t medium example.com
```

#### Generate a PIN

```bash
spectre-cli -u "John Doe" -t pin example.com
```

#### Generate a password with a different counter (for password rotation)

```bash
spectre-cli -u "John Doe" -c 2 example.com
```

#### Generate a username

```bash
spectre-cli -u "John Doe" -p ident example.com
```

#### Generate a security answer

```bash
spectre-cli -u "John Doe" -p rec -C "maiden name" example.com
```

#### Don't save to file

```bash
spectre-cli -u "John Doe" -f none example.com
```

#### Testing (unsafe - don't use in production!)

```bash
spectre-cli -S "my secret" -u "John Doe" example.com
```

## Configuration

User preferences are stored in `~/.spectre.d/username.json` by default.

The JSON format includes:
- User information (name, identicon, key ID)
- Site configurations (result type, counter, usage stats)
- Login information
- Security questions

## Environment Variables

- `SPECTRE_USERNAME`: Default user name
- `SPECTRE_ALGORITHM`: Default algorithm version (0-3)
- `SPECTRE_FORMAT`: Default file format (none/flat/json)

## Security Considerations

1. **Never use `-S` flag in production**: It's only for testing. Always use interactive password prompt or pipe from secure source.
2. **Personal secret**: Choose a strong, memorable personal secret. This is your master password.
3. **Site names**: Use consistent site names (e.g., always use "example.com", not "www.example.com" sometimes).
4. **Redacted mode**: By default, files are saved in redacted format (secrets encrypted). Keep it that way!

## Algorithm Details

Spectre uses:
- **scrypt** for user key derivation with parameters N=32768, r=8, p=2
- **HMAC-SHA256** for site-specific key derivation
- **Template-based generation** for human-friendly passwords

## Compatibility

This implementation aims to be compatible with the original Spectre/MasterPassword algorithm (version 3). Passwords generated with the same inputs should match other Spectre implementations.

## License

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)** or later.

This is a derivative work based on the original Spectre implementation by Maarten Billemont.
- See [LICENSE](LICENSE) for the full GPL-3.0 license text
- See [COPYRIGHT.md](COPYRIGHT.md) for detailed attribution and copyright information

## Credits

**Original Algorithm and Implementation:**
- Created by: **Maarten Billemont**
- Project: https://spectre.app
- Repository: https://gitlab.com/spectre.app/api
- Copyright: © 2011-2017 Maarten Billemont
- License: GPL-3.0

**Rust Implementation:**
- Developed by: **Abdulrhman Alkhodiry**
- Copyright: © 2025
- License: GPL-3.0-or-later

This Rust implementation maintains full compatibility with the original Spectre algorithm while providing the benefits of Rust's memory safety and performance.

