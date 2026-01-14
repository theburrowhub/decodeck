# Quickstart: Shell Completions

## Installation

### Bash

```bash
# Option 1: Add to .bashrc (loads on every new shell)
echo 'eval "$(decodeck completions bash)"' >> ~/.bashrc

# Option 2: Save to file (faster startup)
decodeck completions bash > ~/.local/share/bash-completion/completions/decodeck

# Reload
source ~/.bashrc
```

### Zsh

```bash
# Create completions directory if needed
mkdir -p ~/.zfunc

# Generate completion file
decodeck completions zsh > ~/.zfunc/_decodeck

# Add to .zshrc (before compinit)
echo 'fpath=(~/.zfunc $fpath)' >> ~/.zshrc
echo 'autoload -Uz compinit && compinit' >> ~/.zshrc

# Reload
source ~/.zshrc
```

### Fish

```bash
# Fish auto-loads from completions directory
decodeck completions fish > ~/.config/fish/completions/decodeck.fish

# Takes effect immediately in new shells
```

### PowerShell

```powershell
# Add to profile (loads on every new session)
decodeck completions powershell >> $PROFILE

# Reload profile
. $PROFILE
```

## Verification

After installation, test the completions:

```bash
# Type and press TAB
decodeck <TAB>
# Should show: completions  decode

decodeck decode --<TAB>
# Should show: --file  --force  --json  --max-size  --no-interactive  --output  --quiet  --verbose
```

## Troubleshooting

### Bash: Completions not working
- Ensure `bash-completion` package is installed
- Check that `.bashrc` is sourced (not `.bash_profile` only)

### Zsh: Completions not showing
- Run `compinit` after adding to fpath
- Delete `~/.zcompdump` and restart shell

### Fish: No suggestions
- Verify file exists: `ls ~/.config/fish/completions/decodeck.fish`
- Check for syntax errors: `fish -c "source ~/.config/fish/completions/decodeck.fish"`

### PowerShell: Not recognized
- Ensure profile path exists: `New-Item -Path $PROFILE -ItemType File -Force`
- Check execution policy: `Set-ExecutionPolicy RemoteSigned -Scope CurrentUser`
