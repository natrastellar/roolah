use super::model::{DbDecimal, Transaction, TransactionCategory, TransactionMethod};
use miette::{Context, IntoDiagnostic, Result};
use rust_decimal::Decimal;
use sqlx::{Connection, Row, SqliteConnection};
use time::Date;

use super::table_identifiers::{
    self, CategoriesColumn, MethodsColumn, TransactionsColumn,
    TransactionsWithCategoryAndMethodColumn,
};

async fn create_category(
    conn: &mut SqliteConnection,
    category: &str,
) -> Result<TransactionCategory> {
    sqlx::query(&format!(
        "INSERT OR IGNORE INTO {categories} ({name})
        VALUES (?)",
        categories = table_identifiers::CATEGORIES,
        name = CategoriesColumn::CategoryName
    ))
    .bind(category.is_empty())
    .execute(&mut *conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to insert category")?;

    get_category(conn, category).await
}

async fn get_category(conn: &mut SqliteConnection, category: &str) -> Result<TransactionCategory> {
    sqlx::query_as(&format!(
        "SELECT
            {category},
            {name}
        FROM {categories}
        WHERE {name} == ?",
        categories = table_identifiers::CATEGORIES,
        category = CategoriesColumn::CategoryId,
        name = CategoriesColumn::CategoryName,
    ))
    .bind(category)
    .fetch_one(conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to get category")
}

async fn create_method(conn: &mut SqliteConnection, method: &str) -> Result<TransactionMethod> {
    sqlx::query(&format!(
        "INSERT OR IGNORE INTO {methods} ({name})
        VALUES (?)",
        methods = table_identifiers::METHODS,
        name = MethodsColumn::MethodName
    ))
    .bind(method)
    .execute(&mut *conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to insert method")?;

    get_method(conn, method).await
}

async fn get_method(conn: &mut SqliteConnection, method: &str) -> Result<TransactionMethod> {
    sqlx::query_as(&format!(
        "SELECT
            {method},
            {name}
        FROM {methods}
        WHERE {name} = ?",
        methods = table_identifiers::METHODS,
        method = MethodsColumn::MethodId,
        name = MethodsColumn::MethodName,
    ))
    .bind(method)
    .fetch_one(conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to get method")
}

pub struct TransactionArgs<'a> {
    pub date: Date,
    pub posted_date: Option<Date>,
    pub category: &'a str,
    pub amount: Decimal,
    pub debit_account: i64,
    pub credit_account: i64,
    pub authority: &'a str,
    pub description: &'a str,
    pub method: &'a str,
    pub check_number: Option<u32>,
}

impl<'a> TransactionArgs<'a> {
    pub fn new(
        date: Date,
        amount: Decimal,
        debit_account: i64,
        credit_account: i64,
        method: &'a str,
    ) -> Self {
        Self {
            date,
            posted_date: None,
            amount,
            category: "",
            debit_account,
            credit_account,
            authority: "",
            description: "",
            method,
            check_number: None,
        }
    }
}

pub async fn create_transaction(
    conn: &mut SqliteConnection,
    args: TransactionArgs<'_>,
) -> Result<Transaction> {
    let mut transaction = conn.begin().await.into_diagnostic()?;

    let category = match args.category {
        "" => None,
        _ => Some(create_category(&mut transaction, args.category).await?),
    };
    let method = match args.method {
        "" => None,
        _ => Some(create_method(&mut transaction, args.method).await?),
    };

    let inserted = sqlx::query(&format!(
        r#"INSERT INTO {transactions} ({date}, {posted_date}, {category}, {amount}, {debit_account}, {credit_account}, {authority}, {description}, {method}, {check_number})
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING {id}
        "#,
        transactions = table_identifiers::TRANSACTIONS,
        date = TransactionsColumn::Date,
        posted_date = TransactionsColumn::PostedDate,
        category = TransactionsColumn::CategoryId,
        amount = TransactionsColumn::Amount,
        debit_account = TransactionsColumn::DebitAccount,
        credit_account = TransactionsColumn::CreditAccount,
        authority = TransactionsColumn::Authority,
        description = TransactionsColumn::Description,
        method = TransactionsColumn::MethodId,
        check_number = TransactionsColumn::CheckNumber,
        id = TransactionsColumn::Id,
    ))
    .bind(args.date)
    .bind(args.posted_date)
    .bind(category.map(|c| c.id))
    .bind(DbDecimal::from(args.amount))
    .bind(args.debit_account)
    .bind(args.credit_account)
    .bind(args.authority)
    .bind(if args.description.is_empty() { None } else { Some(args.description) })
    .bind(method.map(|m| m.id))
    .bind(args.check_number)
    .fetch_one(&mut transaction)
    .await
    .into_diagnostic()
    .wrap_err("failed to create transaction")?;

    transaction
        .commit()
        .await
        .into_diagnostic()
        .wrap_err("failed to commit")?;

    get_transaction_by_id(conn, inserted.try_get("id").into_diagnostic()?).await
}

pub async fn get_transaction_by_id(conn: &mut SqliteConnection, id: i64) -> Result<Transaction> {
    create_transactions_view(&mut *conn).await?;

    sqlx::query_as(&format!(
        "SELECT *
        FROM {transactions_view}
        WHERE {id} = ?",
        transactions_view = table_identifiers::TRANSACTIONS_WITH_CATEGORY_AND_METHOD,
        id = TransactionsWithCategoryAndMethodColumn::Id,
    ))
    .bind(id)
    .fetch_one(conn)
    .await
    .into_diagnostic()
    .wrap_err(format!("failed to get transaction with id {}", id))
}

pub async fn create_transactions_view(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(&format!(
        "CREATE VIEW IF NOT EXISTS {view} AS
        SELECT *
        FROM {transactions}
        LEFT JOIN {categories}
            USING ({category_id}) 
        LEFT JOIN {methods}
            USING ({method_id})",
        view = table_identifiers::TRANSACTIONS_WITH_CATEGORY_AND_METHOD,
        transactions = table_identifiers::TRANSACTIONS,
        category_id = TransactionsColumn::CategoryId,
        categories = table_identifiers::CATEGORIES,
        method_id = TransactionsColumn::MethodId,
        methods = table_identifiers::METHODS,
    ))
    .execute(conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to create transactions view")?;
    Ok(())
}
