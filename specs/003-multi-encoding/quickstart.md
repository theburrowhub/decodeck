# Quickstart: Multi-Encoding Support

## Basic Usage

### Hexadecimal

```bash
# Decode hex string
decodeck decode --encoding hex "48656c6c6f"
# Output: Hello

# With 0x prefix (auto-detected)
decodeck decode "0x48656c6c6f"
# Output: Hello

# With spaces (ignored)
decodeck decode -e hex "48 65 6c 6c 6f"
# Output: Hello

# Case insensitive
decodeck decode -e hex "48656C6C6F"
# Output: Hello
```

### Base32

```bash
# Decode Base32 (common for TOTP secrets)
decodeck decode --encoding base32 "JBSWY3DPEHPK3PXP"

# Case insensitive
decodeck decode -e base32 "jbswy3dpehpk3pxp"

# Without padding (auto-added)
decodeck decode -e base32 "JBSWY3DPEHPK3PXP"
```

### URL Encoding

```bash
# Decode percent-encoded string
decodeck decode --encoding url "Hello%20World%21"
# Output: Hello World!

# With + for spaces (form data style)
decodeck decode -e url "Hello+World"
# Output: Hello World

# UTF-8 encoded characters
decodeck decode -e url "%C3%A1%C3%A9%C3%AD%C3%B3%C3%BA"
# Output: áéíóú
```

### Base85 (Ascii85)

```bash
# Decode Ascii85 with delimiters
decodeck decode --encoding base85 "<~87cURD]j7BEbo80~>"

# Without delimiters
decodeck decode -e base85 "87cURD]j7BEbo80"
```

### Base64 (Explicit)

```bash
# Standard Base64
decodeck decode --encoding base64 "SGVsbG8gV29ybGQh"

# URL-safe Base64 (auto-detected variant)
decodeck decode -e base64 "SGVsbG8tV29ybGRf"
```

## Auto-Detection

```bash
# Hex with 0x prefix - detected as hex
decodeck decode "0x48656c6c6f"

# Ascii85 with delimiters - detected as base85
decodeck decode "<~87cURD]j7BEbo80~>"

# URL with %XX - detected as url
decodeck decode "Hello%20World"

# Ambiguous - defaults to Base64
decodeck decode "SGVsbG8="
```

## JSON Output

```bash
# Get encoding info in JSON
decodeck decode --json --encoding hex "48656c6c6f"
```

Output:
```json
{
  "success": true,
  "output": {
    "path": "/tmp/decodeck/output.bin",
    "size_bytes": 5
  },
  "encoding": {
    "type": "hex",
    "detected": false,
    "confidence": "explicit"
  }
}
```

## Common Use Cases

### Decode TOTP Secret

```bash
# Extract TOTP secret from authenticator URL
echo "JBSWY3DPEHPK3PXP" | decodeck decode -e base32 -o secret.bin
```

### Decode Hex Dump

```bash
# Decode memory dump or packet capture
cat hexdump.txt | decodeck decode -e hex -o binary.bin
```

### Decode URL Parameters

```bash
# Decode query string value
decodeck decode -e url "name%3DJohn%26age%3D30"
# Output: name=John&age=30
```

### Decode PDF Embedded Data

```bash
# Ascii85 is common in PDF streams
decodeck decode -e base85 "<~87cURD]j7BEbo80~>" -o data.bin
```

## Error Handling

```bash
# Invalid hex character
decodeck decode -e hex "48656g6c6f"
# Error: Invalid hex data at position 5: character 'g' is not valid hexadecimal

# Wrong encoding specified
decodeck decode -e base32 "Hello%20World"
# Error: Invalid Base32 data: unexpected character '%'
```

## Tips

1. **Use `--encoding` when you know the format** - faster and avoids ambiguity
2. **Auto-detection is conservative** - defaults to Base64 when unsure
3. **Check JSON output for confidence** - `detected: true` means auto-detected
4. **Hex accepts flexible input** - spaces, 0x prefix, mixed case all work
