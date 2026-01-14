# Research: Decodificador Base64

**Feature**: 001-base64-decoder
**Date**: 2026-01-14
**Status**: Complete

## Technology Decisions

### 1. Lenguaje: Rust

**Decision**: Rust 1.75+ (edition 2021)

**Rationale**:
- Rendimiento nativo sin runtime overhead
- Memory safety sin garbage collector
- Excelente ecosistema para CLIs (clap, crossterm)
- Cross-compilation nativa para macOS/Linux/Windows
- Manejo seguro de datos binarios

**Alternatives Considered**:
- **Go**: Más simple pero menos control sobre memoria, GC puede afectar latencia
- **Python**: Desarrollo rápido pero requiere runtime, más lento en procesamiento binario
- **TypeScript/Node**: Requiere Node instalado, no ideal para distribución standalone

### 2. CLI Framework: clap

**Decision**: clap v4.x con derive macros

**Rationale**:
- Estándar de facto para CLIs en Rust
- Derive macros reducen boilerplate
- Soporte completo para subcommands, flags, completions
- Generación automática de help y version

**Alternatives Considered**:
- **structopt**: Deprecado, migrado a clap
- **argh**: Más simple pero menos features
- **manual parsing**: Más control pero mucho más código

### 3. Base64 Decoding: base64 crate

**Decision**: base64 crate v0.21+

**Rationale**:
- Implementación RFC 4648 completa
- Soporte para standard y URL-safe alphabets
- API simple y bien documentada
- Mantenido activamente

**Alternatives Considered**:
- **data-encoding**: Más general pero API más verbosa
- **std library**: Rust std no incluye Base64
- **manual implementation**: Innecesario, crate es confiable

### 4. MIME Detection: infer crate

**Decision**: infer crate v0.15+

**Rationale**:
- Detección por magic bytes (no por extensión)
- Soporte para 100+ formatos comunes
- Zero dependencies
- API simple: `infer::get(&bytes)`

**Alternatives Considered**:
- **tree_magic**: Más completo pero más pesado, usa libmagic
- **mime_guess**: Solo por extensión, no por contenido
- **file-format**: Menos formatos soportados

### 5. Terminal Interaction: crossterm

**Decision**: crossterm v0.27+

**Rationale**:
- Cross-platform (macOS, Linux, Windows)
- Raw mode para captura de teclas
- No requiere ncurses/terminfo
- API async-friendly

**Alternatives Considered**:
- **termion**: Solo Unix, no Windows
- **console**: Menos control sobre raw input
- **dialoguer**: Demasiado para nuestro caso simple

### 6. JSON Serialization: serde + serde_json

**Decision**: serde v1.x + serde_json v1.x

**Rationale**:
- Estándar de facto en Rust
- Derive macros para serialización automática
- Rendimiento excelente
- Soporte completo para JSON

**Alternatives Considered**:
- **simd-json**: Más rápido pero más complejo
- **json crate**: Menos ergonómico
- **manual formatting**: Propenso a errores

### 7. Error Handling: thiserror + anyhow

**Decision**: thiserror para library errors, anyhow para CLI

**Rationale**:
- thiserror: Define error types con derive macros
- anyhow: Propagación simple de errores en binario
- Separación clara entre errores de lib y CLI

**Alternatives Considered**:
- **eyre**: Similar a anyhow, menos popular
- **snafu**: Más verbose
- **std Error trait only**: Más boilerplate

### 8. Logging: tracing

**Decision**: tracing crate para structured logging

**Rationale**:
- Structured logging moderno
- Niveles configurables (debug, info, warn, error)
- Integración con observabilidad tools
- Soporte para spans (útil para debugging)

**Alternatives Considered**:
- **log + env_logger**: Más simple pero menos features
- **slog**: Más complejo de configurar
- **println debugging**: No escalable

## Dependency Summary

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
base64 = "0.21"
infer = "0.15"
crossterm = "0.27"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
proptest = "1.4"
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.9"
```

## Platform-Specific Considerations

### File Opening (FR-018)

| Platform | Command | Notes |
|----------|---------|-------|
| macOS | `open <path>` | Uses default app |
| Linux | `xdg-open <path>` | Freedesktop standard |
| Windows | `start "" "<path>"` | Shell builtin |

**Implementation**: Use `opener` crate que abstrae estas diferencias.

### Stdin Detection (FR-019)

```rust
// Detectar si stdin es interactivo o pipe
use std::io::IsTerminal;
let is_interactive = std::io::stdin().is_terminal();
```

### Temporary Directory (FR-014)

```rust
use std::env::temp_dir;
// Retorna platform-appropriate temp dir
// macOS: /var/folders/...
// Linux: /tmp
// Windows: %TEMP%
```

## Security Considerations

### Path Traversal Prevention (FR-022)

```rust
// Validar que la ruta no contiene ../ ni es absoluta inesperada
fn sanitize_path(path: &Path) -> Result<PathBuf> {
    let canonical = path.canonicalize()?;
    // Verificar que está dentro del directorio permitido
}
```

### Size Limits (FR-021)

- Default: 100MB máximo de input
- Configurable via `--max-size`
- Validar ANTES de intentar decodificar

### No Auto-Execution (FR-023)

- Abrir archivo SOLO con acción explícita (espacio)
- Usar aplicación del sistema, no ejecutar directamente
- Log de operaciones de apertura
