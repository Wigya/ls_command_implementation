use ls_implementation::{run, Config};

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();

    let config: Config = Config::new(&args).unwrap();

    if config.tool == "ls" {
        run(&config).unwrap();
    }
}
