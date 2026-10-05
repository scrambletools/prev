//! Opening links and printing, in prev's own words: through the XDG
//! desktop portal on Linux, and through the system itself elsewhere. The
//! desktop's settings, its accent color, light or dark and animations, are
//! read by scramble-ui.

#[cfg(target_os = "linux")]
pub use scramble_ui::desktop::color_scheme;
pub use scramble_ui::desktop::{accent_changes, accent_color, animations_enabled};

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(not(target_os = "linux"))]
pub use other::*;

#[cfg(target_os = "linux")]
mod linux {
    use std::path::PathBuf;

    pub async fn open_uri(uri: String) -> Result<(), String> {
        let parsed = ashpd::Uri::parse(&uri).map_err(|error| {
            crate::fl!(
                "app-link-invalid",
                uri = uri.as_str(),
                error = error.to_string()
            )
        })?;
        ashpd::desktop::open_uri::OpenFileRequest::default()
            .send_uri(&parsed)
            .await
            .map(|_| ())
            .map_err(|error| {
                crate::fl!(
                    "app-link-open-failed",
                    uri = uri.as_str(),
                    error = error.to_string()
                )
            })
    }

    pub async fn print(path: PathBuf, title: String) -> Result<(), String> {
        use ashpd::desktop::print::{PreparePrintOptions, PrintOptions, PrintProxy};
        use std::os::fd::AsFd;

        let failed = |error: ashpd::Error| crate::fl!("print-failed", error = error.to_string());
        let proxy = PrintProxy::new().await.map_err(failed)?;
        let prepared = proxy
            .prepare_print(
                None,
                &title,
                Default::default(),
                Default::default(),
                PreparePrintOptions::default(),
            )
            .await
            .map_err(failed)?;
        let prepared = match prepared.response() {
            Ok(prepared) => prepared,
            Err(ashpd::Error::Response(_)) => return Ok(()),
            Err(error) => return Err(failed(error)),
        };
        let file = std::fs::File::open(&path)
            .map_err(|error| crate::fl!("print-failed", error = error.to_string()))?;
        proxy
            .print(
                None,
                &title,
                &file.as_fd(),
                PrintOptions::default().set_token(prepared.token),
            )
            .await
            .map_err(failed)?;
        Ok(())
    }
}

#[cfg(not(target_os = "linux"))]
mod other {
    use std::path::PathBuf;

    /// Opens `uri` in the default app: through the URL handler every
    /// Windows version has, or `open` on macOS.
    pub async fn open_uri(uri: String) -> Result<(), String> {
        #[cfg(windows)]
        let mut command = std::process::Command::new("rundll32");
        #[cfg(windows)]
        command.args(["url.dll,FileProtocolHandler", &uri]);
        #[cfg(not(windows))]
        let mut command = std::process::Command::new("open");
        #[cfg(not(windows))]
        command.arg(&uri);
        command.spawn().map(|_| ()).map_err(|error| {
            crate::fl!(
                "app-link-open-failed",
                uri = uri.as_str(),
                error = error.to_string()
            )
        })
    }

    /// Prints through the Windows print dialog, on a thread of its own:
    /// the dialog and the spooling block until they are done.
    #[cfg(windows)]
    pub async fn print(path: PathBuf, title: String) -> Result<(), String> {
        let (sender, receiver) = iced::futures::channel::oneshot::channel();
        std::thread::Builder::new()
            .name("prev-print".into())
            .spawn(move || {
                let _ = sender.send(crate::print_windows::print(&path, &title));
            })
            .map_err(|error| crate::fl!("print-failed", error = error.to_string()))?;
        receiver
            .await
            .unwrap_or_else(|_| Err(crate::fl!("print-stopped")))
    }

    /// Prints through AppKit's print panel, which must run on the main
    /// thread; this waits for it there.
    #[cfg(target_os = "macos")]
    pub async fn print(path: PathBuf, title: String) -> Result<(), String> {
        let (sender, receiver) = iced::futures::channel::oneshot::channel();
        crate::print_macos::on_main(move |mtm| {
            let _ = sender.send(crate::print_macos::print_on_main(mtm, &path, &title));
        });
        receiver
            .await
            .unwrap_or_else(|_| Err(crate::fl!("print-unavailable")))
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    pub async fn print(_path: PathBuf, _title: String) -> Result<(), String> {
        Err(crate::fl!("print-unavailable"))
    }
}
