use nix::sys::wait;
use nix::sys::wait::WaitStatus;
use nix::unistd::Pid;

pub struct ShellCore {
    pub history: Vec<String>,
}

impl ShellCore {
    pub fn new() -> ShellCore {
        let core = ShellCore {
            history: Vec::new(),
        };

        core
    }

    pub fn wait_process(&mut self, child: Pid) {
        let exit_status = match wait::waitpid(child, None) {
            Ok(WaitStatus::Exited(_pid, status)) => status,
            Ok(unsupported) => {
                eprintln!("Unsupported: {:?}", unsupported);
                1
            }
            Err(err) => {
                panic!("Error: {:?}", err);
            }
        };

        eprintln!("終了ステータス: {}", exit_status);
    }
}
