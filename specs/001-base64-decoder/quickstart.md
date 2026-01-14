# Quickstart: decodeck

**Feature**: 001-base64-decoder
**Date**: 2026-01-14

## Installation

### From Source (Development)

```bash
# Clone repository
git clone https://github.com/your-org/decodeck.git
cd decodeck

# Build release binary
cargo build --release

# Binary location
./target/release/decodeck --version
```

### From Cargo (After Publishing)

```bash
cargo install decodeck
```

## Basic Usage

### Decode Base64 String

```bash
# Simple text
decodeck decode "SGVsbG8gV29ybGQ="
# Output: Hello World

# Image data (will prompt to view)
decodeck decode "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
```

### Decode from File

```bash
# Create a test file
echo "SGVsbG8gV29ybGQ=" > encoded.txt

# Decode it
decodeck decode --file encoded.txt
```

### Decode from Pipe

```bash
# From echo
echo "SGVsbG8gV29ybGQ=" | decodeck decode

# From curl (API response)
curl -s https://api.example.com/data | jq -r '.base64_field' | decodeck decode

# From file
cat encoded.txt | decodeck decode
```

### Save to Specific Location

```bash
# Decode and save to specific file
decodeck decode --output ~/Downloads/decoded.png "iVBORw0KGgo..."

# Force overwrite if exists
decodeck decode --output ~/Downloads/decoded.png --force "iVBORw0KGgo..."
```

## Output Formats

### Human-Readable (Default)

```bash
decodeck decode "SGVsbG8gV29ybGQ="
```

Output:
```
Decoded: /tmp/decodeck-abc123/output.txt
Size: 11 bytes
Type: text/plain
Extension: .txt
Encoding: Standard Base64 (with padding)

Press space to view...
```

### JSON Format

```bash
decodeck decode --json "SGVsbG8gV29ybGQ="
```

Output:
```json
{
  "success": true,
  "output": {
    "path": "/tmp/decodeck-abc123/output.txt",
    "is_temporary": true,
    "size_bytes": 11,
    "size_formatted": "11 bytes"
  },
  "metadata": {
    "mime_type": "text/plain",
    "extension": ".txt",
    "category": "document",
    "is_viewable": true,
    "is_playable": false
  }
}
```

## Common Workflows

### API Response Processing

```bash
# Extract and decode base64 field from JSON response
curl -s https://api.example.com/document | \
  jq -r '.content' | \
  decodeck decode --output document.pdf
```

### Batch Processing

```bash
# Decode multiple files
for file in *.b64; do
  decodeck decode --file "$file" --output "${file%.b64}" --quiet
done
```

### Script Integration

```bash
# Use JSON output for scripting
result=$(decodeck decode --json "SGVsbG8=")
path=$(echo "$result" | jq -r '.output.path')
mime=$(echo "$result" | jq -r '.metadata.mime_type')
echo "Decoded $mime file to: $path"
```

### Debug Mode

```bash
# Verbose output for troubleshooting
decodeck decode --verbose "SGVsbG8gV29ybGQ="
```

## Handling Different Encodings

### Standard Base64

```bash
# Characters: A-Za-z0-9+/
decodeck decode "SGVsbG8rV29ybGQ="
```

### URL-Safe Base64

```bash
# Characters: A-Za-z0-9-_ (auto-detected)
decodeck decode "SGVsbG8tV29ybGQ_"
```

### With/Without Padding

```bash
# Both work automatically
decodeck decode "SGVsbG8="    # with padding
decodeck decode "SGVsbG8"     # without padding (auto-corrected)
```

### With Whitespace

```bash
# Whitespace is automatically stripped
decodeck decode "SGVs bG8g V29y bGQ="
```

## Error Handling

### Invalid Base64

```bash
decodeck decode "Invalid$Base64!"
# Error: Invalid character at position 7: '$'
```

### File Not Found

```bash
decodeck decode --file nonexistent.txt
# Error: File not found: nonexistent.txt
```

### Size Limit

```bash
# Default limit is 100MB
decodeck decode --file huge.b64
# Error: Input size (150 MB) exceeds limit (100 MB)

# Increase limit if needed
decodeck decode --file huge.b64 --max-size 200MB
```

## Tips

1. **Non-interactive scripts**: Use `--quiet` to skip the "Press space" prompt
2. **JSON parsing**: Use `--json` with `jq` for reliable parsing
3. **Large files**: The tool processes in streaming mode automatically
4. **Temporary files**: Default output goes to system temp directory
5. **URL-safe detection**: No flag needed, format is auto-detected
