use std::{
    io::{ErrorKind, Result},
    process::{Command, Stdio},
};

pub trait Runner {
    /// A list of commands to execute for this runner, in order.
    const COMPILE_COMMANDS: &[Command];

    fn execute(commands: Vec<Command>) -> Result<()> {
        for mut cmd in commands {
            let status = cmd.stdout(Stdio::null()).status()?;

            if !status.success() {
                return Err(std::io::Error::new(
                    ErrorKind::Other,
                    format!("Command {:?} failed with status {:?}", cmd, status),
                ));
            }
        }
        Ok(())
    }
}
