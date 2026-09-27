//! Single instance on Windows: the first `prev` owns a named pipe, later
//! launches send their file paths to it and exit.
//!
//! Pipes cannot be half closed, so a request is its length (four bytes,
//! little endian) and then NUL-terminated absolute paths in UTF-8; an
//! empty request asks for a new window. The owner acknowledges with `ok`.

use std::io::{self, Read, Write};
use std::path::PathBuf;

use interprocess::local_socket::{GenericNamespaced, ListenerOptions, Stream, prelude::*};

const ACK: &[u8] = b"ok";
const MAX_REQUEST: u32 = 1 << 20;

pub enum Role {
    /// This process owns the pipe and should serve requests.
    Primary(Listener),
    /// Another process took the request.
    Forwarded,
}

pub struct Listener {
    listener: interprocess::local_socket::Listener,
}

/// The pipe's name: one per user, and apart for development builds.
pub fn pipe_name() -> String {
    let app = if prev_store::paths::PRODUCTION {
        "prev"
    } else {
        "prev-dev"
    };
    let user = std::env::var("USERNAME").unwrap_or_default();
    format!("{app}-{user}-instance")
}

/// Becomes the primary instance, or hands `paths` to the running one.
pub fn claim_or_forward(name: &str, paths: &[PathBuf]) -> io::Result<Role> {
    let request = encode(paths)?;
    for _ in 0..2 {
        let pipe = name.to_ns_name::<GenericNamespaced>()?;
        match ListenerOptions::new().name(pipe).create_sync() {
            Ok(listener) => return Ok(Role::Primary(Listener { listener })),
            Err(error) if error.kind() == io::ErrorKind::AddrInUse => {
                match forward(name, &request) {
                    Ok(()) => return Ok(Role::Forwarded),
                    // The owner may have just quit; try to take over.
                    Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
                    Err(error) => return Err(error),
                }
            }
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::other("instance pipe keeps changing hands"))
}

fn forward(name: &str, request: &[u8]) -> io::Result<()> {
    let mut stream = Stream::connect(name.to_ns_name::<GenericNamespaced>()?)?;
    let length = u32::try_from(request.len()).map_err(io::Error::other)?;
    stream.write_all(&length.to_le_bytes())?;
    stream.write_all(request)?;
    stream.flush()?;
    let mut reply = [0; 2];
    stream.read_exact(&mut reply)?;
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
        let text = absolute
            .to_str()
            .ok_or_else(|| io::Error::other(format!("{} is not a valid name", path.display())))?;
        request.extend_from_slice(text.as_bytes());
        request.push(0);
    }
    Ok(request)
}

fn decode(request: &[u8]) -> Vec<PathBuf> {
    request
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty())
        .filter_map(|part| std::str::from_utf8(part).ok())
        .map(PathBuf::from)
        .collect()
}

fn serve(stream: &mut Stream) -> io::Result<Vec<PathBuf>> {
    let mut length = [0; 4];
    stream.read_exact(&mut length)?;
    let length = u32::from_le_bytes(length);
    if length > MAX_REQUEST {
        return Err(io::Error::other("request too large"));
    }
    let mut request = vec![0; length as usize];
    stream.read_exact(&mut request)?;
    Ok(decode(&request))
}

impl Listener {
    /// Serves requests on a background thread, calling `on_request` for each.
    pub fn spawn(self, on_request: impl Fn(Vec<PathBuf>) + Send + 'static) {
        std::thread::Builder::new()
            .name("prev-instance".into())
            .spawn(move || {
                for stream in self.listener.incoming() {
                    let Ok(mut stream) = stream else { continue };
                    let Ok(paths) = serve(&mut stream) else {
                        continue;
                    };
                    on_request(paths);
                    let _ = stream.write_all(ACK);
                    let _ = stream.flush();
                }
            })
            .expect("spawn instance listener thread");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn second_launch_forwards_paths() {
        let name = format!("prev-test-{}", std::process::id());
        let Role::Primary(listener) = claim_or_forward(&name, &[]).unwrap() else {
            panic!("first launch must be primary");
        };
        let (sender, receiver) = mpsc::channel();
        listener.spawn(move |paths| sender.send(paths).unwrap());

        let files = [
            PathBuf::from(r"C:\Temp\a b.pdf"),
            PathBuf::from("relative.png"),
        ];
        assert!(matches!(
            claim_or_forward(&name, &files).unwrap(),
            Role::Forwarded
        ));
        let received = receiver.recv_timeout(Duration::from_secs(2)).unwrap();
        assert_eq!(received[0], PathBuf::from(r"C:\Temp\a b.pdf"));
        assert!(received[1].is_absolute() && received[1].ends_with("relative.png"));

        assert!(matches!(
            claim_or_forward(&name, &[]).unwrap(),
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
    fn decode_ignores_empty_segments() {
        assert_eq!(
            decode(b"C:\\a\0\0C:\\b\0"),
            vec![PathBuf::from(r"C:\a"), PathBuf::from(r"C:\b")]
        );
        assert!(decode(b"").is_empty());
    }
}
