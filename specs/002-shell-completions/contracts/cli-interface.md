# CLI Interface Contract: completions

## Command Signature

```
decodeck completions <SHELL>
```

## Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| SHELL | enum | Yes | Target shell: bash, zsh, fish, powershell |

## Options

| Option | Short | Description |
|--------|-------|-------------|
| --help | -h | Show help with installation examples |

## Output

### Success (exit code 0)
- Stdout: Shell completion script (text)
- Stderr: Empty

### Error (exit code 1)
- Shell not specified or invalid
- Stderr: Error message with valid shells list

## Examples

```bash
# Generate bash completions
decodeck completions bash

# Generate and install for bash
decodeck completions bash >> ~/.bashrc

# Generate for zsh
decodeck completions zsh > ~/.zfunc/_decodeck

# Generate for fish
decodeck completions fish > ~/.config/fish/completions/decodeck.fish

# Generate for PowerShell
decodeck completions powershell >> $PROFILE
```

## Help Text

```
Generate shell completion scripts

Usage: decodeck completions <SHELL>

Arguments:
  <SHELL>  Target shell [possible values: bash, zsh, fish, powershell]

Options:
  -h, --help  Print help

Examples:
  # Bash - add to ~/.bashrc
  decodeck completions bash >> ~/.bashrc

  # Zsh - add to fpath
  decodeck completions zsh > ~/.zfunc/_decodeck

  # Fish - save to completions dir
  decodeck completions fish > ~/.config/fish/completions/decodeck.fish

  # PowerShell - add to profile
  decodeck completions powershell >> $PROFILE
```
