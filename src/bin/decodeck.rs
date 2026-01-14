//! Decodeck CLI - Multi-encoding decoder with metadata display and interactive viewing

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use decodeck::decoder::EncodedData;
use decodeck::encoding::{detect::detect_encoding, EncodingInfo, EncodingType};
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
#[command(author, version, about = "Decode encoded data (Base64, Hex, Base32, URL, Base85) to files with metadata display", long_about = None)]
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

        /// Output file path (default: temporary file)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Encoding type (auto-detected if not specified)
        #[arg(short, long, value_enum)]
        encoding: Option<EncodingType>,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,

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
            output,
            encoding,
            json,
            no_interactive,
            force,
            max_size,
        } => run_decode(
            data,
            file,
            output,
            encoding,
            json,
            no_interactive,
            force,
            cli.quiet,
            max_size,
        ),
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
    output: Option<PathBuf>,
    encoding: Option<EncodingType>,
    json: bool,
    no_interactive: bool,
    force: bool,
    quiet: bool,
    max_size: String,
) -> Result<()> {
    let start = Instant::now();

    // Get input source (priority: argument > file > stdin)
    let input = get_input(data, file)?;

    // Validate input size
    input.validate_size(&max_size)?;

    // Get input as string
    let input_str =
        String::from_utf8(input.raw_data.clone()).context("Input is not valid UTF-8")?;

    // Determine encoding (explicit or auto-detect)
    let encoding_info = if let Some(enc_type) = encoding {
        EncodingInfo::explicit(enc_type)
    } else {
        detect_encoding(&input_str)
    };

    // Decode based on encoding type
    let (decoded, legacy_encoded) = if encoding_info.encoding_type == EncodingType::Base64 {
        // Use existing Base64 decoder for backwards compatibility
        let encoded = EncodedData::parse(&input_str)?;
        let decoded = encoded.decode()?;
        (decoded, Some(encoded))
    } else {
        // Use new multi-encoding decoder
        let decoder = encoding_info.encoding_type.decoder();
        let decoded = decoder.decode(&input_str)?;
        (decoded, None)
    };

    // Detect content metadata
    let metadata = magic::detect(&decoded);

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

fn get_input(data: Option<String>, file: Option<PathBuf>) -> Result<InputSource> {
    // Priority: argument > file > stdin
    if let Some(arg_data) = data {
        return InputSource::from_arg(&arg_data).map_err(|e| e.into());
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
