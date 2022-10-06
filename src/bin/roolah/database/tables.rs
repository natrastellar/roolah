use roolah_derive::ColumnEnum;

pub const ACCOUNT_TYPES: &str = "account_types";
pub const ACCOUNTS: &str = "accounts";
pub const ACCOUNTS_WITH_CURRENCY_AND_TYPE: &str = "accounts_with_currency_and_type";
pub const CATEGORIES: &str = "categories";
pub const CURRENCIES: &str = "currencies";
pub const METHODS: &str = "methods";
pub const TRANSACTIONS: &str = "transactions";

#[derive(ColumnEnum)]
pub enum AccountTypesColumn {
    Id,
    Name,
}

#[derive(ColumnEnum)]
pub enum AccountsColumn {
    Id,
    Name,
    Currency,
    Balance,
    PostedBalance,
    AccountType,
}

#[derive(ColumnEnum)]
pub enum AccountsWithCurrencyAndTypeColumn {
    Id,
    Name,
    CurrencyId,
    Balance,
    PostedBalance,
    AccountTypeId,
    Symbol,
    CurrencyName,
    Precision,
    ThousandSeparator,
    DecimalSeparator,
    AccountTypeName,
}

#[derive(ColumnEnum)]
pub enum CategoriesColumn {
    Id,
    Name,
}

#[derive(ColumnEnum)]
pub enum CurrenciesColumn {
    Id,
    Symbol,
    Name,
    Precision,
    ThousandSeparator,
    DecimalSeparator,
}

#[derive(ColumnEnum)]
pub enum MethodsColumn {
    Id,
    Name,
}

#[derive(ColumnEnum)]
pub enum TransactionsColumn {
    Id,
    Date,
    PostedDate,
    Category,
    Amount,
    DebitAccount,
    CreditAccount,
    Authority,
    Description,
    Method,
    CheckNumber,
}
