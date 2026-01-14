# Tasks: Shell Completions

**Input**: Design documents from `/specs/002-shell-completions/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, contracts/cli-interface.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add clap_complete dependency and prepare CLI structure

- [x] T001 Add `clap_complete = "4.4"` dependency to Cargo.toml
- [x] T002 Add `Completions` variant to `Commands` enum in src/bin/decodeck.rs

---

## Phase 2: Foundational (Core Completions Infrastructure)

**Purpose**: Implement shared completions generation logic that all shells use

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T003 Create `run_completions()` function in src/bin/decodeck.rs using clap_complete::generate()
- [x] T004 Wire `Completions` command handler in main() match block in src/bin/decodeck.rs
- [x] T005 [P] Create integration test file tests/completions_test.rs with test helper functions

**Checkpoint**: Foundation ready - shell-specific tests and validation can now begin

---

## Phase 3: User Story 1 - Bash Completions (Priority: P1) 🎯 MVP

**Goal**: Generate valid bash completion script with subcommand and flag completions

**Independent Test**: Run `decodeck completions bash`, verify output contains `complete -F` and references `decode` subcommand

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T006 [US1] Add test `test_bash_completions_generates_output` in tests/completions_test.rs
- [x] T007 [US1] Add test `test_bash_completions_contains_complete_command` in tests/completions_test.rs
- [x] T008 [US1] Add test `test_bash_completions_includes_decode_subcommand` in tests/completions_test.rs

### Implementation for User Story 1

- [x] T009 [US1] Run tests - verify they fail (Red phase) - SKIPPED: Implementation done in T003-T004
- [x] T010 [US1] Verify `Shell::Bash` variant generates non-empty output in src/bin/decodeck.rs
- [x] T011 [US1] Run tests - verify T006, T007, T008 pass (Green phase)

**Checkpoint**: `decodeck completions bash` generates working bash script

---

## Phase 4: User Story 2 - Zsh Completions (Priority: P2)

**Goal**: Generate valid zsh completion script with descriptions for options

**Independent Test**: Run `decodeck completions zsh`, verify output contains `_decodeck` function definition

### Tests for User Story 2

- [x] T012 [P] [US2] Add test `test_zsh_completions_generates_output` in tests/completions_test.rs
- [x] T013 [P] [US2] Add test `test_zsh_completions_contains_function_definition` in tests/completions_test.rs
- [x] T014 [P] [US2] Add test `test_zsh_completions_includes_decode_subcommand` in tests/completions_test.rs

### Implementation for User Story 2

- [x] T015 [US2] Run tests - verify they fail (Red phase) - SKIPPED: Implementation done in T003-T004
- [x] T016 [US2] Verify `Shell::Zsh` variant generates non-empty output in src/bin/decodeck.rs
- [x] T017 [US2] Run tests - verify T012, T013, T014 pass (Green phase)

**Checkpoint**: `decodeck completions zsh` generates working zsh script

---

## Phase 5: User Story 3 - Fish Completions (Priority: P3)

**Goal**: Generate valid fish completion script with `complete -c` commands

**Independent Test**: Run `decodeck completions fish`, verify output contains `complete -c decodeck`

### Tests for User Story 3

- [x] T018 [P] [US3] Add test `test_fish_completions_generates_output` in tests/completions_test.rs
- [x] T019 [P] [US3] Add test `test_fish_completions_contains_complete_command` in tests/completions_test.rs
- [x] T020 [P] [US3] Add test `test_fish_completions_includes_decode_subcommand` in tests/completions_test.rs

### Implementation for User Story 3

- [x] T021 [US3] Run tests - verify they fail (Red phase) - SKIPPED: Implementation done in T003-T004
- [x] T022 [US3] Verify `Shell::Fish` variant generates non-empty output in src/bin/decodeck.rs
- [x] T023 [US3] Run tests - verify T018, T019, T020 pass (Green phase)

**Checkpoint**: `decodeck completions fish` generates working fish script

---

## Phase 6: User Story 4 - PowerShell Completions (Priority: P4)

**Goal**: Generate valid PowerShell completion script with `Register-ArgumentCompleter`

**Independent Test**: Run `decodeck completions powershell`, verify output contains `Register-ArgumentCompleter`

### Tests for User Story 4

- [x] T024 [P] [US4] Add test `test_powershell_completions_generates_output` in tests/completions_test.rs
- [x] T025 [P] [US4] Add test `test_powershell_completions_contains_register_command` in tests/completions_test.rs
- [x] T026 [P] [US4] Add test `test_powershell_completions_includes_decode_subcommand` in tests/completions_test.rs

### Implementation for User Story 4

- [x] T027 [US4] Run tests - verify they fail (Red phase) - SKIPPED: Implementation done in T003-T004
- [x] T028 [US4] Verify `Shell::PowerShell` variant generates non-empty output in src/bin/decodeck.rs
- [x] T029 [US4] Run tests - verify T024, T025, T026 pass (Green phase)

**Checkpoint**: `decodeck completions powershell` generates working PowerShell script

---

## Phase 7: Edge Cases & Error Handling

**Purpose**: Handle invalid input and improve help text

### Tests for Edge Cases

- [x] T030 [P] Add test `test_completions_invalid_shell_error` in tests/completions_test.rs
- [x] T031 [P] Add test `test_completions_help_shows_shells` in tests/completions_test.rs

### Implementation

- [x] T032 Run edge case tests - verify they fail (Red phase) - SKIPPED: Implementation done in T002
- [x] T033 Verify clap automatically handles invalid shell with proper error message
- [x] T034 Add `#[command(after_help = ...)]` with installation examples for each shell in src/bin/decodeck.rs
- [x] T035 Run edge case tests - verify T030, T031 pass (Green phase)

**Checkpoint**: All error cases handled gracefully with helpful messages

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and documentation

- [x] T036 Run `cargo clippy` and fix any warnings (pre-existing warning in run_decode unrelated to this feature)
- [x] T037 Run `cargo fmt` to ensure code formatting
- [x] T038 Run full test suite `cargo test` - verify all 14 new tests pass
- [x] T039 Manual verification: test `decodeck completions bash` - generates valid bash script
- [x] T040 Manual verification: test `decodeck completions zsh` - generates valid zsh script

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can proceed sequentially (P1 → P2 → P3 → P4)
  - Or in parallel if using TDD (tests for all shells can be written in parallel)
- **Edge Cases (Phase 7)**: Depends on at least US1 being complete
- **Polish (Phase 8)**: Depends on all user stories and edge cases being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independent of US1
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Independent of US1/US2
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Independent of US1/US2/US3

### Within Each User Story (TDD Cycle)

1. Write tests FIRST (T006-T008 for US1, etc.)
2. Run tests - verify they FAIL (Red phase)
3. Implement minimal code to pass
4. Run tests - verify they PASS (Green phase)
5. Story complete before moving to next priority

### Parallel Opportunities

- All tests within a user story marked [P] can be written in parallel
- Tests for US2, US3, US4 can be written in parallel after US1 tests
- Edge case tests (T030, T031) can be written in parallel

---

## Parallel Example: Writing All Shell Tests

```bash
# After Phase 2 (Foundational) completes, launch all test tasks in parallel:
Task: "Add test test_bash_completions_generates_output in tests/completions_test.rs"
Task: "Add test test_zsh_completions_generates_output in tests/completions_test.rs"
Task: "Add test test_fish_completions_generates_output in tests/completions_test.rs"
Task: "Add test test_powershell_completions_generates_output in tests/completions_test.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (add dependency)
2. Complete Phase 2: Foundational (wire completions command)
3. Complete Phase 3: User Story 1 - Bash
4. **STOP and VALIDATE**: Test bash completions in real shell
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Completions command exists
2. Add User Story 1 (Bash) → Test independently → `0.2.0` release
3. Add User Story 2 (Zsh) → Test independently → Still `0.2.0`
4. Add User Story 3 (Fish) → Test independently → Still `0.2.0`
5. Add User Story 4 (PowerShell) → Test independently → Still `0.2.0`
6. Add Edge Cases + Polish → Feature complete → `0.2.0` release

### Estimated Task Count by Phase

| Phase | Tasks | Parallelizable |
|-------|-------|----------------|
| Phase 1: Setup | 2 | 0 |
| Phase 2: Foundational | 3 | 1 |
| Phase 3: US1 - Bash | 6 | 0 |
| Phase 4: US2 - Zsh | 6 | 3 |
| Phase 5: US3 - Fish | 6 | 3 |
| Phase 6: US4 - PowerShell | 6 | 3 |
| Phase 7: Edge Cases | 6 | 2 |
| Phase 8: Polish | 5 | 0 |
| **Total** | **40** | **12** |

---

## Notes

- [P] tasks = different files or independent sections, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- TDD cycle: Red (test fails) → Green (test passes) → Refactor
- clap_complete handles all shell-specific script generation - minimal code required
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
