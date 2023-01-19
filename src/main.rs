mod core;
mod elem_command;
mod feeder;
mod term;
mod utils;

use std::{env, process};

use crate::core::ShellCore;
use crate::elem_command::Command;
use crate::feeder::Feeder;

fn show_version() {
    eprintln!("Moz Bash, TERMINAL");
    eprintln!("© 2023 Shun Kobayashi");
    eprintln!("License: BSD 3-Clause\n");

    eprintln!("This is open source software. You can redistirbute and use in source\nand binary forms with or without modification under the license.");
    eprintln!("There is no warranty, to the extent permitted by law.");
    process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--version" {
        show_version();
    }

    /* Ignore Ctrl+C (Childlen will receive instead.) */
    ctrlc::set_handler(move || {}).expect("Unable to set the Ctrl+C handler.");

    let mut core = ShellCore::new();
    main_loop(&mut core);
}

fn main_loop(core: &mut ShellCore) {
    let mut feeder = Feeder::new();
    loop {
        if feeder.feed_line(core) {
            if let Some(mut c) = Command::parse(&mut feeder, core) {
                c.exec(core);
            } else {
                process::exit(1);
            }
        }
    }
}
