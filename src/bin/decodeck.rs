//! Decodeck CLI - Multi-encoding decoder with metadata display and interactive viewing

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use decodeck::decoder::EncodedData;
use decodeck::encoding::{detect::detect_encoding, scan, EncodingInfo, EncodingType};

/// Format for scanning structured content
#[derive(Debug, Clone, Copy, ValueEnum)]
enum ScanFormat {
    /// JSON format
    Json,
    /// XML format
    Xml,
}
use decodeck::error::{exit_codes, DecodeckError};
use decodeck::input::{InputSource, SourceType};
use decodeck::interactive::InteractivePrompt;
use decodeck::metadata::magic;
use decodeck::output::{format_size, DecodeResult, OutputFile};
use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "decodeck")]
#[command(author, version, about = "Encode and decode data (Base64, Hex, Base32, URL, Base85)", long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Decode encoded data to file
    Decode {
        /// Encoded string to decode
        data: Option<String>,

        /// Read encoded data from file
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Read encoded data from clipboard
        #[arg(long)]
        clipboard: bool,

        /// Output file path (default: temporary file)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Encoding type (auto-detected if not specified)
        #[arg(short, long, value_enum)]
        encoding: Option<EncodingType>,

        /// Decode nested/chained encodings recursively
        #[arg(short = 'c', long)]
        chain: bool,

        /// Maximum chain depth (default: 10)
        #[arg(long, default_value = "10")]
        max_depth: usize,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,

        /// Raw output mode - write decoded bytes to stdout (for piping)
        #[arg(short = 'r', long)]
        raw: bool,

        /// Skip interactive view/play prompt
        #[arg(long)]
        no_interactive: bool,

        /// Force overwrite existing files
        #[arg(short = 'F', long)]
        force: bool,

        /// Maximum input size (e.g., "100MB")
        #[arg(long, default_value = "100MB")]
        max_size: String,
    },
    /// Encode data to specified format
    Encode {
        /// Data string to encode (or use --file)
        data: Option<String>,

        /// Read data from file
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Read data from clipboard
        #[arg(long)]
        clipboard: bool,

        /// Output encoding format
        #[arg(short, long, value_enum, default_value = "base64")]
        encoding: EncodingType,

        /// Copy result to clipboard
        #[arg(long)]
        copy: bool,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },
    /// Scan JSON/XML for encoded content
    Scan {
        /// JSON/XML content to scan (or use --file)
        data: Option<String>,

        /// Read from file
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Read from clipboard
        #[arg(long)]
        clipboard: bool,

        /// Format hint (auto-detected if not specified)
        #[arg(long, value_enum)]
        format: Option<ScanFormat>,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },
    /// Generate shell completion scripts
    #[command(after_help = r#"INSTALLATION EXAMPLES:
  # Bash - add to ~/.bashrc
  decodeck completions bash >> ~/.bashrc

  # Zsh - add to fpath
  decodeck completions zsh > ~/.zfunc/_decodeck

  # Fish - save to completions dir
  decodeck completions fish > ~/.config/fish/completions/decodeck.fish

  # PowerShell - add to profile
  decodeck completions powershell >> $PROFILE"#)]
    Completions {
        /// Target shell
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if cli.verbose {
        decodeck::init_logging(true);
    }

    let result = match cli.command {
        Commands::Decode {
            data,
            file,
            clipboard,
            output,
            encoding,
            chain,
            max_depth,
            json,
            raw,
            no_interactive,
            force,
            max_size,
        } => run_decode(
            data,
            file,
            clipboard,
            output,
            encoding,
            chain,
            max_depth,
            json,
            raw,
            no_interactive,
            force,
            cli.quiet,
            max_size,
        ),
        Commands::Encode {
            data,
            file,
            clipboard,
            encoding,
            copy,
            json,
        } => run_encode(data, file, clipboard, encoding, copy, json, cli.quiet),
        Commands::Scan {
            data,
            file,
            clipboard,
            format,
            json,
        } => run_scan(data, file, clipboard, format, json, cli.quiet),
        Commands::Completions { shell } => {
            run_completions(shell);
            Ok(())
        }
    };

    match result {
        Ok(_) => ExitCode::from(exit_codes::SUCCESS as u8),
        Err(e) => {
            eprintln!("Error: {}", e);
            if e.downcast_ref::<DecodeckError>().is_some() {
                ExitCode::from(exit_codes::USER_ERROR as u8)
            } else {
                ExitCode::from(exit_codes::SYSTEM_ERROR as u8)
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn run_decode(
    data: Option<String>,
    file: Option<PathBuf>,
    clipboard: bool,
    output: Option<PathBuf>,
    encoding: Option<EncodingType>,
    chain: bool,
    max_depth: usize,
    json: bool,
    raw: bool,
    no_interactive: bool,
    force: bool,
    quiet: bool,
    max_size: String,
) -> Result<()> {
    let start = Instant::now();

    // Get input source (priority: argument > clipboard > file > stdin)
    let input = get_input(data, file, clipboard)?;

    // Validate input size
    input.validate_size(&max_size)?;

    // Get input as string
    let input_str =
        String::from_utf8(input.raw_data.clone()).context("Input is not valid UTF-8")?;

    // Decode - either chain mode or single
    let (decoded, encoding_info, legacy_encoded, chain_info) = if chain {
        // Chain decoding mode
        let result = decodeck::encoding::chain::decode_chain(&input_str, Some(max_depth))?;
        let last_encoding = result
            .chain
            .last()
            .cloned()
            .unwrap_or_else(|| EncodingInfo::explicit(EncodingType::Base64));
        (result.data, last_encoding, None, Some(result.chain))
    } else {
        // Single encoding mode
        let encoding_info = if let Some(enc_type) = encoding {
            EncodingInfo::explicit(enc_type)
        } else {
            detect_encoding(&input_str)
        };

        let (decoded, legacy_encoded) = if encoding_info.encoding_type == EncodingType::Base64 {
            let encoded = EncodedData::parse(&input_str)?;
            let decoded = encoded.decode()?;
            (decoded, Some(encoded))
        } else {
            let decoder = encoding_info.encoding_type.decoder();
            let decoded = decoder.decode(&input_str)?;
            (decoded, None)
        };

        (decoded, encoding_info, legacy_encoded, None)
    };

    // Raw mode - write directly to stdout and exit
    if raw {
        use std::io::Write;
        io::stdout().write_all(&decoded)?;
        io::stdout().flush()?;
        return Ok(());
    }

    // Detect content metadata
    let metadata = magic::detect(&decoded);

    // Check for stdout output (output="-")
    let write_to_stdout = output.as_ref().map(|p| p.as_os_str() == "-").unwrap_or(false);

    if write_to_stdout {
        use std::io::Write;
        io::stdout().write_all(&decoded)?;
        io::stdout().flush()?;
        return Ok(());
    }

    // Determine output path
    let is_temp = output.is_none();
    let output_path = if let Some(path) = output {
        // Check if file exists and handle overwrite
        if path.exists() && !force {
            return Err(DecodeckError::OutputExists {
                path: path.display().to_string(),
            }
            .into());
        }
        path
    } else {
        // Create temporary file with appropriate extension
        let temp_dir = std::env::temp_dir().join("decodeck");
        fs::create_dir_all(&temp_dir)?;
        let filename = format!("output{}", metadata.extension);
        temp_dir.join(filename)
    };

    // Write output file
    fs::write(&output_path, &decoded)?;

    let duration = start.elapsed();

    // Build result
    let result = DecodeResult {
        success: true,
        output: OutputFile {
            path: output_path.clone(),
            is_temporary: is_temp,
            size_bytes: decoded.len(),
            size_formatted: format_size(decoded.len()),
            created_at: Some(std::time::SystemTime::now()),
        },
        metadata: metadata.clone(),
        encoding: legacy_encoded,
        encoding_info: encoding_info.clone(),
        duration_ms: duration.as_millis() as u64,
        warnings: vec![],
    };

    // Output result
    if json {
        decodeck::output::json::format(&result, &mut io::stdout())?;
    } else if !quiet {
        decodeck::output::text::format(&result, &mut io::stdout())?;

        // Show chain info if available
        if let Some(ref chain) = chain_info {
            if chain.len() > 1 {
                let chain_str: Vec<_> = chain.iter().map(|e| e.encoding_type.to_string()).collect();
                println!("Chain: {} (depth: {})", chain_str.join(" → "), chain.len());
            }
        }
    }

    // Interactive prompt for viewable/playable content
    let prompt = InteractivePrompt::new(quiet, no_interactive);
    if prompt.show_and_wait(&metadata) {
        if let Err(e) = InteractivePrompt::open_file(&output_path) {
            if !quiet {
                eprintln!("Warning: Could not open file: {}", e);
            }
        }
    }

    Ok(())
}

fn run_completions(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "decodeck", &mut io::stdout());
}

fn get_input(data: Option<String>, file: Option<PathBuf>, clipboard: bool) -> Result<InputSource> {
    // Priority: argument > clipboard > file > stdin
    if let Some(arg_data) = data {
        return InputSource::from_arg(&arg_data).map_err(|e| e.into());
    }

    if clipboard {
        let mut cb = arboard::Clipboard::new().context("Failed to access clipboard")?;
        let text = cb.get_text().context("Failed to read clipboard")?;
        if text.is_empty() {
            return Err(DecodeckError::NoInput.into());
        }
        return Ok(InputSource::new(SourceType::Stdin, text.into_bytes(), None));
    }

    if let Some(file_path) = file {
        return InputSource::from_file(&file_path).map_err(|e| e.into());
    }

    // Try stdin if not a terminal
    if !io::stdin().is_terminal() {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        if buffer.is_empty() {
            return Err(DecodeckError::NoInput.into());
        }
        return Ok(InputSource::new(SourceType::Stdin, buffer, None));
    }

    Err(DecodeckError::NoInput.into())
}

fn run_encode(
    data: Option<String>,
    file: Option<PathBuf>,
    clipboard: bool,
    encoding: EncodingType,
    copy: bool,
    json: bool,
    quiet: bool,
) -> Result<()> {
    // Get input data (priority: argument > clipboard > file > stdin)
    let input_bytes = if let Some(arg_data) = data {
        arg_data.into_bytes()
    } else if clipboard {
        let mut cb = arboard::Clipboard::new().context("Failed to access clipboard")?;
        cb.get_text()
            .context("Failed to read clipboard")?
            .into_bytes()
    } else if let Some(file_path) = file {
        fs::read(&file_path).context("Failed to read input file")?
    } else if !io::stdin().is_terminal() {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        buffer
    } else {
        return Err(DecodeckError::NoInput.into());
    };

    if input_bytes.is_empty() {
        return Err(DecodeckError::NoInput.into());
    }

    // Encode the data
    let encoded = decodeck::encoding::encode::encode(&input_bytes, encoding)?;

    // Copy to clipboard if requested
    if copy {
        let mut cb = arboard::Clipboard::new().context("Failed to access clipboard")?;
        cb.set_text(&encoded)
            .context("Failed to copy to clipboard")?;
        if !quiet && !json {
            eprintln!("Copied to clipboard");
        }
    }

    // Output
    if json {
        let output = serde_json::json!({
            "success": true,
            "encoding": encoding.display_name(),
            "input_size": input_bytes.len(),
            "output_size": encoded.len(),
            "encoded": encoded,
            "copied_to_clipboard": copy
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else if quiet {
        print!("{}", encoded);
    } else {
        println!("{}", encoded);
    }

    Ok(())
}

fn run_scan(
    data: Option<String>,
    file: Option<PathBuf>,
    clipboard: bool,
    format: Option<ScanFormat>,
    json: bool,
    quiet: bool,
) -> Result<()> {
    // Get input
    let input = get_input(data, file, clipboard)?;
    let input_str =
        String::from_utf8(input.raw_data.clone()).context("Input is not valid UTF-8")?;

    // Scan based on format
    let result = match format {
        Some(ScanFormat::Json) => scan::scan_json(&input_str)?,
        Some(ScanFormat::Xml) => scan::scan_xml(&input_str)?,
        None => scan::scan_auto(&input_str)?,
    };

    // Output
    if json {
        let output = serde_json::json!({
            "success": true,
            "format": result.format,
            "values_scanned": result.values_scanned,
            "findings_count": result.findings.len(),
            "findings": result.findings.iter().map(|f| {
                serde_json::json!({
                    "path": f.path,
                    "encoding": f.encoding.display_name(),
                    "confidence": format!("{:?}", f.confidence).to_lowercase(),
                    "original": f.original,
                    "decoded": f.decoded,
                    "is_text": f.is_text
                })
            }).collect::<Vec<_>>()
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        if !quiet {
            println!(
                "Scanned {} values in {} format",
                result.values_scanned, result.format
            );
            println!("Found {} encoded values:\n", result.findings.len());
        }

        for finding in &result.findings {
            println!("📍 {}", finding.path);
            println!(
                "   Encoding: {} ({:?})",
                finding.encoding.display_name(),
                finding.confidence
            );
            println!("   Original: {}", truncate_string(&finding.original, 60));
            println!(
                "   Decoded:  {}",
                truncate_string(&finding.decoded, 60)
            );
            println!();
        }

        if result.findings.is_empty() && !quiet {
            println!("No encoded content found.");
        }
    }

    Ok(())
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
