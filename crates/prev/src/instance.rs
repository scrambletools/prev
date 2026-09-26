//! Single instance: the first `prev` owns a Unix socket, later launches send
//! their file paths to it and exit.
//!
//! A request is a sequence of NUL-terminated absolute paths; an empty request
//! asks for a new window. The owner acknowledges with `ok`.

use std::ffi::OsString;
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::time::Duration;

const ACK: &[u8] = b"ok";
const FORWARD_TIMEOUT: Duration = Duration::from_secs(2);
const MAX_REQUEST: u64 = 1 << 20;

pub const SOCKET_NAME: &str = "instance.sock";

pub enum Role {
    /// This process owns the socket and should serve requests.
    Primary(Listener),
    /// Another process took the request.
    Forwarded,
}

pub struct Listener {
    listener: UnixListener,
}

/// Becomes the primary instance, or hands `paths` to the running one.
pub fn claim_or_forward(socket: &Path, paths: &[PathBuf]) -> io::Result<Role> {
    if let Some(dir) = socket.parent() {
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(dir)?;
    }
    for _ in 0..2 {
        match UnixListener::bind(socket) {
            Ok(listener) => {
                fs::set_permissions(socket, fs::Permissions::from_mode(0o600))?;
                return Ok(Role::Primary(Listener { listener }));
            }
            Err(error) if error.kind() == io::ErrorKind::AddrInUse => {
                match forward(socket, paths) {
                    Ok(()) => return Ok(Role::Forwarded),
                    Err(error) if is_stale(&error) => fs::remove_file(socket)?,
                    Err(error) => return Err(error),
                }
            }
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::other("instance socket keeps reappearing"))
}

fn is_stale(error: &io::Error) -> bool {
    matches!(
        error.kind(),
        io::ErrorKind::ConnectionRefused | io::ErrorKind::NotFound
    )
}

fn forward(socket: &Path, paths: &[PathBuf]) -> io::Result<()> {
    let mut stream = UnixStream::connect(socket)?;
    stream.set_read_timeout(Some(FORWARD_TIMEOUT))?;
    stream.set_write_timeout(Some(FORWARD_TIMEOUT))?;
    stream.write_all(&encode(paths)?)?;
    stream.shutdown(std::net::Shutdown::Write)?;
    let mut reply = Vec::new();
    stream.read_to_end(&mut reply)?;
    if reply == ACK {
        Ok(())
    } else {
        Err(io::Error::other("running instance did not acknowledge"))
    }
}

fn encode(paths: &[PathBuf]) -> io::Result<Vec<u8>> {
    let mut request = Vec::new();
    for path in paths {
        let absolute = std::path::absolute(path)?;
        request.extend_from_slice(absolute.as_os_str().as_bytes());
        request.push(0);
    }
    Ok(request)
}

fn decode(request: &[u8]) -> Vec<PathBuf> {
    request
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty())
        .map(|part| PathBuf::from(OsString::from_vec(part.to_vec())))
        .collect()
}

impl Listener {
    /// Serves requests on a background thread, calling `on_request` for each.
    pub fn spawn(self, on_request: impl Fn(Vec<PathBuf>) + Send + 'static) {
        std::thread::Builder::new()
            .name("prev-instance".into())
            .spawn(move || {
                for stream in self.listener.incoming() {
                    let Ok(mut stream) = stream else { continue };
                    let _ = stream.set_read_timeout(Some(FORWARD_TIMEOUT));
                    let mut request = Vec::new();
                    if (&mut stream)
                        .take(MAX_REQUEST)
                        .read_to_end(&mut request)
                        .is_err()
                    {
                        continue;
                    }
                    on_request(decode(&request));
                    let _ = stream.write_all(ACK);
                }
            })
            .expect("spawn instance listener thread");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    fn socket_in(dir: &tempfile::TempDir) -> PathBuf {
        dir.path().join("prev").join(SOCKET_NAME)
    }

    #[test]
    fn second_launch_forwards_paths() {
        let dir = tempfile::tempdir().unwrap();
        let socket = socket_in(&dir);
        let Role::Primary(listener) = claim_or_forward(&socket, &[]).unwrap() else {
            panic!("first launch must be primary");
        };
        let (sender, receiver) = mpsc::channel();
        listener.spawn(move |paths| sender.send(paths).unwrap());

        let files = [PathBuf::from("/tmp/a b.pdf"), PathBuf::from("relative.png")];
        assert!(matches!(
            claim_or_forward(&socket, &files).unwrap(),
            Role::Forwarded
        ));
        let received = receiver.recv_timeout(Duration::from_secs(2)).unwrap();
        assert_eq!(received[0], PathBuf::from("/tmp/a b.pdf"));
        assert!(received[1].is_absolute() && received[1].ends_with("relative.png"));

        assert!(matches!(
            claim_or_forward(&socket, &[]).unwrap(),
            Role::Forwarded
        ));
        assert!(
            receiver
                .recv_timeout(Duration::from_secs(2))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn stale_socket_is_replaced() {
        let dir = tempfile::tempdir().unwrap();
        let socket = socket_in(&dir);
        fs::create_dir_all(socket.parent().unwrap()).unwrap();
        drop(UnixListener::bind(&socket).unwrap());
        assert!(socket.exists(), "socket file outlives its listener");
        assert!(matches!(
            claim_or_forward(&socket, &[]).unwrap(),
            Role::Primary(_)
        ));
    }

    #[test]
    fn socket_is_private() {
        let dir = tempfile::tempdir().unwrap();
        let socket = socket_in(&dir);
        let _role = claim_or_forward(&socket, &[]).unwrap();
        let mode = fs::metadata(&socket).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        let dir_mode = fs::metadata(socket.parent().unwrap())
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(dir_mode, 0o700);
    }

    #[test]
    fn decode_ignores_empty_segments() {
        assert_eq!(
            decode(b"/a\0\0/b\0"),
            vec![PathBuf::from("/a"), PathBuf::from("/b")]
        );
        assert!(decode(b"").is_empty());
    }
}
