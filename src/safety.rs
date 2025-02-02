use std::{
    env, mem,
    os::unix::process::CommandExt as _,
    process::{Command, ExitStatus},
};

#[repr(C)]
pub struct Header {
    seq: u32,
    len: u32,
}

impl Header {
    /// # Safety
    /// `bytes` must have 4-byte alignment.
    pub unsafe fn from_bytes(bytes: [u8; 8]) -> Self {
        // Not recommended. Use bytemuck or zerocopy or something similar.
        mem::transmute(bytes)
    }
}

pub fn run_special_command(command: &str) -> ExitStatus {
    #[expect(deprecated, reason = "old code, should be updated")]
    Command::new(command)
        .before_exec(|| unsafe {
            if libc::setpgid(0, 0) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        })
        .status()
        .expect("failed to spawn command")
}

pub fn dangerous_env_games() {
    env::set_var("secret_token", "s33kr1t");
}
