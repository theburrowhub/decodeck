# Feature Specification: Decodificador Base64

**Feature Branch**: `001-base64-decoder`
**Created**: 2026-01-14
**Status**: Draft
**Input**: User description: "Decodificador Base64 - CLI que decodifica datos en Base64 a archivo, muestra metadatos del contenido (tamaño, mimetype, extensión) y permite visualización/reproducción nativa con Press space to view|play"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Decodificar Base64 a Archivo (Priority: P1)

Como usuario, quiero decodificar datos en formato Base64 y guardarlos como archivo
para poder recuperar el contenido original de datos codificados que recibo en logs,
APIs, o mensajes.

**Why this priority**: Es la funcionalidad core del producto. Sin decodificación,
no hay valor. Representa el MVP mínimo viable.

**Independent Test**: Puede probarse completamente pasando un string Base64 conocido
y verificando que el archivo resultante coincide byte a byte con el original.

**Acceptance Scenarios**:

1. **Given** un string Base64 válido como argumento, **When** ejecuto `decodeck decode <base64-string>`, **Then** el sistema guarda el contenido decodificado en un archivo temporal y muestra la ruta.

2. **Given** un archivo que contiene datos Base64, **When** ejecuto `decodeck decode --file <ruta>`, **Then** el sistema lee el archivo, decodifica el contenido y guarda el resultado.

3. **Given** datos Base64 en stdin (pipe), **When** ejecuto `echo <base64> | decodeck decode`, **Then** el sistema lee de stdin, decodifica y guarda el resultado.

4. **Given** un string Base64 inválido, **When** intento decodificarlo, **Then** el sistema muestra un mensaje de error claro indicando el problema de formato.

---

### User Story 2 - Mostrar Metadatos del Contenido (Priority: P2)

Como usuario, quiero ver información útil sobre el contenido decodificado (tamaño,
tipo MIME, extensión sugerida) para entender qué contienen los datos sin necesidad
de abrir el archivo.

**Why this priority**: Complementa la decodificación con información de contexto.
Permite tomar decisiones informadas antes de abrir archivos desconocidos.

**Independent Test**: Puede probarse decodificando archivos de tipos conocidos
(PNG, PDF, MP3) y verificando que los metadatos reportados son correctos.

**Acceptance Scenarios**:

1. **Given** datos Base64 de una imagen PNG, **When** los decodifico, **Then** el sistema muestra: tamaño en bytes/KB/MB, MIME type "image/png", extensión sugerida ".png".

2. **Given** datos Base64 de un PDF, **When** los decodifico, **Then** el sistema detecta el tipo mediante magic bytes y muestra el MIME type correcto.

3. **Given** datos Base64 de tipo desconocido, **When** los decodifico, **Then** el sistema muestra "application/octet-stream" y extensión ".bin".

4. **Given** la flag `--json`, **When** decodifico cualquier contenido, **Then** los metadatos se muestran en formato JSON estructurado.

---

### User Story 3 - Visualización Interactiva (Priority: P3)

Como usuario, quiero poder visualizar o reproducir el contenido decodificado
directamente desde la terminal presionando una tecla, para inspeccionar rápidamente
el contenido sin buscar el archivo manualmente.

**Why this priority**: Es una mejora de experiencia de usuario que agiliza el
workflow, pero no es esencial para la funcionalidad core.

**Independent Test**: Puede probarse decodificando una imagen, verificando que
aparece el prompt "Press space to view", y que al presionar se abre el visor nativo.

**Acceptance Scenarios**:

1. **Given** contenido decodificado de tipo visualizable (imagen, PDF), **When** la decodificación termina, **Then** el sistema muestra "Press space to view" y espera input.

2. **Given** contenido decodificado de tipo reproducible (audio, video), **When** la decodificación termina, **Then** el sistema muestra "Press space to play" y espera input.

3. **Given** el prompt interactivo visible, **When** presiono espacio, **Then** el sistema abre el archivo con la aplicación nativa del sistema operativo.

4. **Given** el prompt interactivo visible, **When** presiono cualquier otra tecla o Enter, **Then** el sistema termina sin abrir el archivo.

5. **Given** ejecución en modo pipe o con `--quiet`, **When** hay contenido visualizable, **Then** NO se muestra el prompt interactivo (se omite automáticamente).

---

### User Story 4 - Salida a Ruta Específica (Priority: P4)

Como usuario, quiero poder especificar la ruta de destino para el archivo
decodificado, para organizar mis archivos según mis necesidades.

**Why this priority**: Mejora de usabilidad que permite integración en workflows
más complejos, pero el default de archivo temporal es suficiente para el MVP.

**Independent Test**: Puede probarse especificando una ruta y verificando que el
archivo se crea exactamente en esa ubicación.

**Acceptance Scenarios**:

1. **Given** la flag `--output <ruta>`, **When** decodifico contenido, **Then** el archivo se guarda en la ruta especificada.

2. **Given** la flag `--output` con una ruta a directorio inexistente, **When** decodifico, **Then** el sistema muestra error indicando que el directorio no existe.

3. **Given** la flag `--output` con un archivo existente, **When** decodifico, **Then** el sistema pregunta confirmación antes de sobrescribir (o usa `--force` para omitir).

---

### Edge Cases

- **Base64 con padding incorrecto**: El sistema intenta corregir padding faltante (agregar '=' necesarios) antes de fallar.
- **Archivos muy grandes**: El sistema procesa en streaming para evitar cargar todo en memoria. Límite configurable (default: 100MB).
- **Caracteres inválidos en Base64**: El sistema reporta la posición del primer carácter inválido encontrado.
- **Entrada vacía**: El sistema muestra error claro "No input data provided".
- **Permisos de escritura**: Si no hay permisos para escribir en la ruta destino, mostrar error descriptivo.
- **Stdin sin datos (timeout)**: Si stdin está vacío después de un timeout razonable (5 segundos), mostrar mensaje de ayuda.

## Requirements *(mandatory)*

### Functional Requirements

**Entrada de Datos**

- **FR-001**: El sistema DEBE aceptar datos Base64 como argumento de línea de comandos.
- **FR-002**: El sistema DEBE aceptar datos Base64 desde un archivo especificado con `--file`.
- **FR-003**: El sistema DEBE aceptar datos Base64 desde stdin (para uso en pipes).
- **FR-004**: El sistema DEBE detectar automáticamente la fuente de entrada (argumento > archivo > stdin).

**Decodificación**

- **FR-005**: El sistema DEBE decodificar Base64 estándar (RFC 4648).
- **FR-006**: El sistema DEBE soportar Base64 URL-safe (RFC 4648 §5) automáticamente.
- **FR-007**: El sistema DEBE manejar Base64 con o sin padding ('=').
- **FR-008**: El sistema DEBE ignorar whitespace (espacios, newlines, tabs) en la entrada Base64.

**Metadatos**

- **FR-009**: El sistema DEBE mostrar el tamaño del contenido decodificado en formato human-readable.
- **FR-010**: El sistema DEBE detectar el tipo MIME mediante análisis de magic bytes.
- **FR-011**: El sistema DEBE sugerir una extensión de archivo basada en el tipo MIME detectado.
- **FR-012**: El sistema DEBE mostrar la codificación detectada (standard vs URL-safe).

**Salida**

- **FR-013**: El sistema DEBE guardar el contenido decodificado en un archivo.
- **FR-014**: El sistema DEBE usar un directorio temporal por defecto si no se especifica `--output`.
- **FR-015**: El sistema DEBE soportar salida en formato texto legible (default) y JSON (`--json`).
- **FR-016**: El sistema DEBE retornar código de salida 0 en éxito, 1 en error de usuario, 2 en error de sistema.

**Interactividad**

- **FR-017**: El sistema DEBE mostrar prompt "Press space to view|play" para contenido visualizable/reproducible.
- **FR-018**: El sistema DEBE abrir el archivo con la aplicación nativa del SO al presionar espacio.
- **FR-019**: El sistema DEBE omitir el modo interactivo cuando detecta ejecución en pipe o con `--quiet`.
- **FR-020**: El sistema DEBE permitir desactivar interactividad con `--no-interactive`.

**Seguridad**

- **FR-021**: El sistema DEBE validar que los datos de entrada no excedan el límite de tamaño configurado.
- **FR-022**: El sistema DEBE sanitizar rutas de salida para prevenir path traversal.
- **FR-023**: El sistema NO DEBE ejecutar contenido decodificado automáticamente sin acción explícita del usuario.

### Key Entities

- **EncodedData**: Representa los datos de entrada en formato Base64. Atributos: contenido raw, fuente (arg/file/stdin), variante (standard/url-safe), tiene padding.

- **DecodedContent**: Representa el resultado de la decodificación. Atributos: bytes decodificados, tamaño, hash para verificación.

- **ContentMetadata**: Información sobre el contenido decodificado. Atributos: MIME type, extensión sugerida, es visualizable, es reproducible, tamaño formateado.

- **OutputFile**: Representa el archivo de salida. Atributos: ruta, es temporal, permisos.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: El usuario puede decodificar un archivo Base64 de 1MB en menos de 2 segundos.

- **SC-002**: El sistema detecta correctamente el tipo MIME del 95% de los formatos comunes (imágenes, documentos, audio, video).

- **SC-003**: El usuario puede completar el flujo completo (decodificar + visualizar) en menos de 5 segundos para archivos menores a 10MB.

- **SC-004**: El 100% de los mensajes de error incluyen información suficiente para que el usuario entienda qué salió mal y cómo corregirlo.

- **SC-005**: La salida JSON es parseable por herramientas estándar (jq, scripts) sin errores.

- **SC-006**: El sistema funciona correctamente en los 3 modos de entrada (argumento, archivo, stdin) sin diferencias de comportamiento.

## Assumptions

- El usuario tiene permisos de escritura en el directorio temporal del sistema.
- El sistema operativo tiene aplicaciones nativas asociadas a los tipos de archivo comunes.
- Los datos Base64 de entrada caben en memoria para validación inicial (el procesamiento puede ser streaming).
- El límite de tamaño por defecto de 100MB es apropiado para la mayoría de casos de uso.
