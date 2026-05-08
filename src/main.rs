use std::env;

fn main() {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        None | Some("-h") | Some("--help") => print_help(),
        Some("-V") | Some("--version") => println!("rushbuild 0.1.0"),
        Some(cmd) => {
            eprintln!("Unknown command: {cmd}");
            print_help();
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!(
        "\
rushbuild 0.1.0

Rust sibling of goshbuild.

Usage:
  rushbuild --help
  rushbuild --version

Reference:
  https://github.com/runplus-community/goshbuild

Status:
  scaffold only; the Rust packer is the next implementation step.
"
    );
}

