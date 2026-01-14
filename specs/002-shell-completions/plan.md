# Implementation Plan: Shell Completions

**Branch**: `002-shell-completions` | **Date**: 2026-01-14 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-shell-completions/spec.md`

## Summary

Añadir comando `decodeck completions <shell>` que genera scripts de autocompletado para bash, zsh, fish y powershell. Utilizando el soporte nativo de clap para generación de completions via `clap_complete`.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**: clap 4.4 (existing), clap_complete (new)
**Storage**: N/A (solo salida a stdout)
**Testing**: cargo test + tests de integración CLI
**Target Platform**: macOS + Linux + Windows (cross-platform)
**Project Type**: Single CLI project
**Performance Goals**: <100ms para generación de script
**Constraints**: Scripts deben funcionar en versiones estables de cada shell
**Scale/Scope**: 4 shells soportados (bash, zsh, fish, powershell)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Biblioteca-First | ✅ | clap_complete es la biblioteca, CLI consume |
| II. Interfaz CLI Dual | ✅ | Salida texto a stdout, --help disponible |
| III. TDD Obligatorio | ✅ | Tests para cada shell antes de implementar |
| IV. Seguridad Primero | ✅ | Solo genera texto, no ejecuta nada |
| V. Simplicidad (YAGNI) | ✅ | Usa soporte nativo de clap, mínimo código |
| VI. Versionado Semántico | ✅ | Nueva feature = MINOR bump |
| VII. Observabilidad | ✅ | --help con ejemplos de instalación |

**Gate Status**: ✅ PASSED - No violations

## Project Structure

### Documentation (this feature)

```text
specs/002-shell-completions/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # N/A (no data model needed)
├── quickstart.md        # Installation instructions
├── contracts/
│   └── cli-interface.md # CLI contract
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
src/
├── bin/
│   └── decodeck.rs      # Añadir subcomando Completions
└── completions/         # Nuevo módulo (opcional, puede estar en bin)
    └── mod.rs           # Generación de completions

tests/
├── integration/
│   └── completions_test.rs  # Tests de integración
└── unit/
    └── completions_test.rs  # Tests unitarios
```

**Structure Decision**: Extensión mínima del proyecto existente. El código de completions puede vivir directamente en `decodeck.rs` usando clap_complete, o en un módulo separado si crece.

## Complexity Tracking

> No violations - no complexity justification needed
