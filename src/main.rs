use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = "sqlite:test.db";

    let pool = SqlitePool::connect(db_url).await?;

    let new_todo_id = add_todo(&pool, "hello".to_string()).await?;

    println!("New record id: {:?}", new_todo_id);

    get_todo(&pool, new_todo_id).await?;

    Ok(())
}

async fn add_todo(pool: &SqlitePool, desc: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    let id = sqlx::query!(
        r#"
            INSERT INTO test ( descriptions ) VALUES ( ?1 )
        "#,
        desc,
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn get_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<()> {
    // let mut conn = pool.acquire().await?;

    let todo = sqlx::query!(
        r#"
            SELECT id, descriptions, done
            FROM test    
            WHERE id = $1
        "#,
        id,
    )
    .fetch_one(pool)
    .await?;

    println!(
        "- [{}] {}: {}",
        if todo.done { "x" } else { " " },
        todo.id,
        &todo.descriptions,
    );

    Ok(())
}
