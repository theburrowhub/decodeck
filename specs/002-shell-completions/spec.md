# Feature Specification: Shell Completions

**Feature Branch**: `002-shell-completions`
**Created**: 2026-01-14
**Status**: Draft
**Input**: User description: "Añadir generación de scripts de autocompletado para shells (bash, zsh, fish, powershell). El comando `decodeck completions <shell>` debe generar el script de completado para el shell especificado y mostrarlo en stdout para que el usuario pueda redirigirlo a un archivo o evaluarlo directamente."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Generate Bash Completions (Priority: P1)

Como usuario de bash, quiero generar el script de autocompletado para poder tener sugerencias automáticas de comandos y flags cuando escribo `decodeck` en mi terminal.

**Why this priority**: Bash es el shell más común en sistemas Linux y macOS legacy, representando la mayoría de usuarios. El autocompletado mejora significativamente la experiencia de usuario y reduce errores.

**Independent Test**: Ejecutar `decodeck completions bash`, verificar que genera un script válido de bash que cuando se evalúa proporciona completado para todos los subcomandos y flags.

**Acceptance Scenarios**:

1. **Given** el usuario tiene bash como shell, **When** ejecuta `decodeck completions bash`, **Then** se imprime en stdout un script de completado válido para bash
2. **Given** el script de completado generado, **When** el usuario lo evalúa con `source` o `eval`, **Then** al escribir `decodeck d<TAB>` se completa a `decodeck decode`
3. **Given** el completado activo, **When** el usuario escribe `decodeck decode --<TAB>`, **Then** se muestran todas las flags disponibles (--output, --json, --quiet, etc.)

---

### User Story 2 - Generate Zsh Completions (Priority: P2)

Como usuario de zsh, quiero generar el script de autocompletado para tener sugerencias automáticas con descripciones de cada opción.

**Why this priority**: Zsh es el shell por defecto en macOS moderno y muy popular entre desarrolladores. Su sistema de completado es más rico que bash.

**Independent Test**: Ejecutar `decodeck completions zsh` y verificar que genera un script compatible con el sistema de completado de zsh.

**Acceptance Scenarios**:

1. **Given** el usuario tiene zsh como shell, **When** ejecuta `decodeck completions zsh`, **Then** se imprime un script de completado válido para zsh
2. **Given** el completado instalado, **When** el usuario escribe `decodeck decode -<TAB>`, **Then** se muestran las flags con sus descripciones

---

### User Story 3 - Generate Fish Completions (Priority: P3)

Como usuario de fish shell, quiero generar el script de autocompletado para aprovechar las capacidades de sugerencia automática de fish.

**Why this priority**: Fish tiene una base de usuarios creciente que valora la experiencia de usuario. Su formato de completado es diferente a bash/zsh.

**Independent Test**: Ejecutar `decodeck completions fish` y verificar que genera comandos `complete` válidos para fish.

**Acceptance Scenarios**:

1. **Given** el usuario tiene fish como shell, **When** ejecuta `decodeck completions fish`, **Then** se imprime un script de completado válido para fish
2. **Given** el completado instalado, **When** el usuario escribe `decodeck `, **Then** fish sugiere automáticamente los subcomandos disponibles

---

### User Story 4 - Generate PowerShell Completions (Priority: P4)

Como usuario de PowerShell en Windows, quiero generar el script de autocompletado para tener la misma experiencia que en shells Unix.

**Why this priority**: Soporte para usuarios Windows. PowerShell es el shell moderno por defecto en Windows.

**Independent Test**: Ejecutar `decodeck completions powershell` y verificar que genera un script de completado válido para PowerShell.

**Acceptance Scenarios**:

1. **Given** el usuario tiene PowerShell, **When** ejecuta `decodeck completions powershell`, **Then** se imprime un script de completado válido
2. **Given** el completado cargado, **When** el usuario escribe `decodeck <TAB>`, **Then** PowerShell sugiere los subcomandos

---

### Edge Cases

- ¿Qué pasa si el usuario especifica un shell no soportado? → Error claro con lista de shells válidos
- ¿Qué pasa si el usuario no especifica ningún shell? → Mostrar ayuda con shells disponibles
- ¿Qué pasa si el script se ejecuta en un entorno sin terminal? → Debe funcionar igual (stdout)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Sistema DEBE añadir un nuevo subcomando `completions` al CLI
- **FR-002**: El subcomando `completions` DEBE aceptar un argumento obligatorio especificando el shell target
- **FR-003**: Sistema DEBE soportar generación de completions para: bash, zsh, fish, powershell
- **FR-004**: Sistema DEBE imprimir el script de completado a stdout (sin escribir archivos)
- **FR-005**: El script generado DEBE incluir completado para todos los subcomandos existentes (decode)
- **FR-006**: El script generado DEBE incluir completado para todas las flags de cada subcomando
- **FR-007**: El script generado DEBE incluir descripciones de cada opción cuando el shell lo soporte
- **FR-008**: Sistema DEBE mostrar error claro si el shell especificado no está soportado
- **FR-009**: Sistema DEBE mostrar la lista de shells válidos en el mensaje de error y en --help
- **FR-010**: El comando `decodeck completions --help` DEBE mostrar ejemplos de uso para cada shell

### Key Entities

- **Shell**: Identificador del shell target (bash, zsh, fish, powershell)
- **Completion Script**: Script generado específico para cada shell que define las reglas de completado

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: El comando `decodeck completions <shell>` genera un script válido en menos de 100ms
- **SC-002**: Los scripts generados funcionan correctamente en las versiones actuales de cada shell (bash 4+, zsh 5+, fish 3+, PowerShell 5+)
- **SC-003**: El completado incluye el 100% de los subcomandos y flags disponibles
- **SC-004**: Los usuarios pueden instalar el completado siguiendo las instrucciones mostradas con `--help`

## Assumptions

- El proyecto usa clap para CLI parsing, que tiene soporte nativo para generación de completions
- Los usuarios saben cómo configurar su shell para cargar scripts de completado
- Se proporcionará documentación sobre cómo instalar el completado para cada shell
