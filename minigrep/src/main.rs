use minigrep::Config;
use std::{env, process};

fn main() {
    // 读取传入程序的命令行参数并将其收集到一个 vector 中
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application err: {}", e);

        process::exit(1);
    }
}
