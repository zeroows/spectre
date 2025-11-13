# Spectre CLI Examples

This document provides practical examples for using the Spectre CLI.

## Table of Contents

1. [Basic Password Generation](#basic-password-generation)
2. [Different Password Types](#different-password-types)
3. [Site-Specific Options](#site-specific-options)
4. [Working with Usernames](#working-with-usernames)
5. [Security Questions](#security-questions)
6. [Advanced Usage](#advanced-usage)

## Basic Password Generation

### Interactive Mode (Recommended)

```bash
# You'll be prompted for your personal secret
./spectre-cli -u "Alice Anderson" github.com
```

### With Environment Variable

```bash
export SPECTRE_USERNAME="Alice Anderson"
./spectre-cli github.com
```

## Different Password Types

### Long Password (Default)

14 characters, includes symbols - best for most websites:

```bash
./spectre-cli -u "Alice Anderson" -t long github.com
# Example output: Kuco6*VufiNiri
```

### Maximum Security

20 characters with symbols:

```bash
./spectre-cli -u "Alice Anderson" -t maximum important-site.com
# Example output: vuhd2@Vowi2(XifaNafe
```

### Medium Password

8 characters with symbols - for sites with length restrictions:

```bash
./spectre-cli -u "Alice Anderson" -t medium oldsite.com
# Example output: Qus3*Foj
```

### Basic Password

8 characters without symbols:

```bash
./spectre-cli -u "Alice Anderson" -t basic simplesite.com
# Example output: QixoDuga
```

### Short Password

4 characters, no symbols:

```bash
./spectre-cli -u "Alice Anderson" -t short quicksite.com
# Example output: Woz4
```

### PIN

4 digit numeric PIN:

```bash
./spectre-cli -u "Alice Anderson" -t pin bank.com
# Example output: 2739
```

### Name

9 character pronounceable name:

```bash
./spectre-cli -u "Alice Anderson" -t name forum.com
# Example output: toxdaqaca
```

### Phrase

20+ character passphrase:

```bash
./spectre-cli -u "Alice Anderson" -t phrase encryption.com
# Example output: xoq nuqhexoz ziq hihqo
```

## Site-Specific Options

### Password Rotation with Counter

If you need to change a password (e.g., after a breach), increment the counter:

```bash
# Original password
./spectre-cli -u "Alice Anderson" -c 1 example.com

# Rotated password (when you need to change it)
./spectre-cli -u "Alice Anderson" -c 2 example.com

# Another rotation
./spectre-cli -u "Alice Anderson" -c 3 example.com
```

### Quiet Mode (Just Output Password)

Perfect for piping to clipboard or other tools:

```bash
# macOS - copy to clipboard
./spectre-cli -u "Alice Anderson" -q github.com | pbcopy

# Linux - copy to clipboard (with xclip)
./spectre-cli -u "Alice Anderson" -q github.com | xclip -selection clipboard

# Store in variable
PASSWORD=$(./spectre-cli -u "Alice Anderson" -q github.com)
```

### No Newline (For Scripts)

```bash
./spectre-cli -u "Alice Anderson" -n -q github.com
```

## Working with Usernames

Generate consistent usernames for sites:

```bash
# Generate a username
./spectre-cli -u "Alice Anderson" -p ident twitter.com
# Example output: xohdaqoxo

# Generate username with "name" template
./spectre-cli -u "Alice Anderson" -p ident -t name forum.com
# Example output: daqivakow
```

## Security Questions

Generate answers to security questions:

```bash
# Mother's maiden name
./spectre-cli -u "Alice Anderson" -p rec -C "mother maiden name" bank.com

# Favorite pet
./spectre-cli -u "Alice Anderson" -p rec -C "favorite pet" bank.com

# First car
./spectre-cli -u "Alice Anderson" -p rec -C "first car" insurance.com

# Use phrase type for longer answers
./spectre-cli -u "Alice Anderson" -p rec -t phrase -C "first address" bank.com
```

## Advanced Usage

### Different Algorithm Versions

```bash
# Use algorithm version 2 (for compatibility)
./spectre-cli -u "Alice Anderson" -a 2 oldaccount.com

# Use latest algorithm (version 3, default)
./spectre-cli -u "Alice Anderson" -a 3 newsite.com
```

### File Format Options

```bash
# Don't save to file (ephemeral mode)
./spectre-cli -u "Alice Anderson" -f none github.com

# Use JSON format (default)
./spectre-cli -u "Alice Anderson" -f json github.com

# Use flat format (legacy)
./spectre-cli -u "Alice Anderson" -f flat github.com
```

### Updating Personal Secret

If you need to change your master password:

```bash
# -U allows updating the personal secret
./spectre-cli -U "Alice Anderson" github.com
# You'll be prompted for both old and new personal secrets
```

### Verbose Mode

See what's happening:

```bash
# More verbose
./spectre-cli -u "Alice Anderson" -v github.com

# Very verbose
./spectre-cli -u "Alice Anderson" -vv github.com
```

### Pipe Personal Secret (For Automation - Use Carefully!)

```bash
# From file (secure way)
cat /secure/path/to/secret | ./spectre-cli -u "Alice Anderson" -s 0 github.com

# From password manager (example)
pass show spectre/master | ./spectre-cli -u "Alice Anderson" -s 0 github.com
```

## Practical Workflows

### Daily Usage Workflow

1. **Setup environment:**
```bash
export SPECTRE_USERNAME="Alice Anderson"
alias spectre='./spectre-cli'
```

2. **Generate passwords as needed:**
```bash
spectre github.com | pbcopy
# Paste into website

spectre gmail.com | pbcopy
# Paste into website

spectre bank.com -t pin
# Type the PIN manually
```

### Multiple User Workflow

```bash
# Personal account
./spectre-cli -u "Alice Anderson" github.com

# Work account
./spectre-cli -u "Alice Anderson (Work)" github.com

# Each user gets their own configuration file
```

### Backup and Migration

```bash
# Your user data is stored at:
~/.spectre.d/Alice\ Anderson.json

# Backup
cp ~/.spectre.d/*.json ~/backup/

# Restore
cp ~/backup/*.json ~/.spectre.d/

# The JSON files don't contain secrets (redacted mode),
# just preferences and metadata
```

## Tips and Best Practices

1. **Use descriptive site names**: Be consistent (e.g., always use "github.com" not sometimes "github" or "www.github.com")

2. **Counter for rotations**: Instead of changing your master password, increment the counter when you need a new password for a site

3. **Security questions**: Use `-p rec` with meaningful contexts to generate consistent answers

4. **Don't use `-S` in production**: Always use interactive prompt or secure piping

5. **Backup your user preferences**: While stateless, site preferences (counters, types) are helpful to back up

6. **Set environment variables**: For convenience, set `SPECTRE_USERNAME` in your shell profile

7. **Use quiet mode for automation**: Combine `-q` and `-n` for clean output in scripts

8. **Keep it simple**: The whole point is you only need to remember:
   - Your full name
   - Your personal secret
   - Site names (which you can see in your browser)

