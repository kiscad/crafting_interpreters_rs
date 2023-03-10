use std::env;
use std::io::{BufRead, BufReader, Write, stdin};
use std::path::PathBuf;
use std::process;
use std::fs;
use anyhow::Error;

use lox::scanner::Scanner;
use lox::G_HAD_ERROR;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    
    if args.len() > 1 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[0]).unwrap();
    } else {
        run_promt().unwrap();
    }
}

fn run_file(fname: &str) -> Result<(), Error>{
    let path = PathBuf::from(fname);
    let src = fs::read_to_string(path)?;
    run(&src);

    unsafe { if G_HAD_ERROR {
        process::exit(65);
    }}
    Ok(())
}

fn run_promt() -> Result<(), Error>{
    let input = stdin();
    let mut reader = BufReader::new(input);

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut line = String::new();
        reader.read_line(&mut line)?;

        if line.is_empty() {
            break
        }

        run(&line);

        unsafe { G_HAD_ERROR = false; }
    }
    Ok(())
}

fn run(src: &str) {
    print!("{src}");
    let mut scanner = Scanner::new();
    let tokens = scanner.scan_tokens(src);

    let tokens = &tokens[..];
    let (expr, _) = lox::parser2::parse(tokens).unwrap();
    println!("{}", expr.format_ast());
}
