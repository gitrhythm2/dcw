use std::env;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();

    let opts = match args[1].as_str() {
        "c" => parse_container(&args[2..]),
        "i" => parse_image(&args[2..]),
        "n" => parse_network(&args[2..]),
        "v" => parse_volume(&args[2..]),
        "-h" => {
            usage();
            std::process::exit(0);
        }
        _ => to_vector(&args[1..]),
    };
    println!("> docker {}", opts.join(" "));

    let mut command = Command::new("docker");
    for opt in opts {
        command.arg(opt);
    }

    command
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command")
        .wait()
        .expect("Failed to wait for command");
}

fn parse_container(args: &[String]) -> Vec<&str> {
    let mut v = to_vector(args);
    v.insert(0, "container");
    v
}

fn parse_image(args: &[String]) -> Vec<&str> {
    let mut v = to_vector(args);
    v.insert(0, "image");
    v
}

fn parse_network(args: &[String]) -> Vec<&str> {
    let mut v = to_vector(args);
    v.insert(0, "network");
    v
}

fn parse_volume(args: &[String]) -> Vec<&str> {
    let mut v = to_vector(args);
    v.insert(0, "volume");
    v
}

fn to_vector(args: &[String]) -> Vec<&str> {
    args.iter().map(|x| x.as_str()).collect::<Vec<&str>>()
}

fn usage() {
    println!("Usage: dcw <subcommand> <options...>");
    println!("Subcommand");
    println!("  c: container");
    println!("  i: image");
    println!("  n: network");
    println!("  v: volume");
}
