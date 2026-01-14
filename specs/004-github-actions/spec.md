# Feature Specification: GitHub Actions CI/CD

**Feature Branch**: `004-github-actions`
**Created**: 2026-01-14
**Status**: Draft
**Input**: User description: "Añadir GitHub Actions para CI/CD: compilar en múltiples plataformas, ejecutar tests, y publicar releases en GitHub"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - CI on Pull Requests (Priority: P1)

Como desarrollador, quiero que cada Pull Request ejecute automáticamente los tests y verificaciones de calidad para asegurar que el código cumple los estándares antes de merge.

**Why this priority**: CI en PRs es el caso de uso más crítico - previene que código roto entre al main branch y da feedback rápido a los desarrolladores.

**Independent Test**: Crear un PR con cambios, verificar que se ejecutan tests, clippy, y formato automáticamente.

**Acceptance Scenarios**:

1. **Given** un PR abierto, **When** se hace push a la branch, **Then** se ejecutan todos los tests automáticamente
2. **Given** tests fallando, **When** el workflow termina, **Then** el PR se marca como fallido con detalles del error
3. **Given** tests pasando, **When** el workflow termina, **Then** el PR se marca como exitoso y listo para review

---

### User Story 2 - Multi-Platform Build (Priority: P2)

Como mantenedor del proyecto, quiero compilar binarios para Linux, macOS y Windows automáticamente para poder distribuir el software a usuarios de todas las plataformas.

**Why this priority**: Sin builds multi-plataforma, los usuarios tendrían que compilar manualmente, lo cual reduce adopción.

**Independent Test**: Trigger un build y verificar que se generan binarios para las 3 plataformas principales.

**Acceptance Scenarios**:

1. **Given** código en main, **When** se ejecuta el workflow de build, **Then** se compilan binarios para Linux x64
2. **Given** código en main, **When** se ejecuta el workflow de build, **Then** se compilan binarios para macOS (Intel y ARM)
3. **Given** código en main, **When** se ejecuta el workflow de build, **Then** se compilan binarios para Windows x64

---

### User Story 3 - Automated Release Publishing (Priority: P3)

Como mantenedor, quiero que al crear un tag de versión se publique automáticamente un release en GitHub con los binarios compilados para que los usuarios puedan descargar fácilmente.

**Why this priority**: Automatizar releases reduce trabajo manual y asegura consistencia en cada versión publicada.

**Independent Test**: Crear tag v1.0.0, verificar que se crea GitHub Release con binarios adjuntos.

**Acceptance Scenarios**:

1. **Given** un tag con formato vX.Y.Z, **When** se hace push del tag, **Then** se crea automáticamente un GitHub Release
2. **Given** release creado, **When** los builds terminan, **Then** los binarios se adjuntan como assets del release
3. **Given** release publicado, **When** un usuario visita la página, **Then** puede descargar binarios para su plataforma

---

### User Story 4 - Quality Checks (Priority: P4)

Como desarrollador, quiero que se ejecuten verificaciones de calidad (clippy, formato, security audit) automáticamente para mantener alta calidad de código.

**Why this priority**: Calidad automatizada previene deuda técnica y vulnerabilidades antes de que lleguen a producción.

**Independent Test**: Introducir código con warnings de clippy, verificar que el CI falla.

**Acceptance Scenarios**:

1. **Given** código con warnings de clippy, **When** se ejecuta CI, **Then** el workflow falla indicando los warnings
2. **Given** código mal formateado, **When** se ejecuta CI, **Then** el workflow falla indicando archivos a formatear
3. **Given** dependencia con vulnerabilidad conocida, **When** se ejecuta security audit, **Then** se reporta el problema

---

### Edge Cases

- ¿Qué pasa si el build falla en solo una plataforma? → Release no se publica, error visible en workflow
- ¿Qué pasa con tags que no siguen formato semver? → Se ignoran, no trigger release workflow
- ¿Qué pasa si hay timeout en compilación? → Retry automático con límite, luego fallo
- ¿Qué pasa con PRs de forks? → CI ejecuta pero con permisos limitados (no secrets)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: CI DEBE ejecutarse automáticamente en cada push a PRs
- **FR-002**: CI DEBE ejecutar todos los tests con `cargo test`
- **FR-003**: CI DEBE ejecutar clippy con warnings como errores (`-D warnings`)
- **FR-004**: CI DEBE verificar formato con `cargo fmt --check`
- **FR-005**: Build DEBE generar binarios para: linux-x64, macos-x64, macos-arm64, windows-x64
- **FR-006**: Release DEBE triggerearse automáticamente al crear tags vX.Y.Z
- **FR-007**: Release DEBE incluir binarios comprimidos (.tar.gz para Unix, .zip para Windows)
- **FR-008**: Release DEBE generar checksums SHA256 para cada archivo
- **FR-009**: Los binarios DEBEN nombrarse consistentemente: decodeck-{version}-{platform}.{ext}
- **FR-010**: CI DEBE completar en menos de 10 minutos para feedback rápido
- **FR-011**: Release DEBE incluir release notes generadas desde commits/changelog
- **FR-012**: CI DEBE cachear dependencias de Rust para acelerar builds subsecuentes

### Key Entities

- **Workflow**: Definición de pipeline CI/CD (archivo YAML en .github/workflows/)
- **Build Artifact**: Binario compilado para una plataforma específica
- **Release**: Versión publicada en GitHub con assets descargables
- **Platform Matrix**: Lista de plataformas objetivo para compilación

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: CI completa en menos de 10 minutos para PRs típicos
- **SC-002**: 100% de PRs ejecutan verificaciones de calidad antes de merge
- **SC-003**: Releases incluyen binarios funcionales para las 4 plataformas objetivo
- **SC-004**: Los binarios publicados pasan verificación de checksum SHA256
- **SC-005**: El proceso de release manual se reduce de 30+ minutos a 0 (completamente automatizado)

## Assumptions

- El repositorio está alojado en GitHub (no GitLab, Bitbucket, etc.)
- Se usará GitHub Actions como plataforma CI/CD (no Jenkins, CircleCI, etc.)
- Las credenciales necesarias (GITHUB_TOKEN) están disponibles automáticamente
- Los runners de GitHub tienen capacidad suficiente para compilar Rust
- Se seguirá versionado semántico (semver) para tags de release
