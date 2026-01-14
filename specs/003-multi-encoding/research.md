# Research: Multi-Encoding Support

## Crate Selection

### Hex Encoding

#### Decision
Usar `data-encoding` crate para decodificación hexadecimal.

#### Rationale
- Crate maduro y bien mantenido (parte del ecosistema de Cloudflare)
- Soporta múltiples encodings (hex, base32) en un solo crate
- API consistente y eficiente
- Sin dependencias transitivas problemáticas
- Permite configuración flexible (mayúsculas/minúsculas, con/sin espacios)

#### Alternatives Considered
1. **hex crate**: Rechazado - Menos mantenido, API más limitada
2. **Implementación manual**: Rechazado - Reinventar la rueda, propenso a errores

### Base32 Encoding

#### Decision
Usar `data-encoding` crate para Base32.

#### Rationale
- Mismo crate que hex, reduce dependencias
- Soporta RFC 4648 Base32 estándar
- Case-insensitive por configuración
- Manejo de padding automático

### URL Encoding

#### Decision
Usar `percent-encoding` crate (parte de servo/rust-url).

#### Rationale
- Crate oficial del proyecto Servo
- Estándar de facto en el ecosistema Rust
- Cumple con RFC 3986
- API simple y directa

#### Alternatives Considered
1. **urlencoding crate**: Rechazado - Menos mantenido que percent-encoding
2. **Implementación manual**: Rechazado - Edge cases complicados (UTF-8, etc.)

### Base85/Ascii85 Encoding

#### Decision
Usar `base85` crate para decodificación Ascii85.

#### Rationale
- Única implementación viable en el ecosistema Rust
- Soporta formato Ascii85 estándar (con delimitadores <~ ~>)
- Opcional: soporte para Z85 (variante ZeroMQ)

#### Note
- Z85 se implementará como variante opcional si hay demanda
- Prioridad baja (P4 en spec)

## Architecture Decisions

### Trait-Based Design

#### Decision
Crear trait `Decoder` que todos los encodings implementen.

#### Pattern
```rust
pub trait Decoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>, DecodeckError>;
    fn name(&self) -> &'static str;
    fn can_decode(&self, input: &str) -> bool;
}
```

#### Rationale
- Permite polimorfismo para auto-detección
- Facilita añadir nuevos encodings en el futuro
- Sigue Principio I (Biblioteca-First)
- Permite testing independiente de cada encoding

### Auto-Detection Heuristics

#### Decision
Implementar detección basada en características del input.

#### Algorithm (en orden de prioridad)
1. **Prefijo `0x`**: → Hex (100% confianza)
2. **Delimitadores `<~` y `~>`**: → Base85 Ascii85 (100% confianza)
3. **Patrón `%XX`**: → URL encoding (alta confianza)
4. **Solo caracteres hex + longitud par**: → Hex (media confianza)
5. **Solo caracteres Base32 (`A-Z2-7=`)**: → Base32 (media confianza)
6. **Default**: → Base64

#### Rationale
- Priorizar detección determinista (prefijos, delimitadores)
- Casos ambiguos defaultean a Base64 con warning
- Usuario siempre puede especificar `--encoding` explícitamente

### Backwards Compatibility

#### Decision
Mantener comportamiento actual cuando no se especifica `--encoding`.

#### Rationale
- Scripts existentes que usan decodeck no deben romperse
- Base64 sigue siendo el default implícito
- Nuevas features son opt-in via `--encoding`

## Testing Strategy

### Unit Tests per Encoding

Para cada encoding:
1. Decodificación de casos válidos
2. Manejo de variantes (mayúsculas/minúsculas, con/sin padding)
3. Errores claros para input inválido
4. Edge cases específicos del formato

### Integration Tests

1. Flag `--encoding` funciona para cada tipo
2. Auto-detección identifica correctamente cada formato
3. Salida JSON incluye campo "encoding"
4. Error messages son útiles y específicos

### Property Tests

1. Roundtrip: encode → decode = original (donde aplique)
2. Nunca panic en input arbitrario
3. Performance consistente con diferentes tamaños

## Performance Considerations

### Decision
No optimizar prematuramente, medir primero.

### Baseline
- Decodificación Base64 actual: ~100ms para 10MB
- Target: <10% overhead por encoding adicional

### Approach
1. Implementar funcionalidad correcta primero
2. Medir performance en CI
3. Optimizar solo si hay regresión significativa
