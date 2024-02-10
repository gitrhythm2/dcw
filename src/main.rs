use std::env;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();

    let opts = match args[1].as_str() {
        "c" => container_parser(&args[2..]),
        "i" => image_parser(&args[2..]),
        "n" => network_parser(&args[2..]),
        "v" => volume_parser(&args[2..]),
        _ => {
            eprintln!("Invalid command: {}", args[1]);
            std::process::exit(1);
        }
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

fn container_parser(args: &[String]) -> Vec<&str> {
    let mut v = to_vector(args);
    v.insert(0, "container");
    v
}

fn image_parser(args: &[String]) -> Vec<&str> {
    let mut v = to_vector(args);
    v.insert(0, "image");
    v
}

fn network_parser(args: &[String]) -> Vec<&str> {
    let mut v = to_vector(args);
    v.insert(0, "network");
    v
}

fn volume_parser(args: &[String]) -> Vec<&str> {
    let mut v = to_vector(args);
    v.insert(0, "volume");
    v
}

fn to_vector(args: &[String]) -> Vec<&str> {
    args.iter().map(|x| x.as_str()).collect::<Vec<&str>>()
}
