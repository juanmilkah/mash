use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        // print the $ sign
        println!("$ ");
        stdout().flush().unwrap();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();
        let mut commands = input.split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.split_whitespace().peekable();
            let command = parts.next().unwrap();
            let mut args = parts;

            match command {
                // make cd built in
                "cd" => {
                    let new_dir = args.peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);

                    if let Err(e) = env::set_current_dir(root) {
                        eprintln!("{:?}", e)
                    }
                    previous_command = None;
                }

                "exit" => return,

                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => previous_command = Some(output),
                        Err(e) => {
                            eprintln!("{:?}", e);
                            previous_command = None;
                        }
                    }
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            match final_command.wait() {
                Ok(_) => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
}
