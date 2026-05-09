use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "--help" || args[0] == "help" {
        println!("demo-app");
        println!("usage:");
        println!("  demo-app [--rushbuild-test <sentinel>] [exit42] [args...]");
        return;
    }

    if args[0] == "--rushbuild-test" {
        if let Some(sentinel) = args.get(1) {
            println!("{sentinel}");
            return;
        }
        eprintln!("missing sentinel");
        process::exit(2);
    }

    if args[0] == "exit42" {
        println!("exit42");
        process::exit(42);
    }

    println!("{}", args.join(" "));
}
