use thiserror::Error;

#[derive(Debug, Error)]
pub enum SetuError {
    #[error("Joplin URL is invalid")]
    InvalidJoplinUrl,

    #[error("Joplin URL must use HTTP on a loopback address")]
    UnsafeJoplinUrl,

    #[error("Joplin clipper service returned an unexpected response")]
    UnexpectedJoplinResponse,

    #[error("JOPLIN_TOKEN must not be empty")]
    InvalidJoplinToken,

    #[error("Joplin rejected the configured token")]
    JoplinAuthenticationFailed,

    #[error("SETU_ALLOWED_NOTEBOOKS must contain 1 to 20 unique, bounded names")]
    InvalidNotebookAllowlist,

    #[error("a notebook allowlist requires JOPLIN_TOKEN")]
    NotebookAllowlistRequiresToken,

    #[error("an allowed notebook could not be found exactly")]
    AllowedNotebookNotFound,

    #[error("an allowed notebook name is ambiguous")]
    AllowedNotebookAmbiguous,

    #[error("could not reach the local Joplin clipper service")]
    JoplinConnection,
}
