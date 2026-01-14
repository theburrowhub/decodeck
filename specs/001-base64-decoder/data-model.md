# Data Model: Decodificador Base64

**Feature**: 001-base64-decoder
**Date**: 2026-01-14

## Entities

### InputSource

Representa la fuente de datos Base64 de entrada.

```text
InputSource
├── source_type: SourceType     # Arg | File | Stdin
├── raw_data: Vec<u8>           # Contenido raw (antes de limpieza)
├── path: Option<PathBuf>       # Ruta si source_type == File
└── size_bytes: usize           # Tamaño de entrada
```

**SourceType Enum**:
- `Arg`: Datos pasados como argumento CLI
- `File`: Datos leídos de archivo especificado
- `Stdin`: Datos leídos de stdin (pipe)

**Validation Rules**:
- `size_bytes` <= MAX_SIZE (default 100MB)
- Si `source_type == File`, `path` MUST be Some
- Si `source_type == Stdin`, verificar is_terminal() == false

### EncodedData

Representa datos Base64 después de parsing y normalización.

```text
EncodedData
├── data: String                # Base64 limpio (sin whitespace)
├── variant: Base64Variant      # Standard | UrlSafe
├── has_padding: bool           # True si termina en '='
├── original_length: usize      # Longitud antes de limpieza
└── source: InputSource         # Referencia a fuente original
```

**Base64Variant Enum**:
- `Standard`: Alphabet A-Za-z0-9+/ (RFC 4648)
- `UrlSafe`: Alphabet A-Za-z0-9-_ (RFC 4648 §5)

**Validation Rules**:
- `data` solo contiene caracteres válidos del alphabet
- Si padding requerido y falta, se autocompleta
- Longitud de `data` debe ser múltiplo de 4 (después de padding)

### DecodedContent

Representa el resultado de la decodificación.

```text
DecodedContent
├── bytes: Vec<u8>              # Contenido decodificado
├── size_bytes: usize           # Tamaño en bytes
├── checksum: String            # SHA256 para verificación (opcional)
└── source: EncodedData         # Referencia a datos codificados
```

**Derived Properties**:
- `size_formatted`: String con formato human-readable (KB, MB, etc.)

### ContentMetadata

Información extraída del contenido decodificado.

```text
ContentMetadata
├── mime_type: String           # e.g., "image/png"
├── extension: String           # e.g., ".png"
├── category: ContentCategory   # Image | Document | Audio | Video | Other
├── is_viewable: bool           # True para imágenes, PDFs, etc.
├── is_playable: bool           # True para audio, video
└── magic_bytes: Option<[u8;8]> # Primeros bytes para debug
```

**ContentCategory Enum**:
- `Image`: PNG, JPEG, GIF, WebP, SVG, etc.
- `Document`: PDF, DOC, DOCX, TXT, etc.
- `Audio`: MP3, WAV, FLAC, OGG, etc.
- `Video`: MP4, MKV, AVI, WebM, etc.
- `Archive`: ZIP, TAR, GZ, etc.
- `Other`: Tipo desconocido

**MIME to Category Mapping**:
```text
image/*     → Image    (viewable)
video/*     → Video    (playable)
audio/*     → Audio    (playable)
application/pdf → Document (viewable)
text/*      → Document (viewable)
application/zip → Archive (neither)
*           → Other    (neither)
```

### OutputFile

Representa el archivo de salida generado.

```text
OutputFile
├── path: PathBuf               # Ruta completa al archivo
├── is_temporary: bool          # True si está en temp dir
├── size_bytes: usize           # Tamaño escrito
├── created_at: SystemTime      # Timestamp de creación
└── metadata: ContentMetadata   # Metadatos del contenido
```

**Validation Rules**:
- `path` debe existir después de escritura
- `path` no debe contener path traversal (`..`)
- Si `is_temporary == false`, verificar permisos de escritura

### DecodeResult

Resultado completo de una operación de decodificación.

```text
DecodeResult
├── output: OutputFile          # Archivo generado
├── metadata: ContentMetadata   # Información del contenido
├── encoding: EncodedData       # Info de codificación detectada
├── duration_ms: u64            # Tiempo de procesamiento
└── warnings: Vec<String>       # Advertencias (padding corregido, etc.)
```

## State Transitions

```text
┌─────────────────┐
│   InputSource   │
│  (raw input)    │
└────────┬────────┘
         │ parse + validate
         ▼
┌─────────────────┐
│  EncodedData    │
│ (clean base64)  │
└────────┬────────┘
         │ decode
         ▼
┌─────────────────┐
│ DecodedContent  │
│ (binary bytes)  │
└────────┬────────┘
         │ analyze + write
         ▼
┌─────────────────┐    ┌─────────────────┐
│   OutputFile    │◄───│ContentMetadata  │
│ (saved file)    │    │ (MIME, ext)     │
└────────┬────────┘    └─────────────────┘
         │ combine
         ▼
┌─────────────────┐
│  DecodeResult   │
│ (final output)  │
└─────────────────┘
```

## Output Formats

### Text Format (default)

```text
Decoded: /tmp/decodeck-abc123/output.png
Size: 1.2 MB (1,258,291 bytes)
Type: image/png
Extension: .png
Encoding: Standard Base64 (with padding)

Press space to view...
```

### JSON Format (--json)

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

### Error Format (JSON)

```json
{
  "success": false,
  "error": {
    "code": "INVALID_BASE64",
    "message": "Invalid character at position 42: '$'",
    "details": {
      "position": 42,
      "character": "$",
      "expected": "A-Za-z0-9+/="
    }
  }
}
```
