use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
struct Args {
    file: Option<PathBuf>,
    output: Option<PathBuf>,
    include_dirs: Vec<PathBuf>,
    defined_macros: Vec<String>,
    help: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut args = Args::default();
    let mut argv = std::env::args().peekable();
    argv.next();
    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "-h" => args.help = true,
            "-I" => match argv.next().as_deref() {
                Some(path) => args.include_dirs.push(path.into()),
                None => return Err("`-I` must specify a path to include".into()),
            },
            "-D" => match argv.next().as_deref() {
                Some(m) => {
                    args.defined_macros.push(m.into());
                }
                None => return Err("`-D` must specify a macro string".into()),
            },
            arg if arg.starts_with('-') => {
                return Err(format!("unexpected dashed argument: {}", arg));
            }
            path => {
                if args.file.is_none() {
                    args.file = Some(path.into());
                } else if args.output.is_none() {
                    args.output = Some(path.into());
                } else {
                    return Err("more than 2 files specified".into());
                }
            }
        }
    }
    if args.file.as_ref().is_none() {
        return Err("missing <file>".into());
    }
    if args.output.as_ref().is_none() {
        return Err("missing <output>".into());
    }
    Ok(args)
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            println!("{}", e);
            Args {
                file: None,
                output: None,
                include_dirs: vec![],
                defined_macros: vec![],
                help: true,
            }
        }
    };
    if args.help {
        print!(
            "
USAGE:
    cpp_bundle <file> <output> [FLAGS]

ARGS:
    <file>     Input file path.
    <output>   Output file path.

FLAGS:
    -h                 Prints help information
    -I <include_path>  Add extra include_directories
    -D <macro>         Define extra macros
"
        );
        return;
    }
    println!("{:#?}", args);
}
