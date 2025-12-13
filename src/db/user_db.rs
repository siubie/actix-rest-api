use crate::models::user::User;
use sqlx::MySqlPool;

pub struct UserDb;

impl UserDb {
    pub async fn create_user(pool: &MySqlPool, name: &str, email: &str) -> Result<User, sqlx::Error> {
        let result = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, email, created_at, updated_at)
            VALUES (?, ?, NOW(), NOW())
            "#,
        )
        .bind(name)
        .bind(email)
        .fetch_one(pool)
        .await;

        match result {
            Ok(user) => Ok(user),
            Err(_) => {
                // Fetch the inserted user by email since MySQL doesn't return the inserted row
                sqlx::query_as::<_, User>(
                    r#"
                    SELECT id, name, email, created_at, updated_at
                    FROM users
                    WHERE email = ?
                    ORDER BY id DESC
                    LIMIT 1
                    "#,
                )
                .bind(email)
                .fetch_one(pool)
                .await
            }
        }
    }

    pub async fn get_user_by_id(pool: &MySqlPool, id: i64) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn get_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update_user(
        pool: &MySqlPool,
        id: i64,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let mut query = String::from("UPDATE users SET updated_at = NOW()");
        let mut bindings = Vec::new();

        if let Some(n) = name {
            query.push_str(", name = ?");
            bindings.push(n);
        }

        if let Some(e) = email {
            query.push_str(", email = ?");
            bindings.push(e);
        }

        query.push_str(" WHERE id = ?");

        let mut q = sqlx::query(&query);
        for binding in bindings {
            q = q.bind(binding);
        }
        q = q.bind(id);

        q.execute(pool).await?;

        Self::get_user_by_id(pool, id).await
    }

    pub async fn delete_user(pool: &MySqlPool, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }
}
