use gita;

const HELP: &str = r#"gita v0.1.0

USAGE:
gita epoch [commit]    Get timestamp of commit from epoch
"#;

fn help() {
    println!("{}", HELP);
}

fn main() {
    let args = ::std::env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        help();
        return;
    }

    match args[1].as_str() {
        "epoch" => gita::epoch(&args[2]),
        _ => help(),
    }
}
