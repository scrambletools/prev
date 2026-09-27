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
        let parsed =
            ashpd::Uri::parse(&uri).map_err(|error| format!("Invalid link {uri}: {error}"))?;
        ashpd::desktop::open_uri::OpenFileRequest::default()
            .send_uri(&parsed)
            .await
            .map(|_| ())
            .map_err(|error| format!("Could not open {uri}: {error}"))
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

        let failed = |error: ashpd::Error| format!("Could not print: {error}");
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
        let file =
            std::fs::File::open(&path).map_err(|error| format!("Could not print: {error}"))?;
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

    /// Opens `uri` in the default app, through the URL handler every
    /// Windows version has.
    pub async fn open_uri(uri: String) -> Result<(), String> {
        std::process::Command::new("rundll32")
            .args(["url.dll,FileProtocolHandler", &uri])
            .spawn()
            .map(|_| ())
            .map_err(|error| format!("Could not open {uri}: {error}"))
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

    /// Not read from the system here; animations stay on unless the
    /// settings turn them off.
    #[cfg(not(windows))]
    pub async fn animations_enabled() -> Option<bool> {
        None
    }

    pub async fn print(_path: PathBuf, _title: String) -> Result<(), String> {
        Err("Printing is not available on this system yet.".to_owned())
    }
}
