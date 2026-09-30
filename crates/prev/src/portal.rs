//! Opening links, printing and reading desktop settings: through the XDG
//! desktop portal on Linux, and through the system itself elsewhere.

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

    /// Whether the desktop wants animations, from the GNOME interface setting
    /// the settings portal passes on. `None` when the portal cannot tell.
    pub async fn animations_enabled() -> Option<bool> {
        let settings = ashpd::desktop::settings::Settings::new().await.ok()?;
        settings
            .read::<bool>("org.gnome.desktop.interface", "enable-animations")
            .await
            .ok()
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
            Err(ashpd::Error::Response(ashpd::desktop::ResponseError::Cancelled)) => return Ok(()),
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

    /// Whether Windows animates controls and elements, its "Animation
    /// effects" setting.
    #[cfg(windows)]
    #[allow(unsafe_code)]
    pub async fn animations_enabled() -> Option<bool> {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            SPI_GETCLIENTAREAANIMATION, SystemParametersInfoW,
        };
        let mut enabled: i32 = 1;
        // SAFETY: the setting is a BOOL written to `enabled`, which lives
        // through the call.
        let read = unsafe {
            SystemParametersInfoW(SPI_GETCLIENTAREAANIMATION, 0, (&raw mut enabled).cast(), 0)
        };
        (read != 0).then_some(enabled != 0)
    }

    /// Whether macOS animates, the opposite of its Reduce Motion setting.
    #[cfg(target_os = "macos")]
    pub async fn animations_enabled() -> Option<bool> {
        let workspace = objc2_app_kit::NSWorkspace::sharedWorkspace();
        Some(!workspace.accessibilityDisplayShouldReduceMotion())
    }

    /// Not read from the system here; animations stay on unless the
    /// settings turn them off.
    #[cfg(not(any(windows, target_os = "macos")))]
    pub async fn animations_enabled() -> Option<bool> {
        None
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
