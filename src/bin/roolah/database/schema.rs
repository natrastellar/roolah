use super::table_identifiers::{
    self, AccountTypesColumn, AccountsColumn, CategoriesColumn, CurrenciesColumn, MethodsColumn,
    TransactionsColumn,
};
use miette::{IntoDiagnostic, Result, WrapErr};
use sqlx::{Connection, SqliteConnection};

macro_rules! drop_existing_tables {
    ($($table:expr),*) => {
        (|| {
            let mut s = String::new();
            $(
                s += "DROP TABLE IF EXISTS ";
                s += $table;
                s += ";";
            )*
            s
        })()
    };
}

macro_rules! drop_existing_views {
    ($($view:expr),*) => {
        (|| {
            let mut s = String::new();
            $(
                s += "DROP VIEW IF EXISTS ";
                s += $view;
                s += ";";
            )*
            s
        })()
    };
}

#[allow(clippy::redundant_closure_call)]
pub async fn drop_tables(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(&drop_existing_tables!(
        table_identifiers::TRANSACTIONS,
        table_identifiers::ACCOUNTS,
        table_identifiers::ACCOUNT_TYPES,
        table_identifiers::CURRENCIES,
        table_identifiers::CATEGORIES,
        table_identifiers::METHODS
    ))
    .execute(&mut *conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to drop tables")?;

    sqlx::query(&drop_existing_views!(
        table_identifiers::ACCOUNTS_WITH_CURRENCY_AND_TYPE,
        table_identifiers::TRANSACTIONS_WITH_CATEGORY_AND_METHOD
    ))
    .execute(conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to drop views")?;

    Ok(())
}

pub async fn create_tables(conn: &mut SqliteConnection) -> Result<()> {
    let mut transaction = conn.begin().await.into_diagnostic()?;

    create_currencies_table(&mut transaction).await?;
    create_account_types_table(&mut transaction).await?;
    create_accounts_table(&mut transaction).await?;
    create_categories_table(&mut transaction).await?;
    create_methods_table(&mut transaction).await?;
    create_transactions_table(&mut transaction).await?;

    transaction.commit().await.into_diagnostic()
}

async fn create_currencies_table(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {currencies} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {symbol} TEXT
                NOT NULL,
            {name} TEXT
                UNIQUE
                NOT NULL
                CHECK ({name} != ''),
            {precision} INTEGER
                NOT NULL
                DEFAULT 2,
            {thousand_separator} TEXT
                NOT NULL
                DEFAULT ',',
            {decimal_separator} TEXT
                NOT NULL
                DEFAULT '.'
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS currency_name ON {currencies} ({name})",
        currencies = table_identifiers::CURRENCIES,
        id = CurrenciesColumn::Id,
        symbol = CurrenciesColumn::Symbol,
        name = CurrenciesColumn::Name,
        precision = CurrenciesColumn::Precision,
        thousand_separator = CurrenciesColumn::ThousandSeparator,
        decimal_separator = CurrenciesColumn::DecimalSeparator,
    ))
    .execute(conn)
    .await
    .into_diagnostic()?;
    Ok(())
}

async fn create_account_types_table(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {account_types} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {name} TEXT
                UNIQUE
                NOT NULL
                CHECK ({name} != '')
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS account_type_name ON {account_types} ({name})",
        account_types = table_identifiers::ACCOUNT_TYPES,
        id = AccountTypesColumn::Id,
        name = AccountTypesColumn::Name
    ))
    .execute(conn)
    .await
    .into_diagnostic()?;
    Ok(())
}

async fn create_accounts_table(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {accounts} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {name} TEXT
                UNIQUE
                NOT NULL
                CHECK ({name} != ''),
            {currency} INTEGER
                NOT NULL
                REFERENCES {currencies}({currency_id})
                ON DELETE RESTRICT,
            {balance} TEXT
                NOT NULL
                DEFAULT '0'
                CHECK ({balance} != ''),
            {posted_balance} TEXT
                NOT NULL
                DEFAULT '0'
                CHECK ({posted_balance} != ''),
            {account_type} INTEGER
                NOT NULL
                REFERENCES {account_types}({account_type_id})
                ON DELETE RESTRICT
                CHECK ({account_type} != '')
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS account_name ON {accounts} ({name})",
        accounts = table_identifiers::ACCOUNTS,
        id = AccountsColumn::Id,
        name = AccountsColumn::Name,
        currency = AccountsColumn::Currency,
        currencies = table_identifiers::CURRENCIES,
        currency_id = CurrenciesColumn::Id,
        balance = AccountsColumn::Balance,
        posted_balance = AccountsColumn::PostedBalance,
        account_type = AccountsColumn::AccountType,
        account_types = table_identifiers::ACCOUNT_TYPES,
        account_type_id = AccountTypesColumn::Id
    ))
    .execute(conn)
    .await
    .into_diagnostic()?;
    Ok(())
}

async fn create_categories_table(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {categories} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {name} TEXT
                UNIQUE
                NOT NULL
                CHECK ({name} != '')
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS category_name ON {categories} ({name})",
        categories = table_identifiers::CATEGORIES,
        id = CategoriesColumn::CategoryId,
        name = CategoriesColumn::CategoryName,
    ))
    .execute(conn)
    .await
    .into_diagnostic()?;
    Ok(())
}

async fn create_methods_table(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {methods} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {name} TEXT
                UNIQUE
                NOT NULL
                CHECK ({name} != '')
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS method_name ON {methods} ({name})",
        methods = table_identifiers::METHODS,
        id = MethodsColumn::MethodId,
        name = MethodsColumn::MethodName,
    ))
    .execute(conn)
    .await
    .into_diagnostic()?;
    Ok(())
}

async fn create_transactions_table(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {transactions} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {date} TEXT
                NOT NULL
                DEFAULT CURRENT_DATE
                CHECK ({date} != ''),
            {posted_date} TEXT
                CHECK ({posted_date} != '')
                CHECK ({posted_date} IS NULL
                    OR strftime('%s', {posted_date}) >= strftime('%s', {date})
                ),
            {category} INTEGER
                REFERENCES {categories}({category_id})
                ON DELETE SET NULL,
            {amount} TEXT
                NOT NULL
                CHECK ({amount} != ''),
            {debit_account} INTEGER
                REFERENCES {accounts}({account_id})
                ON DELETE SET NULL,
            {credit_account} INTEGER
                REFERENCES {accounts}({account_id})
                ON DELETE SET NULL,
            {authority} TEXT
                NOT NULL,
            {description} TEXT
                NOT NULL,
            {method} INTEGER
                REFERENCES {methods}({method_id})
                ON DELETE SET NULL,
            {check_number} INTEGER
                CHECK ({check_number} IS NULL
                    OR ({debit_account} NOT NULL)
                ),
            UNIQUE ({check_number}, {debit_account})
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_date ON {transactions} ({date});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_posted_date ON {transactions} ({posted_date});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_category ON {transactions} ({category});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_amount ON {transactions} ({amount});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_debit_account ON {transactions} ({debit_account});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_credit_account ON {transactions} ({credit_account});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_authority ON {transactions} ({authority});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_description ON {transactions} ({description});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_method ON {transactions} ({method});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_check_number ON {transactions} ({check_number});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_debit_account_change_magnitude ON {transactions} ({debit_account}, abs({amount}));
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_category_change_magnitude ON {transactions} ({category}, abs({amount}))",
        transactions = table_identifiers::TRANSACTIONS,
        id = TransactionsColumn::Id,
        date = TransactionsColumn::Date,
        posted_date = TransactionsColumn::PostedDate,
        category = TransactionsColumn::CategoryId,
        categories = table_identifiers::CATEGORIES,
        category_id = CategoriesColumn::CategoryId,
        amount = TransactionsColumn::Amount,
        debit_account = TransactionsColumn::DebitAccount,
        accounts = table_identifiers::ACCOUNTS,
        account_id = AccountsColumn::Id,
        credit_account = TransactionsColumn::CreditAccount,
        authority = TransactionsColumn::Authority,
        description = TransactionsColumn::Description,
        method = TransactionsColumn::MethodId,
        methods = table_identifiers::METHODS,
        method_id = MethodsColumn::MethodId,
        check_number = TransactionsColumn::CheckNumber,
    ))
    .execute(conn)
    .await
    .into_diagnostic()?;
    Ok(())
}
