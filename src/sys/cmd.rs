use std::ffi;
use std::io;
use std::process::{Output, Stdio};

use tokio::process::Command;

#[derive(Debug)]
pub(crate) struct Cmd {
    command: Command,
}

impl Cmd {
    pub(crate) fn new<C, I, A>(command: C, args: I) -> Self
    where
        C: AsRef<ffi::OsStr>,
        I: IntoIterator<Item = A>,
        A: AsRef<ffi::OsStr>,
    {
        let mut command = Command::new(command);
        command.args(args);
        Self { command }
    }

    fn process_output(&self, output: Output) -> io::Result<String> {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();

        if output.status.success() {
            println!("SUCCESS:\n{}", stdout);
            Ok(stdout)
        } else {
            println!("FAILURE:\n{}", stderr);
            Err(io::Error::new(io::ErrorKind::Other, stderr))
        }
    }

    pub(crate) async fn exec(&mut self) -> io::Result<String> {
        let output = self
            .command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
        self.process_output(output)
    }
}
