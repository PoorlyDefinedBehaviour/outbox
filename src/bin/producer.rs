use std::{str::FromStr, time::Duration};

use anyhow::{anyhow, Context, Result};
use rand::Rng;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[repr(i32)]
enum EntityStatus {
    Active = 0,
}

#[repr(i32)]
enum OperationStatus {
    Pending = 0,
}

#[repr(i32)]
enum Operation {
    Archive = 0,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&env::<String>("DATABASE_URL")?)
        .await?;

    loop {
        if let Err(err) = generate_events(&pool).await {
            eprintln!("error generating events. error={err:?}");
        }

        tokio::time::sleep(Duration::from_secs(rand::thread_rng().gen_range(1..=3))).await;
    }
}

async fn generate_events(pool: &Pool<MySql>) -> Result<()> {
    println!("generating events");

    let mut tx = pool.begin().await?;

    let result = sqlx::query!(
        "INSERT INTO entities(status) VALUES(?)",
        EntityStatus::Active as i32
    )
    .execute(&mut tx)
    .await?;
    let entity_id = result.last_insert_id();

    sqlx::query!(
        "INSERT INTO operations (entity_id, operation, status) VALUES(?, ?, ?)",
        entity_id,
        Operation::Archive as i32,
        OperationStatus::Pending as i32
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

#[tracing::instrument(name = "config::env", skip_all, fields(key = %key))]
fn env<T: FromStr>(key: &str) -> Result<T>
where
    <T as FromStr>::Err: std::error::Error,
{
    let value = std::env::var(key)
        .with_context(|| format!("unable to find env variable. key={key}"))?
        .parse()
        .map_err(|err| anyhow!("unable to parse value into expected type error={:?}", err))?;
    Ok(value)
}
