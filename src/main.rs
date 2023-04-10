use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use model::*;
mod model;

fn main() -> Result<()> {
    let cli = model::Cli::parse();

    match cli.command {
        Commands::Mix {
            inputs,
            output,
            force,
        } => {
            let mut inputs_f: Vec<BufReader<File>> = Vec::with_capacity(inputs.len());

            for i in inputs {
                inputs_f.push(BufReader::new(File::open(&i).with_context(|| {
                    format!("Failed to open input file {}", i.to_string_lossy())
                })?));
            }

            let mut output_f = File::options()
                .create_new(!force)
                .create(force)
                .truncate(force)
                .write(true)
                .open(&output)
                .with_context(|| "Failed to open output file")?;

            let mut buf = vec![0; inputs_f.len()];

            'write_loop: loop {
                for (b, f) in buf.iter_mut().zip(inputs_f.iter_mut()) {
                    let mut buf = [0; 1];

                    if f.read(&mut buf).with_context(|| "Failed to read input")? == 0 {
                        break 'write_loop;
                    }

                    *b = buf[0];
                }

                output_f
                    .write(&buf)
                    .with_context(|| "Failed to write output")?;
            }
        }
        Commands::Split {
            input,
            outputs,
            force,
        } => {
            let mut output_fs: Vec<File> = Vec::with_capacity(outputs.len());
            for o in outputs {
                output_fs.push(
                    File::options()
                        .create_new(!force)
                        .create(force)
                        .truncate(force)
                        .write(true)
                        .open(&o)
                        .with_context(|| {
                            format!("Failed to open output file {}", o.to_string_lossy())
                        })?,
                );
            }

            let mut input_f = BufReader::new(File::open(&input).with_context(|| {
                format!("Failed to open input file {}", input.to_string_lossy())
            })?);

            let mut buf = vec![0; output_fs.len()];

            loop {
                if let Err(_) = input_f.read_exact(&mut buf) {
                    break;
                };

                for (b, mut f) in buf.iter().zip(output_fs.iter()) {
                    f.write(&[*b])
                        .with_context(|| format!("Failed to write file"))?;
                }
            }
        }
    }

    Ok(())
}
