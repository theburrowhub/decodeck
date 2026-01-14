# Feature Specification: Multi-Encoding Support

**Feature Branch**: `003-multi-encoding`
**Created**: 2026-01-14
**Status**: Draft
**Input**: User description: "Añadir soporte para múltiples tipos de codificación de entrada y salida: hex, base32, base85, URL encoding"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Decode Hexadecimal Data (Priority: P1)

Como usuario, quiero decodificar datos en formato hexadecimal para poder trabajar con datos binarios representados en hex (común en logs, dumps de memoria, hashes).

**Why this priority**: Hex es el segundo formato de codificación más común después de base64, especialmente en contextos de debugging y seguridad.

**Independent Test**: Ejecutar `decodeck decode --encoding hex "48656c6c6f"` y verificar que produce "Hello".

**Acceptance Scenarios**:

1. **Given** datos codificados en hexadecimal, **When** ejecuto `decodeck decode --encoding hex "48656c6c6f"`, **Then** se decodifica correctamente a "Hello"
2. **Given** hex con espacios o separadores, **When** ejecuto decode con "48 65 6c 6c 6f", **Then** se ignoran los espacios y decodifica correctamente
3. **Given** hex con prefijo 0x, **When** ejecuto decode con "0x48656c6c6f", **Then** se reconoce y decodifica correctamente

---

### User Story 2 - Decode Base32 Data (Priority: P2)

Como usuario, quiero decodificar datos en Base32 para trabajar con TOTP secrets y otros datos codificados en este formato.

**Why this priority**: Base32 es común en autenticación 2FA (TOTP/HOTP secrets) y en algunos sistemas de archivos.

**Independent Test**: Ejecutar `decodeck decode --encoding base32 "JBSWY3DPEHPK3PXP"` y verificar que produce "Hello!".

**Acceptance Scenarios**:

1. **Given** datos codificados en Base32, **When** ejecuto decode con --encoding base32, **Then** se decodifica correctamente
2. **Given** Base32 sin padding, **When** ejecuto decode, **Then** el sistema añade padding automáticamente
3. **Given** Base32 con minúsculas, **When** ejecuto decode, **Then** se acepta y decodifica (case-insensitive)

---

### User Story 3 - Decode URL-Encoded Data (Priority: P3)

Como usuario, quiero decodificar datos URL-encoded para poder leer parámetros de URLs y datos de formularios.

**Why this priority**: URL encoding es ubicuo en desarrollo web para query strings y form data.

**Independent Test**: Ejecutar `decodeck decode --encoding url "Hello%20World%21"` y verificar que produce "Hello World!".

**Acceptance Scenarios**:

1. **Given** texto URL-encoded, **When** ejecuto decode con --encoding url, **Then** se decodifica %XX a caracteres
2. **Given** texto con + para espacios, **When** ejecuto decode, **Then** los + se convierten a espacios
3. **Given** texto parcialmente encoded, **When** ejecuto decode, **Then** solo se decodifican las secuencias %XX

---

### User Story 4 - Decode Base85/Ascii85 Data (Priority: P4)

Como usuario, quiero decodificar datos en Base85/Ascii85 para trabajar con datos de Adobe PDF y PostScript.

**Why this priority**: Base85 es más eficiente que Base64 pero menos común. Usado en PDF y algunos sistemas especializados.

**Independent Test**: Ejecutar `decodeck decode --encoding base85 "<~87cURD]j7BEbo80~>"` y verificar decodificación correcta.

**Acceptance Scenarios**:

1. **Given** datos en formato Ascii85 (con delimitadores <~ ~>), **When** ejecuto decode, **Then** se decodifica correctamente
2. **Given** datos en Z85 (variante de Base85), **When** especifico --encoding z85, **Then** se usa el alfabeto Z85

---

### User Story 5 - Auto-Detect Encoding (Priority: P5)

Como usuario, quiero que el sistema detecte automáticamente el tipo de codificación cuando sea posible para no tener que especificarlo manualmente.

**Why this priority**: Mejora la experiencia de usuario al reducir fricción, pero requiere heurísticas que pueden fallar.

**Independent Test**: Ejecutar `decodeck decode "48656c6c6f"` sin --encoding y verificar que detecta hex automáticamente.

**Acceptance Scenarios**:

1. **Given** datos que claramente son hex (solo 0-9a-f), **When** ejecuto decode sin --encoding, **Then** se detecta como hex
2. **Given** datos ambiguos, **When** ejecuto decode, **Then** se usa base64 como default con warning
3. **Given** datos con prefijo 0x, **When** ejecuto decode, **Then** se detecta automáticamente como hex

---

### Edge Cases

- ¿Qué pasa con codificaciones inválidas? → Error claro indicando el problema y posición
- ¿Qué pasa si el usuario especifica encoding incorrecto? → Intento de decodificar falla con mensaje útil
- ¿Qué pasa con datos mixtos (parcialmente codificados)? → Solo decodificar la parte codificada en URL encoding
- ¿Qué pasa con caracteres no ASCII en URL encoding? → Soporte para UTF-8 encoded

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Sistema DEBE añadir flag `--encoding` / `-e` al comando decode
- **FR-002**: Sistema DEBE soportar los siguientes encodings: base64 (default), hex, base32, url, base85
- **FR-003**: Sistema DEBE auto-detectar encoding cuando no se especifica (con heurísticas)
- **FR-004**: Sistema DEBE mostrar el encoding detectado/usado en la salida
- **FR-005**: Hex DEBE aceptar con o sin prefijo 0x, con o sin espacios
- **FR-006**: Hex DEBE ser case-insensitive (acepta mayúsculas y minúsculas)
- **FR-007**: Base32 DEBE ser case-insensitive y auto-añadir padding
- **FR-008**: URL encoding DEBE soportar tanto %XX como + para espacios
- **FR-009**: Base85 DEBE soportar formato Ascii85 (con delimitadores <~ ~>)
- **FR-010**: Sistema DEBE mostrar warning cuando auto-detecta encoding ambiguo
- **FR-011**: La salida JSON DEBE incluir campo "encoding" con el tipo usado
- **FR-012**: Sistema DEBE validar que los datos son válidos para el encoding especificado

### Key Entities

- **Encoding**: Tipo de codificación (base64, hex, base32, url, base85)
- **EncodedData**: Datos de entrada con su encoding detectado o especificado
- **DecodedData**: Datos binarios resultantes de la decodificación

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Todos los encodings soportados decodifican correctamente el 100% de datos válidos
- **SC-002**: Auto-detección identifica correctamente el encoding en >90% de casos no ambiguos
- **SC-003**: El tiempo de decodificación no aumenta más de 10% comparado con base64-only
- **SC-004**: Los mensajes de error para encodings inválidos son específicos y accionables

## Assumptions

- Los encodings más comunes (base64, hex) tienen prioridad en auto-detección
- URL encoding solo decodifica (no se implementa encode en esta fase)
- Z85 es una variante opcional de Base85 que puede no implementarse en fase inicial
