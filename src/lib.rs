pub fn run(arguments: Vec<String>) {
    let args = Arguments::new(&arguments);
}

struct Arguments {
    cmd: Vec<String>,
    verbose: bool,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        let mut cmd = Vec::new();
        let mut argss = Vec::new();

        for arg in args {
            println!("{:?}", arg);
            match arg.chars().next() {
                Some('-') => argss.push(arg.clone()),
                Some(_) => cmd.push(arg.clone()),
                None => println!("error reading argument {}", arg)
            }
        }

        let mut verb: bool = false;

        for arg in argss {
            match arg.as_ref() {
                "-v" => verb = true,
                "--verbose" => verb = true,
                _ => {}
            }
        }
        Ok(Arguments { cmd: cmd, verbose: verb })
    }
}
