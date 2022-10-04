use roolah_derive::ColumnEnum;

pub const ACCOUNT_TYPES: &str = "account_types";
pub const ACCOUNTS: &str = "accounts";
pub const ACCOUNTS_WITH_CURRENCY_AND_TYPE: &str = "accounts_with_currency_and_type";
pub const CATEGORIES: &str = "categories";
pub const CURRENCIES: &str = "currencies";
pub const METHODS: &str = "methods";
pub const TRANSACTIONS: &str = "transactions";

#[allow(dead_code)]
#[derive(ColumnEnum)]
pub enum AccountTypesColumn {
    Id,
    Name,
}

#[allow(dead_code)]
#[derive(ColumnEnum)]
pub enum AccountsColumn {
    Id,
    Name,
    Currency,
    Balance,
    PostedBalance,
    AccountType,
}

#[allow(dead_code)]
#[derive(ColumnEnum)]
pub enum AccountsWithCurrencyAndTypeColumn {
    Id,
    Name,
    CurrencyId,
    Balance,
    PostedBalance,
    AccountType,
    Symbol,
    CurrencyName,
    Precision,
    ThousandSeparator,
    DecimalSeparator,
    AccountTypeId,
    AccountTypeName,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(ColumnEnum)]
pub enum MethodsColumn {
    Id,
    Name,
}

#[allow(dead_code)]
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
