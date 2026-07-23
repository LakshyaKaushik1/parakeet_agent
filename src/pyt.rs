use std::io::{Read, Write};
use std::sync::mpsc;
use portable_pty::{native_pty_system, CommandBuilder, PtySize, MasterPty};
use anyhow::Result;

/// What the app holds onto: a way to WRITE commands into the shell,
/// and a way to RECEIVE output that arrived from it. Notice there's
/// no `reader` field anymore - the reader itself now lives entirely
/// inside the background thread spawn_shell starts; the only thing
/// that crosses back out to the caller is the receiving end of a
/// channel, which is safe to poll from the main/UI thread without
/// ever blocking it.
pub struct ShellHandle {
    pub writer: Box<dyn Write + Send>,
    pub output_rx: mpsc::Receiver<Vec<u8>>,
    pub master: Box<dyn MasterPty + Send>,
}

pub fn spawn_shell() -> Result<ShellHandle> {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let cmd = CommandBuilder::new("bash");
    let _child = pair.slave.spawn_command(cmd)?;

    let mut reader = pair.master.try_clone_reader()?;
    let writer = pair.master.take_writer()?;

    // The channel: tx goes into the background thread, rx comes
    // back out to the caller (eventually App) to poll from the UI
    // loop without blocking it.
    let (tx, rx) = mpsc::channel::<Vec<u8>>();

    std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,       // shell process exited: EOF
                Ok(n) => {
                    if tx.send(buf[..n].to_vec()).is_err() {
                        break;        // nobody listening anymore, stop reading
                    }
                }
                Err(_) => break,      // real I/O error, give up
            }
        }
    });

    Ok(ShellHandle {
        writer,
        output_rx: rx,
        master: pair.master,
    })
}

impl ShellHandle {
    pub fn resize(&self, rows: u16, cols: u16) -> Result<()> {
        self.master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(())
    }
}