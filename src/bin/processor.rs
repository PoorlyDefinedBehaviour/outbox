use std::{fmt::Write, str::FromStr, time::Duration};

use anyhow::{anyhow, Context, Result};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[repr(i32)]
enum OperationStatus {
    Pending = 0,
    Completed = 1,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&env::<String>("DATABASE_URL")?)
        .await?;

    loop {
        if let Err(err) = process_outbox(&pool).await {
            eprintln!("error processing outbox. error={err:?}");
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn process_outbox(pool: &Pool<MySql>) -> Result<()> {
    let operations = sqlx::query!(
        "
        SELECT * FROM operations 
        WHERE status = ?
        ORDER BY id DESC 
        LIMIT 10
        ",
        OperationStatus::Pending as i32
    )
    .fetch_all(pool)
    .await?;

    if operations.is_empty() {
        return Ok(());
    }

    println!("got {} operations to process", operations.len());

    for operation in operations.iter() {
        println!("processing: {:?}", operation);
    }

    let query_str = format!(
        "
            UPDATE operations
            SET status = ?
            WHERE id in ({placeholders})
        ",
        placeholders = placeholders(operations.len())?
    );

    let mut query = sqlx::query(&query_str);

    query = query.bind(OperationStatus::Completed as i32);

    for operation in operations {
        query = query.bind(operation.id);
    }

    query.execute(pool).await?;

    Ok(())
}

fn placeholders(count: usize) -> Result<String, std::fmt::Error> {
    let mut buffer = String::new();

    for i in 0..count {
        if i == count - 1 {
            write!(&mut buffer, "?")?;
        } else {
            write!(&mut buffer, "?, ")?;
        }
    }

    Ok(buffer)
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
