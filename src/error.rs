use miette::Diagnostic;

#[derive(Debug, Diagnostic, thiserror::Error)]
#[diagnostic(code(roolah::error), url(docsrs))]
pub enum Error {
    #[error(transparent)]
    Other(Box<dyn std::error::Error>),
}
