# Research: Shell Completions

## clap_complete Integration

### Decision
Usar `clap_complete` crate para generación de shell completions.

### Rationale
- Es el crate oficial del ecosistema clap para completions
- Soporta los 4 shells requeridos: Bash, Zsh, Fish, PowerShell
- Se integra directamente con la definición de CLI existente
- Genera scripts optimizados para cada shell
- Mantenido activamente junto con clap

### Alternatives Considered
1. **Generación manual de scripts**: Rechazado - Mucho más código, propenso a errores, difícil de mantener
2. **Shell completions dinámicas**: Rechazado - Requiere ejecución del binario para cada completado, más lento

## Implementation Approach

### Decision
Añadir subcomando `completions` que acepta shell como argumento y usa clap_complete::generate().

### Code Pattern
```rust
use clap_complete::{generate, Shell};

#[derive(Subcommand)]
enum Commands {
    Decode { /* existing */ },
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

// En el handler:
fn run_completions(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "decodeck", &mut std::io::stdout());
}
```

### Rationale
- Patrón estándar usado por la mayoría de CLIs Rust
- Mínimo código adicional (~10 líneas)
- Shell enum viene de clap_complete, incluye los 4 shells
- Salida a stdout permite `decodeck completions bash > ~/.bash_completion.d/decodeck`

## Shell-Specific Notes

### Bash
- Script usa `complete -F` para registrar función de completado
- Compatible con bash 4.0+
- Instalación: `source <(decodeck completions bash)` o guardar en archivo

### Zsh
- Genera función `_decodeck` para sistema de completado de zsh
- Incluye descripciones de opciones automáticamente
- Instalación: guardar en `$fpath` o `~/.zfunc/_decodeck`

### Fish
- Genera comandos `complete -c decodeck ...`
- Fish carga automáticamente desde `~/.config/fish/completions/`
- Formato más legible que bash/zsh

### PowerShell
- Usa `Register-ArgumentCompleter`
- Compatible con PowerShell 5.1+ y PowerShell Core
- Añadir a `$PROFILE` para carga automática

## Dependencies

### Decision
Añadir `clap_complete = "4.4"` a Cargo.toml

### Rationale
- Misma versión major que clap existente (4.4)
- Sin dependencias transitivas adicionales significativas
- ~50KB de código adicional compilado

## Testing Strategy

### Decision
Tests de integración que verifican:
1. Comando genera salida no vacía para cada shell
2. Salida contiene keywords esperados del shell target
3. Comando falla con shell inválido

### Rationale
- No es práctico evaluar el script en cada shell en CI
- Verificar estructura básica es suficiente para detectar regresiones
- Tests manuales de funcionalidad real en shells locales
