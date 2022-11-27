use roolah_derive::ColumnEnum;

pub const ACCOUNT_TYPES: &str = "account_types";
pub const ACCOUNTS: &str = "accounts";
pub const ACCOUNTS_WITH_CURRENCY_AND_TYPE: &str = "accounts_with_currency_and_type";
pub const CATEGORIES: &str = "categories";
pub const CURRENCIES: &str = "currencies";
pub const METHODS: &str = "methods";
pub const TRANSACTIONS: &str = "transactions";
pub const TRANSACTIONS_WITH_CATEGORY_AND_METHOD: &str = "transactions_with_category_and_method";

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
    CategoryId,
    CategoryName,
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
    MethodId,
    MethodName,
}

#[derive(ColumnEnum)]
pub enum TransactionsColumn {
    Id,
    Date,
    PostedDate,
    CategoryId,
    Amount,
    DebitAccount,
    CreditAccount,
    Authority,
    Description,
    MethodId,
    CheckNumber,
}

#[derive(ColumnEnum)]
pub enum TransactionsWithCategoryAndMethodColumn {
    Id,
    Date,
    PostedDate,
    CategoryId,
    CategoryName,
    Amount,
    DebitAccount,
    CreditAccount,
    Authority,
    Description,
    MethodId,
    MethodName,
    CheckNumber,
}
