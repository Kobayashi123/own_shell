use crate::{Feeder, ShellCore};
use nix::unistd::execvp;
use std::ffi::CString;
use std::process;

pub struct Command {
    text: String,
    args: Vec<String>,
    cargs: Vec<CString>,
}

impl Command {
    pub fn exec(&mut self, _core: &mut ShellCore) {
        //引数_coreはまだ使いません
        if self.text == "exit\n" {
            process::exit(0);
        }

        println!("{:?}", execvp(&self.cargs[0], &self.cargs));
    }

    pub fn parse(feeder: &mut Feeder, _core: &mut ShellCore) -> Option<Command> {
        let line = feeder.consume(feeder.remaining.len());
        let args: Vec<String> = line.trim_end().split(' ').map(|w| w.to_string()).collect();

        let cargs: Vec<CString> = args
            .iter()
            .map(|w| CString::new(w.clone()).unwrap())
            .collect();

        if args.len() > 0 {
            Some(Command {
                text: line,
                args: args,
                cargs: cargs,
            })
        } else {
            None
        }
    }
}
