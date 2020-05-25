extern crate libmov;

use libmov::mov_file::MovFile;
use std::env;
use std::error::Error;
use std::io;
use std::io::Write;
use std::path::Path;

type Result<T = ()> = std::result::Result<T, Box<dyn Error + 'static>>;

fn pause() -> Result {
    let mut stdout = io::stdout();
    write!(stdout, "\nPress ENTER to continue... ")?;
    stdout.flush()?;
    io::stdin().read_line(&mut String::new())?;

    Ok(())
}

fn help(path: &Path) -> Result {
    let filename = {
        let filename = String::from(path.file_name().unwrap().to_str().unwrap());
        let extension = path.extension().unwrap().to_str().unwrap_or("").to_lowercase();
        match extension.as_str() {
            "exe" => filename,
            _ => format!("./{}", filename),
        }
    };
    println!("»» {} v{} by Aldo_MX\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("» Description: {}\n", env!("CARGO_PKG_DESCRIPTION"));
    println!("» Example: {} A01.MOV A02.MOV\n", filename);
    pause()?;

    Ok(())
}

fn convert(path: &Path) -> Result {
    let mov = MovFile::new_from_file(&path)?;
    mov.save_m2v(&path.with_extension("M2V"))?;

    Ok(())
}

fn main() -> Result {
    let raw_args: Vec<String> = env::args().collect();
    let (exe, args) = raw_args.split_first().unwrap();
    if args.len() == 0 {
        let path = Path::new(exe);
        help(&path)?;
        return Ok(());
    }

    let mut has_errors = false;
    let mut stderr = io::stderr();
    let mut stdout = io::stdout();
    for filename in args {
        let path = Path::new(filename);
        write!(stdout, "Converting {}... ", path.display())?;
        stdout.flush()?;

        match convert(path) {
            Ok(_) => println!("OK!"),
            Err(error) => {
                has_errors = true;
                println!("ERROR!");
                writeln!(stderr, "{}", error)?;
            },
        }
    }

    if has_errors {
        pause()?;
    }

    Ok(())
}
