//! Desktop portal requests: opening links and printing.

use std::path::PathBuf;

pub async fn open_uri(uri: String) -> Result<(), String> {
    let parsed = ashpd::Uri::parse(&uri).map_err(|error| format!("Invalid link {uri}: {error}"))?;
    ashpd::desktop::open_uri::OpenFileRequest::default()
        .send_uri(&parsed)
        .await
        .map(|_| ())
        .map_err(|error| format!("Could not open {uri}: {error}"))
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
    let file = std::fs::File::open(&path).map_err(|error| format!("Could not print: {error}"))?;
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
