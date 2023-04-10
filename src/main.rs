use anyhow::{Context, Result as AHResult};
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use model::*;
mod model;

fn mixer<R: Read, W: Write>(r: &mut [R], w: &mut W) -> Result<(), std::io::Error> {
    let mut buf = vec![0; r.len()];

    'write_loop: loop {
        for (buf_p, f) in buf.iter_mut().zip(r.iter_mut()) {
            let mut byte = [0];

            if f.read(&mut byte)? == 0 {
                break 'write_loop;
            }

            *buf_p = byte[0];
        }

        w.write(&buf)?;
    }

    Ok(())
}

#[test]
fn mixer_test() {
    use std::io::Cursor;
    let mut input = [Cursor::new("Hello"), Cursor::new("World")];
    let mut output = Cursor::new(Vec::<u8>::new());
    mixer(&mut input, &mut output).unwrap();

    let mut out_s = String::new();
    output.set_position(0);
    output.read_to_string(&mut out_s).unwrap();
    assert_eq!(out_s, "HWeolrllod");
}

#[test]
fn mixer_test_2() {
    use std::io::Cursor;
    let mut input = [Cursor::new("Hello!!!!"), Cursor::new("World")];
    let mut output = Cursor::new(Vec::<u8>::new());
    mixer(&mut input, &mut output).unwrap();

    let mut out_s = String::new();
    output.set_position(0);
    output.read_to_string(&mut out_s).unwrap();
    assert_eq!(out_s, "HWeolrllod");
}

fn splitter<R: Read, W: Write>(r: &mut R, w: &mut [W]) -> Result<(), std::io::Error> {
    let mut buf = vec![0; w.len()];

    loop {
        if r.read_exact(&mut buf).is_err() {
            break;
        }

        for (b, f) in buf.iter().zip(w.iter_mut()) {
            f.write(&[*b])?;
        }
    }

    Ok(())
}

#[test]
fn splitter_test() {
    use std::io::Cursor;
    let mut input = Cursor::new("HWeolrllod");
    let mut output = [Cursor::new(Vec::<u8>::new()), Cursor::new(Vec::<u8>::new())];

    splitter(&mut input, &mut output).unwrap();

    let mut out_s = String::new();
    output[0].set_position(0);
    output[0].read_to_string(&mut out_s).unwrap();
    assert_eq!(out_s, "Hello");

    let mut out_s = String::new();
    output[1].set_position(0);
    output[1].read_to_string(&mut out_s).unwrap();
    assert_eq!(out_s, "World");
}

#[test]
fn splitter_test_2() {
    use std::io::Cursor;
    let mut input = Cursor::new("HWeolrllod!!!");
    let mut output = [Cursor::new(Vec::<u8>::new()), Cursor::new(Vec::<u8>::new())];

    splitter(&mut input, &mut output).unwrap();

    let mut out_s = String::new();
    output[0].set_position(0);
    output[0].read_to_string(&mut out_s).unwrap();
    assert_eq!(out_s, "Hello!");

    let mut out_s = String::new();
    output[1].set_position(0);
    output[1].read_to_string(&mut out_s).unwrap();
    assert_eq!(out_s, "World!");
}

fn main() -> AHResult<()> {
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
                .open(output)
                .with_context(|| "Failed to open output file")?;

            mixer(&mut inputs_f, &mut output_f)?;
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

            splitter(&mut input_f, &mut output_fs)?;
        }
    }

    Ok(())
}
