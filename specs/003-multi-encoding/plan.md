# Implementation Plan: Multi-Encoding Support

**Branch**: `003-multi-encoding` | **Date**: 2026-01-14 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-multi-encoding/spec.md`

## Summary

Extender decodeck para soportar múltiples formatos de codificación además de Base64: hexadecimal, Base32, URL encoding y Base85/Ascii85. Incluye flag `--encoding` para especificar el formato y auto-detección cuando no se especifica.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**:
- clap 4.4 (existing) - CLI parsing
- base64 0.22 (existing) - Base64 decoding
- data-encoding (new) - Hex, Base32 encoding
- percent-encoding (new) - URL encoding
- base85 (new) - Ascii85/Z85 decoding

**Storage**: N/A (solo decodificación en memoria)
**Testing**: cargo test + tests de integración CLI
**Target Platform**: macOS + Linux + Windows (cross-platform)
**Project Type**: Single CLI project with library
**Performance Goals**: <10% overhead vs base64-only
**Constraints**: Mantener retrocompatibilidad con uso actual (base64 como default)
**Scale/Scope**: 5 encodings soportados (base64, hex, base32, url, base85)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Biblioteca-First | ✅ | Nuevo módulo `encoding/` con trait común para todos los formatos |
| II. Interfaz CLI Dual | ✅ | Flag --encoding, salida JSON incluye campo "encoding" |
| III. TDD Obligatorio | ✅ | Tests para cada encoding antes de implementar |
| IV. Seguridad Primero | ✅ | Validación de input, límites de tamaño existentes aplican |
| V. Simplicidad (YAGNI) | ✅ | Solo decode (no encode), Z85 opcional |
| VI. Versionado Semántico | ✅ | Nueva feature = MINOR bump (0.2.0) |
| VII. Observabilidad | ✅ | Mostrar encoding detectado/usado en output |

**Gate Status**: ✅ PASSED - No violations

## Project Structure

### Documentation (this feature)

```text
specs/003-multi-encoding/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── quickstart.md        # Usage examples
├── contracts/
│   └── cli-interface.md # CLI contract
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
src/
├── encoding/            # NEW: Multi-encoding support
│   ├── mod.rs           # Encoding trait + enum
│   ├── base64.rs        # Refactored from decoder/
│   ├── hex.rs           # Hexadecimal decoder
│   ├── base32.rs        # Base32 decoder
│   ├── url.rs           # URL percent-encoding decoder
│   ├── base85.rs        # Ascii85/Z85 decoder
│   └── detect.rs        # Auto-detection heuristics
├── decoder/
│   └── mod.rs           # UPDATE: Use encoding module
├── bin/
│   └── decodeck.rs      # UPDATE: Add --encoding flag
└── output/
    └── json.rs          # UPDATE: Add encoding field

tests/
├── encoding/            # NEW: Unit tests per encoding
│   ├── hex_test.rs
│   ├── base32_test.rs
│   ├── url_test.rs
│   └── base85_test.rs
└── integration/
    └── multi_encoding_test.rs  # Integration tests
```

**Structure Decision**: Crear nuevo módulo `encoding/` que abstraiga todos los formatos con un trait común `Decoder`. Refactorizar Base64 existente para usar este trait. Esto sigue Principio I (Biblioteca-First) permitiendo reutilización.

## Architecture Design

### Encoding Trait

```rust
pub trait Decoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeckError>;
    fn name(&self) -> &'static str;
    fn can_decode(&self, input: &str) -> bool; // For auto-detection
}
```

### Encoding Enum

```rust
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum EncodingType {
    Base64,  // default
    Hex,
    Base32,
    Url,
    Base85,
}
```

### Auto-Detection Strategy

1. Si input comienza con `0x` → Hex
2. Si input contiene solo `[0-9a-fA-F]` y longitud par → Hex
3. Si input contiene `%XX` patterns → URL encoding
4. Si input contiene `<~` y `~>` → Base85 (Ascii85)
5. Si input contiene solo `[A-Z2-7=]` → Base32
6. Default → Base64

## Complexity Tracking

> No violations - implementation follows existing patterns

## Dependencies to Add

```toml
[dependencies]
data-encoding = "2.5"      # Hex, Base32
percent-encoding = "2.3"   # URL encoding
base85 = "2.0"             # Ascii85/Z85
```

**Justification**:
- `data-encoding`: Crate estándar de la comunidad Rust para encodings, bien mantenido, sin dependencias transitivas problemáticas
- `percent-encoding`: Crate oficial de servo/rust-url, estándar de facto
- `base85`: Implementación ligera de Ascii85, única opción viable en el ecosistema
