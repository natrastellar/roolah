use super::model::{Account, CurrencyRecord};
use miette::Diagnostic;

#[derive(Debug, Diagnostic, thiserror::Error)]
pub enum Error {
    #[error("existing currency has the same name")]
    #[diagnostic(code(database::currency::create_currency))]
    CurrencyAlreadyExists(CurrencyRecord<'static>),
    #[error("existing account has the same name")]
    #[diagnostic(code(database::account::create_account))]
    AccountAlreadyExists(Account<'static>),
}
