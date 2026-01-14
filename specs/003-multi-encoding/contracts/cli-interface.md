# CLI Interface Contract: Multi-Encoding decode

## Command Signature

```
decodeck decode [OPTIONS] [DATA]
```

## New Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| --encoding | -e | enum | auto | Encoding format: base64, hex, base32, url, base85 |

## Encoding Values

| Value | Aliases | Description |
|-------|---------|-------------|
| base64 | b64 | Standard or URL-safe Base64 (RFC 4648) |
| hex | hexadecimal | Hexadecimal (0-9, a-f, A-F) |
| base32 | b32 | Base32 (RFC 4648) |
| url | percent, urlencode | URL percent-encoding (RFC 3986) |
| base85 | ascii85, a85 | Ascii85 encoding |

## Auto-Detection Behavior

When `--encoding` is not specified:

1. Input starts with `0x` → hex
2. Input wrapped in `<~` and `~>` → base85
3. Input contains `%XX` patterns → url
4. Input is valid hex (only 0-9a-fA-F, even length) → hex
5. Input is valid Base32 (only A-Z2-7=) → base32
6. Otherwise → base64 (current behavior)

## Output Changes

### Text Output

```
Decoded: /tmp/decodeck/output.bin
Size: 1.2 KB
Encoding: hex (auto-detected)    # NEW LINE
MIME: application/octet-stream
```

### JSON Output

```json
{
  "success": true,
  "output": { ... },
  "metadata": { ... },
  "encoding": {                  // NEW FIELD
    "type": "hex",
    "detected": true,
    "confidence": "high"
  },
  "duration_ms": 5
}
```

## Examples

```bash
# Decode hexadecimal
decodeck decode --encoding hex "48656c6c6f"
decodeck decode -e hex "0x48656c6c6f"

# Decode Base32
decodeck decode --encoding base32 "JBSWY3DPEHPK3PXP"

# Decode URL-encoded
decodeck decode --encoding url "Hello%20World%21"

# Decode Ascii85
decodeck decode --encoding base85 "<~87cURD]j7BEbo80~>"

# Auto-detect (hex with 0x prefix)
decodeck decode "0x48656c6c6f"

# Explicit Base64 (override auto-detection)
decodeck decode --encoding base64 "SGVsbG8="
```

## Error Messages

### Invalid Encoding Specified

```
error: invalid value 'invalid' for '--encoding <ENCODING>'
  [possible values: base64, hex, base32, url, base85]
```

### Invalid Data for Encoding

```
Error: Invalid hex data at position 5: character 'g' is not valid hexadecimal
Error: Invalid Base32 data: unexpected character '8'
Error: Invalid URL encoding: incomplete percent sequence at position 10
```

## Help Text

```
decodeck decode [OPTIONS] [DATA]

Arguments:
  [DATA]  Encoded data to decode (or use --file/-f)

Options:
  -e, --encoding <ENCODING>  Encoding format [default: auto]
                             [possible values: base64, hex, base32, url, base85]
  -f, --file <FILE>          Read encoded data from file
  -o, --output <OUTPUT>      Output file path
  -j, --json                 Output in JSON format
  -F, --force                Force overwrite existing files
      --max-size <SIZE>      Maximum input size [default: 100MB]
      --no-interactive       Skip interactive prompt
  -h, --help                 Print help

Encoding Aliases:
  base64:  b64
  hex:     hexadecimal
  base32:  b32
  url:     percent, urlencode
  base85:  ascii85, a85
```

## Backwards Compatibility

- Default behavior unchanged: no `--encoding` flag = Base64 assumed
- Existing scripts continue to work without modification
- Auto-detection is conservative: ambiguous cases default to Base64
