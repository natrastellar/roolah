use miette::Diagnostic;
use roolah::{model::Account, finance::CurrencyFormat};

#[derive(Debug, Diagnostic, thiserror::Error)]
pub enum Error {
    #[error("existing currency has the same name")]
    #[diagnostic(code(database::currency::create_currency))]
    CurrencyAlreadyExists {
        id: i64,
        currency: CurrencyFormat<'static>
    },
    #[error("existing account has the same name")]
    #[diagnostic(code(database::account::create_account))]
    AccountAlreadyExists(Account<'static>),
}
