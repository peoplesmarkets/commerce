use chrono::{DateTime, Utc};
use deadpool_postgres::{tokio_postgres::Row, Pool};
use sea_query::{Asterisk, Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::api::peoplesmarkets::commerce::v1::MarketBoothResponse;
use crate::db::DbError;

#[derive(Iden)]
#[iden(rename = "market_booths")]
pub enum MarketBoothIden {
    Table,
    MarketBoothId,
    UserId,
    CreatedAt,
    UpdatedAt,
    Name,
    Description,
}

#[derive(Debug, Clone)]
pub struct MarketBooth {
    pub market_booth_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
}

impl MarketBooth {
    pub async fn create(
        pool: &Pool,
        user_id: &String,
        name: String,
        description: Option<String>,
    ) -> Result<Self, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(MarketBoothIden::Table)
            .columns([
                MarketBoothIden::UserId,
                MarketBoothIden::Name,
                MarketBoothIden::Description,
            ])
            .values([
                user_id.into(),
                name.into(),
                description.unwrap_or_default().into(),
            ])?
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = client.query_one(sql.as_str(), &values.as_params()).await?;

        Ok(Self::from(row))
    }

    pub async fn get(
        pool: &Pool,
        market_booth_id: &Uuid,
    ) -> Result<Option<Self>, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::select()
            .column(Asterisk)
            .from(MarketBoothIden::Table)
            .and_where(
                Expr::col(MarketBoothIden::MarketBoothId).eq(*market_booth_id),
            )
            .build_postgres(PostgresQueryBuilder);

        Ok(client
            .query_opt(sql.as_str(), &values.as_params())
            .await?
            .map(Self::from))
    }

    pub async fn list(
        pool: &Pool,
        user_id: Option<&String>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Self>, DbError> {
        let client = pool.get().await?;

        let (sql, values) = {
            let mut query = Query::select();

            query.column(Asterisk).from(MarketBoothIden::Table);

            if let Some(user_id) = user_id {
                query.and_where(Expr::col(MarketBoothIden::UserId).eq(user_id));
            }

            query
                .limit(limit)
                .offset(offset)
                .build_postgres(PostgresQueryBuilder)
        };

        let rows = client.query(sql.as_str(), &values.as_params()).await?;

        Ok(rows.iter().map(Self::from).collect())
    }

    pub async fn update(
        pool: &Pool,
        market_booth_id: &Uuid,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<Self, DbError> {
        let client = pool.get().await?;

        let (sql, values) = {
            let mut query = Query::update();
            query.table(MarketBoothIden::Table);

            if let Some(name) = name {
                query.value(MarketBoothIden::Name, name);
            }

            if let Some(description) = description {
                query.value(MarketBoothIden::Description, description);
            }

            query
                .and_where(
                    Expr::col(MarketBoothIden::MarketBoothId)
                        .eq(*market_booth_id),
                )
                .returning_all();

            query.build_postgres(PostgresQueryBuilder)
        };

        let row = client.query_one(sql.as_str(), &values.as_params()).await?;

        Ok(Self::from(row))
    }

    pub async fn delete(
        pool: &Pool,
        market_booth_id: &Uuid,
    ) -> Result<(), DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::delete()
            .from_table(MarketBoothIden::Table)
            .and_where(
                Expr::col(MarketBoothIden::MarketBoothId).eq(*market_booth_id),
            )
            .build_postgres(PostgresQueryBuilder);

        client.execute(sql.as_str(), &values.as_params()).await?;

        Ok(())
    }
}

impl From<MarketBooth> for MarketBoothResponse {
    fn from(market_booth: MarketBooth) -> Self {
        Self {
            market_booth_id: market_booth.market_booth_id.to_string(),
            user_id: market_booth.user_id,
            created_at: market_booth.created_at.timestamp(),
            updated_at: market_booth.updated_at.timestamp(),
            name: market_booth.name,
            description: market_booth.description,
        }
    }
}

impl From<&Row> for MarketBooth {
    fn from(row: &Row) -> Self {
        Self {
            market_booth_id: row.get("market_booth_id"),
            user_id: row.get("user_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            name: row.get("name"),
            description: row.get("description"),
        }
    }
}

impl From<Row> for MarketBooth {
    fn from(row: Row) -> Self {
        Self::from(&row)
    }
}
