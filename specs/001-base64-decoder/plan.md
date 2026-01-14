# Implementation Plan: Decodificador Base64

**Branch**: `001-base64-decoder` | **Date**: 2026-01-14 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-base64-decoder/spec.md`

## Summary

CLI tool que decodifica datos Base64 (desde argumento, archivo, o stdin) a archivo,
muestra metadatos del contenido (tamaño, MIME type, extensión), y permite
visualización/reproducción interactiva con la aplicación nativa del SO.

Enfoque técnico: Arquitectura biblioteca-first en Rust, con una biblioteca core
para decodificación/detección y una capa CLI delgada que la consume.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**: clap (CLI parsing), base64 (encoding), infer (MIME detection)
**Storage**: Sistema de archivos (temporal + rutas especificadas)
**Testing**: cargo test (unit + integration), proptest (property-based)
**Target Platform**: macOS, Linux, Windows (cross-compilation)
**Project Type**: Single project (biblioteca + CLI)
**Performance Goals**: <2s para 1MB, <5s para 10MB (SC-001, SC-003)
**Constraints**: <100MB límite default, streaming para archivos grandes
**Scale/Scope**: CLI local, single-user, sin persistencia de estado

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principio | Requisito | Cumplimiento | Notas |
|-----------|-----------|--------------|-------|
| I. Biblioteca-First | Funcionalidad en biblioteca independiente | ✅ | `decodeck-core` lib + `decodeck` CLI bin |
| II. Interfaz CLI Dual | Soportar texto + JSON | ✅ | `--json` flag especificado en FR-015 |
| III. TDD Obligatorio | Tests primero | ✅ | Workflow en tasks.md seguirá TDD |
| IV. Seguridad Primero | Validar inputs, sanitizar rutas | ✅ | FR-021, FR-022, FR-023 |
| V. Simplicidad (YAGNI) | Solución mínima | ✅ | Solo Base64, sin features especulativos |
| VI. Versionado Semántico | SemVer estricto | ✅ | Definido en Cargo.toml |
| VII. Observabilidad | Logging, verbose, errores claros | ✅ | `--verbose`, SC-004 |

**Gate Status**: ✅ PASSED - Todos los principios satisfechos

## Project Structure

### Documentation (this feature)

```text
specs/001-base64-decoder/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0: Technology decisions
├── data-model.md        # Phase 1: Entity definitions
├── quickstart.md        # Phase 1: Getting started guide
├── contracts/           # Phase 1: API contracts
│   └── cli-interface.md # CLI commands and flags
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── tasks.md             # Phase 2: Implementation tasks
```

### Source Code (repository root)

```text
src/
├── lib.rs               # Library crate root (decodeck-core)
├── decoder/
│   ├── mod.rs           # Base64 decoding logic
│   └── variants.rs      # Standard vs URL-safe handling
├── metadata/
│   ├── mod.rs           # MIME detection, file info
│   └── magic.rs         # Magic bytes database
├── input/
│   ├── mod.rs           # Input source abstraction
│   └── sources.rs       # Arg, file, stdin handlers
├── output/
│   ├── mod.rs           # Output formatting
│   ├── text.rs          # Human-readable output
│   └── json.rs          # JSON structured output
└── bin/
    └── decodeck.rs      # CLI binary entry point

tests/
├── integration/
│   ├── decode_test.rs   # E2E decode scenarios
│   ├── metadata_test.rs # MIME detection tests
│   └── cli_test.rs      # CLI flag combinations
└── fixtures/
    ├── samples/         # Known Base64 samples
    └── expected/        # Expected decoded outputs
```

**Structure Decision**: Single project with workspace-style organization. La biblioteca
(`src/lib.rs`) expone la API pública, el binario (`src/bin/decodeck.rs`) es una capa
delgada que usa clap para CLI parsing y llama a la biblioteca.

## Complexity Tracking

> No hay violaciones de constitución que justificar.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |
