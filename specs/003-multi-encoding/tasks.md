# Tasks: Multi-Encoding Support

**Input**: Design documents from `/specs/003-multi-encoding/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, contracts/cli-interface.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add new dependencies and create module structure

- [x] T001 Add `data-encoding = "2.5"` dependency to Cargo.toml
- [x] T002 Add `percent-encoding = "2.3"` dependency to Cargo.toml
- [x] T003 Add `ascii85 = "0.2"` dependency to Cargo.toml (using ascii85 instead of base85)
- [x] T004 Create encoding module directory src/encoding/
- [x] T005 Create src/encoding/mod.rs with Decoder trait and EncodingType enum

---

## Phase 2: Foundational (Core Encoding Infrastructure)

**Purpose**: Create shared encoding infrastructure and refactor existing Base64

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T006 Define `Decoder` trait in src/encoding/mod.rs with decode(), name(), can_decode() methods
- [ ] T007 Define `EncodingType` enum (Base64, Hex, Base32, Url, Base85) with clap ValueEnum in src/encoding/mod.rs
- [ ] T008 Create src/encoding/base64.rs implementing Decoder trait (refactor from decoder/mod.rs)
- [ ] T009 Update src/decoder/mod.rs to use encoding module instead of inline Base64 logic
- [ ] T010 Add `--encoding` / `-e` flag to decode command in src/bin/decodeck.rs
- [ ] T011 Update DecodeResult struct to include encoding info in src/output/mod.rs
- [ ] T012 Update JSON output to include "encoding" field in src/output/json.rs
- [ ] T013 Update text output to show encoding used in src/output/text.rs
- [ ] T014 [P] Create tests/encoding_test.rs with test helper functions

**Checkpoint**: Foundation ready - encoding types can now be implemented

---

## Phase 3: User Story 1 - Hex Decoding (Priority: P1) 🎯 MVP

**Goal**: Decode hexadecimal data with support for 0x prefix, spaces, and case-insensitive input

**Independent Test**: Run `decodeck decode --encoding hex "48656c6c6f"`, verify output is "Hello"

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T015 [US1] Add test `test_hex_decode_simple` in tests/encoding_test.rs
- [ ] T016 [US1] Add test `test_hex_decode_with_0x_prefix` in tests/encoding_test.rs
- [ ] T017 [US1] Add test `test_hex_decode_with_spaces` in tests/encoding_test.rs
- [ ] T018 [US1] Add test `test_hex_decode_case_insensitive` in tests/encoding_test.rs
- [ ] T019 [US1] Add test `test_hex_decode_invalid_char_error` in tests/encoding_test.rs

### Implementation for User Story 1

- [ ] T020 [US1] Run tests T015-T019 - verify they fail (Red phase)
- [ ] T021 [US1] Create src/encoding/hex.rs with HexDecoder struct implementing Decoder trait
- [ ] T022 [US1] Implement hex decode with 0x prefix stripping in src/encoding/hex.rs
- [ ] T023 [US1] Implement whitespace removal in hex decode in src/encoding/hex.rs
- [ ] T024 [US1] Implement case-insensitive hex validation in src/encoding/hex.rs
- [ ] T025 [US1] Register HexDecoder in src/encoding/mod.rs
- [ ] T026 [US1] Wire hex encoding to CLI handler in src/bin/decodeck.rs
- [ ] T027 [US1] Run tests T015-T019 - verify they pass (Green phase)

**Checkpoint**: `decodeck decode --encoding hex` works for all hex formats

---

## Phase 4: User Story 2 - Base32 Decoding (Priority: P2)

**Goal**: Decode Base32 data with case-insensitive input and auto-padding

**Independent Test**: Run `decodeck decode --encoding base32 "JBSWY3DPEHPK3PXP"`, verify correct output

### Tests for User Story 2

- [ ] T028 [P] [US2] Add test `test_base32_decode_simple` in tests/encoding_test.rs
- [ ] T029 [P] [US2] Add test `test_base32_decode_lowercase` in tests/encoding_test.rs
- [ ] T030 [P] [US2] Add test `test_base32_decode_without_padding` in tests/encoding_test.rs
- [ ] T031 [P] [US2] Add test `test_base32_decode_invalid_char_error` in tests/encoding_test.rs

### Implementation for User Story 2

- [ ] T032 [US2] Run tests T028-T031 - verify they fail (Red phase)
- [ ] T033 [US2] Create src/encoding/base32.rs with Base32Decoder struct implementing Decoder trait
- [ ] T034 [US2] Implement case-insensitive Base32 decode in src/encoding/base32.rs
- [ ] T035 [US2] Implement auto-padding for Base32 in src/encoding/base32.rs
- [ ] T036 [US2] Register Base32Decoder in src/encoding/mod.rs
- [ ] T037 [US2] Wire base32 encoding to CLI handler in src/bin/decodeck.rs
- [ ] T038 [US2] Run tests T028-T031 - verify they pass (Green phase)

**Checkpoint**: `decodeck decode --encoding base32` works for TOTP secrets and similar

---

## Phase 5: User Story 3 - URL Decoding (Priority: P3)

**Goal**: Decode URL percent-encoded data with support for + as space

**Independent Test**: Run `decodeck decode --encoding url "Hello%20World%21"`, verify output is "Hello World!"

### Tests for User Story 3

- [ ] T039 [P] [US3] Add test `test_url_decode_percent` in tests/encoding_test.rs
- [ ] T040 [P] [US3] Add test `test_url_decode_plus_as_space` in tests/encoding_test.rs
- [ ] T041 [P] [US3] Add test `test_url_decode_utf8` in tests/encoding_test.rs
- [ ] T042 [P] [US3] Add test `test_url_decode_partial` in tests/encoding_test.rs
- [ ] T043 [P] [US3] Add test `test_url_decode_invalid_sequence_error` in tests/encoding_test.rs

### Implementation for User Story 3

- [ ] T044 [US3] Run tests T039-T043 - verify they fail (Red phase)
- [ ] T045 [US3] Create src/encoding/url.rs with UrlDecoder struct implementing Decoder trait
- [ ] T046 [US3] Implement percent-decoding in src/encoding/url.rs
- [ ] T047 [US3] Implement + to space conversion in src/encoding/url.rs
- [ ] T048 [US3] Register UrlDecoder in src/encoding/mod.rs
- [ ] T049 [US3] Wire url encoding to CLI handler in src/bin/decodeck.rs
- [ ] T050 [US3] Run tests T039-T043 - verify they pass (Green phase)

**Checkpoint**: `decodeck decode --encoding url` works for query strings and form data

---

## Phase 6: User Story 4 - Base85/Ascii85 Decoding (Priority: P4)

**Goal**: Decode Ascii85 data with support for <~ ~> delimiters

**Independent Test**: Run `decodeck decode --encoding base85 "<~87cURD]j7BEbo80~>"`, verify correct output

### Tests for User Story 4

- [ ] T051 [P] [US4] Add test `test_base85_decode_with_delimiters` in tests/encoding_test.rs
- [ ] T052 [P] [US4] Add test `test_base85_decode_without_delimiters` in tests/encoding_test.rs
- [ ] T053 [P] [US4] Add test `test_base85_decode_invalid_error` in tests/encoding_test.rs

### Implementation for User Story 4

- [ ] T054 [US4] Run tests T051-T053 - verify they fail (Red phase)
- [ ] T055 [US4] Create src/encoding/base85.rs with Base85Decoder struct implementing Decoder trait
- [ ] T056 [US4] Implement Ascii85 decode with delimiter handling in src/encoding/base85.rs
- [ ] T057 [US4] Register Base85Decoder in src/encoding/mod.rs
- [ ] T058 [US4] Wire base85 encoding to CLI handler in src/bin/decodeck.rs
- [ ] T059 [US4] Run tests T051-T053 - verify they pass (Green phase)

**Checkpoint**: `decodeck decode --encoding base85` works for PDF/PostScript data

---

## Phase 7: User Story 5 - Auto-Detection (Priority: P5)

**Goal**: Automatically detect encoding type when --encoding is not specified

**Independent Test**: Run `decodeck decode "0x48656c6c6f"` without --encoding, verify hex auto-detected

### Tests for User Story 5

- [ ] T060 [P] [US5] Add test `test_autodetect_hex_with_0x_prefix` in tests/encoding_test.rs
- [ ] T061 [P] [US5] Add test `test_autodetect_base85_with_delimiters` in tests/encoding_test.rs
- [ ] T062 [P] [US5] Add test `test_autodetect_url_with_percent` in tests/encoding_test.rs
- [ ] T063 [P] [US5] Add test `test_autodetect_hex_only_hex_chars` in tests/encoding_test.rs
- [ ] T064 [P] [US5] Add test `test_autodetect_default_base64` in tests/encoding_test.rs
- [ ] T065 [P] [US5] Add test `test_autodetect_ambiguous_shows_warning` in tests/encoding_test.rs

### Implementation for User Story 5

- [ ] T066 [US5] Run tests T060-T065 - verify they fail (Red phase)
- [ ] T067 [US5] Create src/encoding/detect.rs with detect_encoding() function
- [ ] T068 [US5] Implement 0x prefix detection for hex in src/encoding/detect.rs
- [ ] T069 [US5] Implement <~ ~> delimiter detection for base85 in src/encoding/detect.rs
- [ ] T070 [US5] Implement %XX pattern detection for url in src/encoding/detect.rs
- [ ] T071 [US5] Implement hex character heuristic in src/encoding/detect.rs
- [ ] T072 [US5] Implement Base32 character heuristic in src/encoding/detect.rs
- [ ] T073 [US5] Implement fallback to Base64 with confidence level in src/encoding/detect.rs
- [ ] T074 [US5] Wire auto-detection to CLI when --encoding not specified in src/bin/decodeck.rs
- [ ] T075 [US5] Add warning for ambiguous detection in src/bin/decodeck.rs
- [ ] T076 [US5] Run tests T060-T065 - verify they pass (Green phase)

**Checkpoint**: `decodeck decode` auto-detects encoding intelligently

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, integration tests, and cleanup

- [ ] T077 Run `cargo clippy` and fix any warnings
- [ ] T078 Run `cargo fmt` to ensure code formatting
- [ ] T079 [P] Add integration test for all encodings via CLI in tests/multi_encoding_integration.rs
- [ ] T080 [P] Add integration test for JSON output with encoding field in tests/multi_encoding_integration.rs
- [ ] T081 [P] Add integration test for error messages in tests/multi_encoding_integration.rs
- [ ] T082 Run full test suite `cargo test` - verify all new tests pass
- [ ] T083 Manual verification: test each encoding with sample data from quickstart.md
- [ ] T084 Verify backwards compatibility: existing base64 commands still work without --encoding

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - US1 (Hex) → US2 (Base32) → US3 (URL) → US4 (Base85) → US5 (Auto-detect)
  - Note: US5 depends on US1-US4 being complete for auto-detection to work
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independent of US1
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Independent of US1/US2
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Independent of US1/US2/US3
- **User Story 5 (P5)**: **DEPENDS on US1-US4** - Auto-detection needs all encoders available

### Within Each User Story (TDD Cycle)

1. Write tests FIRST (T015-T019 for US1, etc.)
2. Run tests - verify they FAIL (Red phase)
3. Implement minimal code to pass
4. Run tests - verify they PASS (Green phase)
5. Story complete before moving to next priority

### Parallel Opportunities

- All tests within a user story marked [P] can be written in parallel
- US1, US2, US3, US4 can be implemented in parallel (different files)
- Only US5 (Auto-detect) must wait for US1-US4
- Phase 8 integration tests marked [P] can run in parallel

---

## Parallel Example: Writing All Encoding Modules

```bash
# After Phase 2 (Foundational) completes, these can run in parallel:
Task: "Create src/encoding/hex.rs with HexDecoder struct"
Task: "Create src/encoding/base32.rs with Base32Decoder struct"
Task: "Create src/encoding/url.rs with UrlDecoder struct"
Task: "Create src/encoding/base85.rs with Base85Decoder struct"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (add dependencies)
2. Complete Phase 2: Foundational (trait, enum, refactor Base64)
3. Complete Phase 3: User Story 1 - Hex
4. **STOP and VALIDATE**: Test hex decoding in real scenarios
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Encoding framework ready
2. Add User Story 1 (Hex) → Test independently → Most common secondary encoding
3. Add User Story 2 (Base32) → Test independently → TOTP support
4. Add User Story 3 (URL) → Test independently → Web development support
5. Add User Story 4 (Base85) → Test independently → PDF/PostScript support
6. Add User Story 5 (Auto-detect) → Test independently → UX improvement
7. Polish → Feature complete → `0.3.0` release

### Estimated Task Count by Phase

| Phase | Tasks | Parallelizable |
|-------|-------|----------------|
| Phase 1: Setup | 5 | 0 |
| Phase 2: Foundational | 10 | 1 |
| Phase 3: US1 - Hex | 13 | 0 |
| Phase 4: US2 - Base32 | 11 | 4 |
| Phase 5: US3 - URL | 12 | 5 |
| Phase 6: US4 - Base85 | 9 | 3 |
| Phase 7: US5 - Auto-detect | 17 | 6 |
| Phase 8: Polish | 8 | 3 |
| **Total** | **85** | **22** |

---

## Notes

- [P] tasks = different files or independent sections, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable (except US5)
- TDD cycle: Red (test fails) → Green (test passes) → Refactor
- data-encoding crate handles hex and base32 with same API
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
