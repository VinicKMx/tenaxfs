use std::env;

use tenaxfs_sim::smoke;

const COMMANDS: &[&str] = &[
    "inspect",
    "verify",
    "timeline",
    "wear",
    "recover",
    "simulate",
    "benchmark",
    "dump",
];

fn main() {
    let mut args = env::args();
    let _program = args.next();

    match args.next().as_deref() {
        Some("simulate") => run_simulate(),
        Some("inspect" | "verify" | "timeline" | "wear" | "recover" | "benchmark" | "dump") => {
            eprintln!("tenaxfs: command is reserved but not implemented in checkpoint 1");
            std::process::exit(2);
        }
        Some("-h" | "--help") | None => print_help(),
        Some(command) => {
            eprintln!("tenaxfs: unknown command `{command}`");
            print_help();
            std::process::exit(2);
        }
    }
}

fn run_simulate() {
    match smoke() {
        Ok(report) => {
            println!("TenaxFS simulator smoke scenario");
            println!("  total size     {} bytes", report.total_size);
            println!("  segments       {}", report.segment_count);
            println!("  erase size     {} bytes", report.erase_size);
            println!("  program size   {} bytes", report.program_size);
        }
        Err(error) => {
            eprintln!("tenaxfs: simulator failed: {}", error.code());
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("TenaxFS host tooling");
    println!();
    println!("Usage:");
    println!("  tenaxfs <command>");
    println!();
    println!("Commands:");

    for command in COMMANDS {
        println!("  {command}");
    }
}
