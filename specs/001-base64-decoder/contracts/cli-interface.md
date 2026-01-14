# CLI Interface Contract: decodeck

**Version**: 0.1.0
**Date**: 2026-01-14

## Command Structure

```text
decodeck [OPTIONS] <COMMAND>

Commands:
  decode    Decode Base64 data to file
  encode    Encode file to Base64 (future)
  help      Print help information

Global Options:
  -h, --help     Print help
  -V, --version  Print version
  -v, --verbose  Enable verbose output
  -q, --quiet    Suppress non-essential output
```

## decode Command

### Synopsis

```text
decodeck decode [OPTIONS] [DATA]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `DATA` | No* | Base64 encoded string to decode |

*Required if `--file` not provided and stdin is empty.

### Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--file` | `-f` | PATH | - | Read Base64 from file |
| `--output` | `-o` | PATH | temp | Output file path |
| `--json` | `-j` | flag | false | Output in JSON format |
| `--quiet` | `-q` | flag | false | Suppress prompts |
| `--no-interactive` | - | flag | false | Skip view/play prompt |
| `--force` | `-F` | flag | false | Overwrite existing files |
| `--max-size` | - | SIZE | 100MB | Maximum input size |
| `--verbose` | `-v` | flag | false | Detailed output |

### Input Priority

1. `DATA` argument (if provided)
2. `--file` option (if provided)
3. stdin (if not a terminal)
4. Error: "No input data provided"

### Exit Codes

| Code | Meaning | Example |
|------|---------|---------|
| 0 | Success | Decode completed |
| 1 | User error | Invalid Base64, file not found |
| 2 | System error | Permission denied, disk full |

### Examples

```bash
# Decode from argument
decodeck decode "SGVsbG8gV29ybGQ="

# Decode from file
decodeck decode --file encoded.txt

# Decode from pipe
cat encoded.txt | decodeck decode

# Decode with specific output
decodeck decode -o output.png "iVBORw0KGgo..."

# Decode with JSON output
decodeck decode --json "SGVsbG8="

# Decode without interactive prompt
decodeck decode --quiet "SGVsbG8="
```

## Output Contracts

### Success Output (text)

```text
Decoded: <output_path>
Size: <size_formatted> (<size_bytes> bytes)
Type: <mime_type>
Extension: <extension>
Encoding: <variant> Base64 (<padding_status>)

Press space to view|play...
```

**Fields**:
- `output_path`: Absolute path to decoded file
- `size_formatted`: e.g., "1.2 MB", "456 KB", "123 bytes"
- `size_bytes`: Integer byte count
- `mime_type`: e.g., "image/png", "application/pdf"
- `extension`: e.g., ".png", ".pdf", ".bin"
- `variant`: "Standard" or "URL-safe"
- `padding_status`: "with padding" or "without padding (added)"

### Success Output (JSON)

```json
{
  "success": true,
  "output": {
    "path": "/tmp/decodeck-abc123/output.png",
    "is_temporary": true,
    "size_bytes": 1258291,
    "size_formatted": "1.2 MB"
  },
  "metadata": {
    "mime_type": "image/png",
    "extension": ".png",
    "category": "image",
    "is_viewable": true,
    "is_playable": false
  },
  "encoding": {
    "variant": "standard",
    "had_padding": true
  },
  "duration_ms": 45,
  "warnings": []
}
```

### Error Output (text)

```text
Error: <error_message>

<context_if_available>

Run 'decodeck decode --help' for usage information.
```

### Error Output (JSON)

```json
{
  "success": false,
  "error": {
    "code": "<ERROR_CODE>",
    "message": "<human_readable_message>",
    "details": {}
  }
}
```

**Error Codes**:

| Code | Description |
|------|-------------|
| `NO_INPUT` | No input data provided |
| `INVALID_BASE64` | Input contains invalid Base64 characters |
| `FILE_NOT_FOUND` | Specified input file does not exist |
| `PERMISSION_DENIED` | Cannot read input or write output |
| `SIZE_EXCEEDED` | Input exceeds maximum size limit |
| `OUTPUT_EXISTS` | Output file exists and --force not set |
| `INVALID_PATH` | Output path is invalid or not writable |
| `DECODE_FAILED` | Base64 decoding failed |
| `SYSTEM_ERROR` | Unexpected system error |

## Interactive Behavior

### Terminal Detection

```text
if stdin.is_terminal() && !--file && !DATA:
    ERROR: "No input data provided"

if stdout.is_terminal() && !--quiet && !--json:
    SHOW: "Press space to view|play..."
else:
    SKIP: interactive prompt
```

### Prompt Behavior

| Condition | Prompt | Action on Space |
|-----------|--------|-----------------|
| is_viewable | "Press space to view..." | Open with viewer |
| is_playable | "Press space to play..." | Open with player |
| neither | (no prompt) | Exit immediately |

| Key | Action |
|-----|--------|
| Space | Open file with native app |
| Enter | Exit without opening |
| Any other | Exit without opening |
| Ctrl+C | Exit without opening |

## Verbose Output

With `--verbose` flag, additional information is printed:

```text
[DEBUG] Input source: argument (42 bytes)
[DEBUG] Detected encoding: Standard Base64
[DEBUG] Padding status: present (2 chars)
[DEBUG] Cleaned input: removed 3 whitespace chars
[DEBUG] Decoding... done (0.002s)
[DEBUG] Output size: 30 bytes
[DEBUG] Detecting MIME type...
[DEBUG] Magic bytes: 89 50 4E 47 0D 0A 1A 0A
[DEBUG] Detected: image/png
[DEBUG] Writing to: /tmp/decodeck-abc123/output.png
[DEBUG] Write complete (0.001s)

Decoded: /tmp/decodeck-abc123/output.png
...
```
