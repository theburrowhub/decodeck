# Tasks: Decodificador Base64

**Input**: Design documents from `/specs/001-base64-decoder/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/cli-interface.md

**Tests**: TDD obligatorio según Principio III de la constitución. Tests PRIMERO, implementación DESPUÉS.

**Organization**: Tasks agrupadas por user story para implementación y testing independiente.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Puede ejecutarse en paralelo (archivos diferentes, sin dependencias)
- **[Story]**: User story a la que pertenece (US1, US2, US3, US4)
- Rutas exactas incluidas en descripciones

## Path Conventions

```text
src/
├── lib.rs               # Library crate root
├── decoder/             # Base64 decoding (US1)
├── metadata/            # MIME detection (US2)
├── input/               # Input sources (US1)
├── output/              # Formatters (US2)
├── interactive/         # Terminal interaction (US3)
└── bin/decodeck.rs      # CLI entry point

tests/
├── integration/         # E2E tests
└── fixtures/            # Test data
```

---

## Phase 1: Setup (Proyecto Base)

**Purpose**: Inicialización del proyecto Rust y estructura básica

- [x] T001 Initialize Rust project with `cargo init --name decodeck` at repository root
- [x] T002 Configure Cargo.toml with dependencies: clap, base64, infer, crossterm, serde, serde_json, thiserror, anyhow, tracing, tracing-subscriber, opener
- [x] T003 [P] Configure dev-dependencies: proptest, assert_cmd, predicates, tempfile in Cargo.toml
- [x] T004 [P] Create .rustfmt.toml with project formatting rules
- [x] T005 [P] Create .clippy.toml with lint configuration
- [x] T006 Create src/lib.rs with module structure declarations
- [x] T007 Create src/bin/decodeck.rs with placeholder main function

---

## Phase 2: Foundational (Infraestructura Core)

**Purpose**: Tipos base y error handling que TODAS las user stories necesitan

**⚠️ CRITICAL**: Ninguna user story puede comenzar hasta completar esta fase

- [x] T008 Define error types with thiserror in src/error.rs (DecodeckError enum with all error codes from contracts)
- [x] T009 [P] Create src/input/mod.rs with InputSource and SourceType enums
- [x] T010 [P] Create src/decoder/mod.rs with Base64Variant enum and EncodedData struct
- [x] T011 [P] Create src/metadata/mod.rs with ContentMetadata and ContentCategory structs
- [x] T012 [P] Create src/output/mod.rs with OutputFile and DecodeResult structs
- [x] T013 Configure tracing subscriber for logging levels in src/lib.rs
- [x] T014 Create tests/fixtures/samples/ directory with test Base64 files (text, png, pdf)
- [x] T015 Create tests/fixtures/expected/ directory with expected decoded outputs

**Checkpoint**: Infraestructura lista - implementación de user stories puede comenzar

---

## Phase 3: User Story 1 - Decodificar Base64 a Archivo (Priority: P1) 🎯 MVP

**Goal**: Decodificar datos Base64 desde argumento, archivo o stdin y guardar como archivo

**Independent Test**: Pasar string Base64 conocido y verificar que archivo resultante coincide byte a byte

### Tests for User Story 1 (TDD - DEBEN FALLAR PRIMERO)

- [x] T016 [P] [US1] Write unit tests for Base64 decoding (standard alphabet) in tests/unit/decoder_test.rs
- [x] T017 [P] [US1] Write unit tests for URL-safe Base64 decoding in tests/unit/decoder_test.rs
- [x] T018 [P] [US1] Write unit tests for padding handling (with/without) in tests/unit/decoder_test.rs
- [x] T019 [P] [US1] Write unit tests for whitespace stripping in tests/unit/decoder_test.rs
- [x] T020 [P] [US1] Write unit tests for invalid Base64 error reporting in tests/unit/decoder_test.rs
- [x] T021 [P] [US1] Write integration test for decode from argument in tests/integration/decode_test.rs
- [x] T022 [P] [US1] Write integration test for decode from file in tests/integration/decode_test.rs
- [x] T023 [P] [US1] Write integration test for decode from stdin in tests/integration/decode_test.rs
- [x] T024 [US1] Run `cargo test` and verify ALL tests FAIL (Red phase)

### Implementation for User Story 1

- [x] T025 [P] [US1] Implement InputSource::from_arg() in src/input/sources.rs
- [x] T026 [P] [US1] Implement InputSource::from_file() in src/input/sources.rs
- [x] T027 [P] [US1] Implement InputSource::from_stdin() in src/input/sources.rs
- [x] T028 [US1] Implement input source auto-detection in src/input/mod.rs (arg > file > stdin priority)
- [x] T029 [US1] Implement Base64 variant detection (standard vs URL-safe) in src/decoder/variants.rs
- [x] T030 [US1] Implement whitespace stripping and padding normalization in src/decoder/mod.rs
- [x] T031 [US1] Implement Base64 decoding using base64 crate in src/decoder/mod.rs
- [x] T032 [US1] Implement temporary file creation in src/output/mod.rs
- [x] T033 [US1] Implement file writing with path sanitization in src/output/mod.rs
- [x] T034 [US1] Wire decode command in src/bin/decodeck.rs with clap
- [x] T035 [US1] Run `cargo test` and verify ALL US1 tests PASS (Green phase)
- [x] T036 [US1] Run `cargo clippy` and fix any warnings

**Checkpoint**: US1 completa - `decodeck decode "SGVsbG8="` funciona y guarda archivo

---

## Phase 4: User Story 2 - Mostrar Metadatos del Contenido (Priority: P2)

**Goal**: Detectar y mostrar MIME type, extensión sugerida, tamaño formateado

**Independent Test**: Decodificar PNG/PDF conocido y verificar metadatos correctos

### Tests for User Story 2 (TDD - DEBEN FALLAR PRIMERO)

- [x] T037 [P] [US2] Write unit tests for MIME detection (PNG, PDF, MP3) in tests/unit/metadata_test.rs
- [x] T038 [P] [US2] Write unit tests for extension mapping in tests/unit/metadata_test.rs
- [x] T039 [P] [US2] Write unit tests for size formatting (bytes, KB, MB) in tests/unit/metadata_test.rs
- [x] T040 [P] [US2] Write unit tests for ContentCategory classification in tests/unit/metadata_test.rs
- [x] T041 [P] [US2] Write integration test for text output format in tests/integration/output_test.rs
- [x] T042 [P] [US2] Write integration test for JSON output format in tests/integration/output_test.rs
- [x] T043 [US2] Run `cargo test` for US2 tests and verify they FAIL (Red phase)

### Implementation for User Story 2

- [x] T044 [P] [US2] Implement MIME detection using infer crate in src/metadata/magic.rs
- [x] T045 [P] [US2] Implement extension-to-MIME mapping in src/metadata/mod.rs
- [x] T046 [US2] Implement ContentCategory classification from MIME type in src/metadata/mod.rs
- [x] T047 [US2] Implement is_viewable and is_playable logic in src/metadata/mod.rs
- [x] T048 [US2] Implement size formatting (human-readable) in src/output/mod.rs
- [x] T049 [P] [US2] Implement text formatter in src/output/text.rs
- [x] T050 [P] [US2] Implement JSON formatter with serde in src/output/json.rs
- [x] T051 [US2] Add --json flag to CLI in src/bin/decodeck.rs
- [x] T052 [US2] Integrate metadata display into decode command output
- [x] T053 [US2] Run `cargo test` for US2 tests and verify they PASS (Green phase)

**Checkpoint**: US2 completa - decodeck muestra metadatos en texto y JSON

---

## Phase 5: User Story 3 - Visualización Interactiva (Priority: P3)

**Goal**: Mostrar prompt "Press space to view|play" y abrir archivo con app nativa

**Independent Test**: Decodificar imagen, verificar prompt, presionar espacio, verificar apertura

### Tests for User Story 3 (TDD - DEBEN FALLAR PRIMERO)

- [x] T054 [P] [US3] Write unit tests for terminal detection (is_terminal) in tests/unit/interactive_test.rs
- [x] T055 [P] [US3] Write unit tests for prompt text selection (view vs play) in tests/unit/interactive_test.rs
- [x] T056 [P] [US3] Write unit tests for key handling (space, enter, other) in tests/unit/interactive_test.rs
- [x] T057 [P] [US3] Write integration test for --quiet suppression in tests/cli_integration.rs
- [x] T058 [P] [US3] Write integration test for --no-interactive flag in tests/cli_integration.rs
- [x] T059 [US3] Run `cargo test` for US3 tests and verify they FAIL (Red phase)

### Implementation for User Story 3

- [x] T060 [US3] Create src/interactive/mod.rs with InteractivePrompt struct
- [x] T061 [US3] Implement terminal detection using std::io::IsTerminal in src/interactive/mod.rs
- [x] T062 [US3] Implement prompt display using crossterm in src/interactive/mod.rs
- [x] T063 [US3] Implement key capture (space, enter, other) using crossterm in src/interactive/mod.rs
- [x] T064 [US3] Implement file opener using opener crate in src/interactive/mod.rs
- [x] T065 [US3] Add --quiet and --no-interactive flags to CLI in src/bin/decodeck.rs
- [x] T066 [US3] Integrate interactive prompt into decode flow (skip in pipe mode)
- [x] T067 [US3] Run `cargo test` for US3 tests and verify they PASS (Green phase)

**Checkpoint**: US3 completa - prompt interactivo funciona con archivos visualizables/reproducibles

---

## Phase 6: User Story 4 - Salida a Ruta Específica (Priority: P4)

**Goal**: Permitir --output para especificar ruta de destino, --force para sobrescribir

**Independent Test**: Especificar --output y verificar archivo en ubicación exacta

### Tests for User Story 4 (TDD - DEBEN FALLAR PRIMERO)

- [x] T068 [P] [US4] Write unit tests for output path validation in tests/unit/output_test.rs
- [x] T069 [P] [US4] Write unit tests for path traversal prevention in tests/unit/output_test.rs
- [x] T070 [P] [US4] Write unit tests for existing file detection in tests/unit/output_test.rs
- [x] T071 [P] [US4] Write integration test for overwrite confirmation prompt in tests/cli_integration.rs
- [x] T072 [P] [US4] Write integration test for --output flag in tests/cli_integration.rs
- [x] T073 [P] [US4] Write integration test for --force flag in tests/cli_integration.rs
- [x] T074 [US4] Run `cargo test` for US4 tests and verify they FAIL (Red phase)

### Implementation for User Story 4

- [x] T075 [US4] Implement output path validation in src/output/mod.rs
- [x] T076 [US4] Implement path traversal detection and prevention in src/output/mod.rs
- [x] T077 [US4] Implement existing file check with confirmation prompt in src/output/mod.rs
- [x] T078 [US4] Add --output and --force flags to CLI in src/bin/decodeck.rs
- [x] T079 [US4] Integrate custom output path into decode flow
- [x] T080 [US4] Run `cargo test` for US4 tests and verify they PASS (Green phase)

**Checkpoint**: US4 completa - --output y --force funcionan correctamente

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Refinamiento final, edge cases, observabilidad y validación de performance

- [x] T081 [P] Add --verbose flag implementation with tracing spans in src/bin/decodeck.rs
- [x] T082 [P] Add --max-size flag for input size limits in src/bin/decodeck.rs
- [ ] T083 Implement size limit validation in src/input/mod.rs
- [ ] T084 [P] Implement stdin timeout detection (5 seconds) in src/input/sources.rs
- [x] T085 [P] Add proper exit codes (0, 1, 2) throughout error handling
- [x] T086 Ensure all error messages include context per SC-004
- [ ] T087 [P] Write property-based tests with proptest in tests/property/decode_properties.rs
- [ ] T088 [P] Write performance test: decode 1MB file in <2s (SC-001) in tests/performance/perf_test.rs
- [ ] T089 [P] Write performance test: full flow for 10MB file in <5s (SC-003) in tests/performance/perf_test.rs
- [x] T090 Run full test suite: `cargo test --all-features`
- [x] T091 Run clippy with all lints: `cargo clippy -- -D warnings`
- [x] T092 Validate quickstart.md examples work correctly
- [x] T093 Build release binary: `cargo build --release`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Sin dependencias - puede comenzar inmediatamente
- **Foundational (Phase 2)**: Depende de Setup - BLOQUEA todas las user stories
- **User Stories (Phases 3-6)**: Todas dependen de Foundational
  - US1 → US2 → US3 → US4 (secuencial recomendado)
  - O en paralelo si hay múltiples desarrolladores
- **Polish (Phase 7)**: Depende de todas las user stories completadas

### User Story Dependencies

- **US1 (P1)**: Después de Foundational - Sin dependencias de otras stories
- **US2 (P2)**: Después de US1 (necesita decoded content para metadatos)
- **US3 (P3)**: Después de US2 (necesita metadatos para view/play decision)
- **US4 (P4)**: Después de US1 (extiende output handling) - puede ir en paralelo con US2/US3

### Within Each User Story (TDD Flow)

1. Tests PRIMERO - DEBEN FALLAR (Red)
2. Implementación - DEBEN PASAR tests (Green)
3. Refactor - Mantener tests pasando
4. Checkpoint - Verificar story funciona independientemente

### Parallel Opportunities

**Phase 1 (Setup):**
```
T003 + T004 + T005 (parallel - config files)
```

**Phase 2 (Foundational):**
```
T009 + T010 + T011 + T012 (parallel - independent modules)
```

**US1 Tests:**
```
T016 + T017 + T018 + T019 + T020 + T021 + T022 + T023 (all parallel)
```

**US1 Implementation:**
```
T025 + T026 + T027 (parallel - input sources)
```

**US2 Tests:**
```
T037 + T038 + T039 + T040 + T041 + T042 (all parallel)
```

---

## Implementation Strategy

### MVP First (Solo User Story 1)

1. Complete Phase 1: Setup (T001-T007)
2. Complete Phase 2: Foundational (T008-T015)
3. Complete Phase 3: US1 Tests + Implementation (T016-T036)
4. **STOP and VALIDATE**: `decodeck decode "SGVsbG8="` funciona
5. Deploy/demo MVP

### Incremental Delivery

| Increment | Stories | Deliverable |
|-----------|---------|-------------|
| MVP | US1 | Decode Base64 to file |
| v0.2 | US1+US2 | + Metadata display |
| v0.3 | US1+US2+US3 | + Interactive viewing |
| v1.0 | All | + Custom output paths |

### TDD Checklist per Story

```text
[ ] Write ALL tests for story
[ ] Run tests - verify RED (all fail)
[ ] Implement feature
[ ] Run tests - verify GREEN (all pass)
[ ] Run clippy - fix warnings
[ ] Checkpoint - manual verification
```

---

## Task Summary

| Phase | Tasks | Parallel Opportunities |
|-------|-------|----------------------|
| Setup | 7 | 3 |
| Foundational | 8 | 4 |
| US1 (P1) | 21 | 11 |
| US2 (P2) | 17 | 9 |
| US3 (P3) | 14 | 5 |
| US4 (P4) | 13 | 6 |
| Polish | 13 | 7 |
| **Total** | **93** | **45 (48%)** |
