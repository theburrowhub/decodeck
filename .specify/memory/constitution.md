<!--
SYNC IMPACT REPORT
==================
Version change: N/A → 1.0.0
Modified principles: N/A (initial creation)
Added sections:
  - 7 Core Principles (I-VII)
  - Constraints Adicionales
  - Flujo de Desarrollo
  - Governance
Removed sections: N/A
Templates requiring updates:
  - .specify/templates/plan-template.md: ✅ Compatible (Constitution Check section exists)
  - .specify/templates/spec-template.md: ✅ Compatible (Requirements section aligns)
  - .specify/templates/tasks-template.md: ✅ Compatible (Test-first workflow supported)
Follow-up TODOs: None
-->

# Decodeck Constitution

## Core Principles

### I. Biblioteca-First

Cada funcionalidad DEBE comenzar como una biblioteca independiente y autocontenida.

- Las bibliotecas DEBEN ser testeables de forma independiente
- Las bibliotecas DEBEN estar documentadas con ejemplos de uso
- Cada biblioteca DEBE tener un propósito claro y específico
- NO se permiten bibliotecas "organizacionales" sin funcionalidad real
- La CLI DEBE ser una capa delgada que consume las bibliotecas

**Rationale**: Facilita testing, reutilización y mantenimiento. Permite consumir
la funcionalidad de decodeck tanto desde CLI como programáticamente.

### II. Interfaz CLI Dual

Toda funcionalidad expuesta via CLI DEBE soportar tanto formato texto legible
como JSON estructurado.

- Protocolo texto: stdin/args → stdout, errores → stderr
- Formato JSON: flag `--json` o `--output=json` para salida estructurada
- Errores DEBEN incluir códigos de salida significativos (0=éxito, 1=error usuario, 2=error sistema)
- La salida interactiva ("Press space to view|play") DEBE ser suprimible con `--quiet` o `--no-interactive`

**Rationale**: Permite composición con pipes Unix y también integración
programática desde otros lenguajes vía JSON parsing.

### III. TDD Obligatorio (NO NEGOCIABLE)

El desarrollo DEBE seguir Test-Driven Development de forma estricta.

- **Secuencia obligatoria**: Tests escritos → Usuario aprueba → Tests fallan → Implementar
- Ciclo Red-Green-Refactor DEBE ser respetado estrictamente
- NO se permite código de producción sin tests que lo respalden
- Tests de contrato para interfaces públicas de cada biblioteca
- Tests de integración para flujos de usuario completos

**Rationale**: Garantiza calidad, documenta comportamiento esperado, y previene
regresiones en funcionalidad de decodificación/codificación.

### IV. Seguridad Primero

Todo input DEBE ser validado y sanitizado antes de procesarse.

- Validar tamaño máximo de datos codificados antes de procesar
- Sanitizar nombres de archivo y rutas antes de escribir
- NO ejecutar contenido decodificado automáticamente (requiere acción explícita del usuario)
- Validar integridad de datos decodificados cuando sea posible (checksums, headers)
- Logging de operaciones sensibles (escritura de archivos, ejecución de viewers)
- Aplicar límites de memoria/tiempo para evitar DoS con inputs maliciosos

**Rationale**: Decodeck procesa datos de fuentes potencialmente no confiables.
La seguridad es crítica para prevenir ejecución de código malicioso o ataques de path traversal.

### V. Simplicidad (YAGNI)

Empezar siempre con la solución más simple que funcione.

- NO agregar features especulativos ("por si acaso")
- NO crear abstracciones hasta que se necesiten en 3+ lugares
- Preferir código duplicado sobre abstracción prematura
- Cada decisión de complejidad DEBE justificarse explícitamente
- Eliminar código muerto inmediatamente, no comentarlo

**Rationale**: La complejidad innecesaria dificulta el mantenimiento y aumenta
la superficie de bugs. Decodeck debe ser una herramienta simple y predecible.

### VI. Versionado Semántico

El proyecto DEBE seguir Semantic Versioning 2.0.0 estrictamente.

- **MAJOR**: Cambios incompatibles en API/CLI (flags removidos, formato de salida cambiado)
- **MINOR**: Nuevos formatos soportados, nuevas flags, funcionalidad adicional compatible
- **PATCH**: Bug fixes, mejoras de rendimiento, correcciones de documentación
- Breaking changes DEBEN documentarse en CHANGELOG con guía de migración
- Deprecations DEBEN advertirse al menos una versión MINOR antes de remover

**Rationale**: Usuarios de decodeck en scripts y pipelines necesitan predictibilidad
sobre qué cambios pueden romper sus workflows.

### VII. Observabilidad

El sistema DEBE ser debuggeable y trazable en todo momento.

- Logging estructurado con niveles (debug, info, warn, error)
- Flag `--verbose` o `-v` para información detallada de procesamiento
- Mensajes de error DEBEN incluir contexto suficiente para diagnosticar problemas
- Métricas de operación disponibles (tamaño procesado, tiempo, formato detectado)
- Modo dry-run (`--dry-run`) para previsualizar operaciones sin ejecutarlas

**Rationale**: Cuando un usuario reporta un problema con cierto archivo codificado,
debe ser posible reproducir y diagnosticar el issue rápidamente.

## Constraints Adicionales

### Stack Tecnológico

- El stack tecnológico específico se definirá en el plan de implementación
- Cualquier dependencia externa DEBE justificarse explícitamente
- Preferir bibliotecas estándar sobre dependencias de terceros cuando sea razonable

### Formatos Soportados

- Base64 es el formato primario y DEBE estar completo antes de agregar otros
- Nuevos formatos se agregan como bibliotecas independientes siguiendo Principio I
- Cada formato DEBE incluir detección automática (magic bytes, heurísticas)

### Compatibilidad

- Soportar entrada desde: archivo, stdin, argumento directo, pipe
- Soportar parsing recursivo de JSON/XML para encontrar datos codificados
- Comportamiento interactivo DEBE ser opt-out (deshabilitado en pipes automáticamente)

## Flujo de Desarrollo

### Proceso de Implementación

1. **Especificación**: Definir comportamiento esperado y casos de uso
2. **Tests primero**: Escribir tests que fallan (Principio III)
3. **Implementación**: Código mínimo para pasar tests (Principio V)
4. **Refactor**: Mejorar sin cambiar comportamiento
5. **Documentación**: Actualizar docs y ejemplos
6. **Review**: Verificar cumplimiento de todos los principios

### Code Review Requirements

- Todo PR DEBE verificar cumplimiento de los 7 principios
- Tests DEBEN pasar antes de merge
- Cambios de API DEBEN actualizar documentación
- Breaking changes requieren actualización de versión MAJOR

### Quality Gates

- Cobertura de tests mínima: definir en configuración de CI
- Linting y formateo automático obligatorio
- Security scanning para dependencias

## Governance

### Autoridad

Esta constitución es el documento rector del proyecto Decodeck.
Todos los PRs, reviews y decisiones técnicas DEBEN verificar cumplimiento.

### Enmiendas

- Cambios a la constitución DEBEN documentarse con rationale
- Cambios DEBEN incluir plan de migración si afectan código existente
- El versionado de la constitución sigue el mismo esquema semántico (Principio VI)
- Toda enmienda requiere actualización del Sync Impact Report

### Cumplimiento

- Los reviews de código DEBEN incluir checklist de principios
- Violaciones DEBEN justificarse en la sección "Complexity Tracking" del plan
- El archivo `CLAUDE.md` o equivalente contiene guías de runtime para agentes de desarrollo

**Version**: 1.0.0 | **Ratified**: 2026-01-14 | **Last Amended**: 2026-01-14
