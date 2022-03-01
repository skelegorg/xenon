pub fn run(arguments: Vec<String>) {
    let args_result = Arguments::new(&arguments);
    if args_result.is_err() {
        println!("xenon version {} | use xenon --help for more commands.", env!("CARGO_PKG_VERSION"));
        return;
    }
    let args: Arguments = args_result.unwrap();
    match args.cmd[1].as_ref() {
        "image" => image(args),
        "restore" => restore(args),
        "--help" => help(),
        "--version" => println!("xenon version {}", env!("CARGO_PKG_VERSION")),
        _ => println!("xenon: no such command \'{}\'", &args.cmd[1])
    }
}

fn image(args: Arguments) {
    // look for config folder location
}

fn restore(args: Arguments) {

}

fn help() {
    println!("xenon version {}", env!("CARGO_PKG_VERSION"));
    println!("Usage: xenon [command] [arguments] [flags]\n");
    println!("Arguments:");
    println!("  [command]   The command to execute.");
    println!("  [arguments] The arguments passed to the command.");
    println!("  [flags]     Options for the command.\n");
    println!("Commands:");
    println!("  image       Create an image of the current config directory.");
    println!("  restore     Load an image.");
}


struct Arguments {
    cmd: Vec<String>,
    flags: Vec<String>,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        let mut cmd = Vec::new();
        let mut argss = Vec::new();

        for arg in args {
            match arg.chars().next() {
                Some('-') => {
                    argss.push(arg.clone());
                    cmd.push(arg.clone());
                },
                Some(_) => cmd.push(arg.clone()),
                None => println!("error reading argument {}", arg)
            }
        }
        
        Ok(Arguments { cmd: cmd, flags: argss})
    }
}
