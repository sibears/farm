use sqlx::PgPool;

use crate::domain::flags::entities::{Flag, FlagStatus, SaveFlag};
use crate::domain::flags::repository::FlagRepo;
use crate::domain::flags::FlagRepoError;
use async_trait::async_trait;
use sqlx::migrate::Migrator;
use std::path::Path;
use std::sync::Arc;

pub struct PostgresFlagRepo {
    pub conn: Arc<PgPool>,
}

impl PostgresFlagRepo {
    pub async fn new(database_url: &str) -> Self {
        let pool = PgPool::connect(database_url).await.unwrap();
        let migrator = Migrator::new(Path::new("migrations")).await.unwrap();
        migrator.run(&pool).await.unwrap();

        PostgresFlagRepo {
            conn: Arc::new(pool),
        }
    }
}

#[async_trait]
impl FlagRepo for PostgresFlagRepo {
    async fn get(&self, id_arg: i32) -> Result<Flag, FlagRepoError> {
        sqlx::query_as!(
            Flag,
            r#"SELECT 
                id, 
                flag, 
                sploit,
                team,
                created_time,
                start_waiting_time,
                status as "status: FlagStatus",
                checksystem_response
               FROM flags WHERE id = $1"#,
            id_arg
        )
        .fetch_one(&*self.conn)
        .await
        .map_err(|_| FlagRepoError::NotFound(id_arg))
    }

    async fn get_all(&self) -> Result<Arc<[Flag]>, FlagRepoError> {
        let flags = sqlx::query_as!(
            Flag,
            r#"SELECT 
                id, 
                flag, 
                sploit,
                team,
                created_time,
                start_waiting_time,
                status as "status: FlagStatus",
                checksystem_response
               FROM flags ORDER BY id"#
        )
        .fetch_all(&*self.conn)
        .await?;

        Ok(flags.into())
    }

    async fn get_all_by_status(&self, flag_status: FlagStatus) -> Result<Vec<Flag>, FlagRepoError> {
        let flags = sqlx::query_as!(
            Flag,
            r#"SELECT 
                id, 
                flag, 
                sploit,
                team,
                created_time,
                start_waiting_time,
                status as "status: FlagStatus",
                checksystem_response
               FROM flags WHERE status = $1"#,
            flag_status as FlagStatus
        )
        .fetch_all(&*self.conn)
        .await?;

        Ok(flags)
    }

    async fn save(&mut self, flag_arg: &SaveFlag) -> Result<usize, FlagRepoError> {
        let result = sqlx::query!(
            "INSERT INTO flags (flag, sploit, team, status, checksystem_response, created_time) 
             VALUES ($1, $2, $3, $4, $5, NOW())",
            flag_arg.flag,
            flag_arg.sploit,
            flag_arg.team,
            flag_arg.status as FlagStatus,
            flag_arg.checksystem_response
        )
        .execute(&*self.conn)
        .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn save_all(&mut self, flags_arg: &[SaveFlag]) -> Result<usize, FlagRepoError> {
        let mut tx = self.conn.begin().await?;
        let mut total_affected = 0;

        for flag in flags_arg {
            let result = sqlx::query!(
                "INSERT INTO flags (flag, sploit, team, status, checksystem_response, created_time) 
                 VALUES ($1, $2, $3, $4, $5, NOW())",
                flag.flag,
                flag.sploit,
                flag.team,
                flag.status as FlagStatus,
                flag.checksystem_response
            )
            .execute(&mut *tx)
            .await?;

            total_affected += result.rows_affected() as usize;
        }

        tx.commit().await?;
        Ok(total_affected)
    }

    async fn delete(&mut self, id_arg: i32) -> Result<usize, FlagRepoError> {
        let result = sqlx::query!("DELETE FROM flags WHERE id = $1", id_arg)
            .execute(&*self.conn)
            .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn delete_all(&mut self, flags_arg: &[Flag]) -> Result<usize, FlagRepoError> {
        let ids: Vec<i32> = flags_arg.iter().map(|item| item.id).collect();
        let result = sqlx::query!("DELETE FROM flags WHERE id = ANY($1)", &ids)
            .execute(&*self.conn)
            .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn update(&mut self, flag_arg: &Flag) -> Result<usize, FlagRepoError> {
        let result = sqlx::query!(
            "UPDATE flags SET 
                flag = $1, 
                sploit = $2, 
                team = $3, 
                status = $4, 
                checksystem_response = $5,
                start_waiting_time = $6
             WHERE id = $7",
            flag_arg.flag,
            flag_arg.sploit,
            flag_arg.team,
            flag_arg.status as FlagStatus,
            flag_arg.checksystem_response,
            flag_arg.start_waiting_time,
            flag_arg.id
        )
        .execute(&*self.conn)
        .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn update_all(&mut self, flags_arg: &[Flag]) -> Result<usize, FlagRepoError> {
        let mut tx = self.conn.begin().await?;
        let mut total_affected = 0;

        for flag in flags_arg {
            let result = sqlx::query!(
                "UPDATE flags SET 
                    flag = $1, 
                    sploit = $2, 
                    team = $3, 
                    status = $4, 
                    checksystem_response = $5,
                    start_waiting_time = $6
                 WHERE id = $7",
                flag.flag,
                flag.sploit,
                flag.team,
                flag.status as FlagStatus,
                flag.checksystem_response,
                flag.start_waiting_time,
                flag.id
            )
            .execute(&mut *tx)
            .await?;

            total_affected += result.rows_affected() as usize;
        }

        tx.commit().await?;
        Ok(total_affected)
    }

    async fn get_limit(&self, limit: u32) -> Result<Vec<Flag>, FlagRepoError> {
        let flags = sqlx::query_as!(
            Flag,
            r#"SELECT 
                id, 
                flag, 
                sploit,
                team,
                created_time,
                start_waiting_time,
                status as "status: FlagStatus",
                checksystem_response
               FROM flags ORDER BY id LIMIT $1"#,
            limit as i64
        )
        .fetch_all(&*self.conn)
        .await?;

        Ok(flags)
    }

    async fn get_limit_with_offset_from_start(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, FlagRepoError> {
        let flags_slice = sqlx::query_as!(
            Flag,
            r#"SELECT 
                id, 
                flag, 
                sploit,
                team,
                created_time,
                start_waiting_time,
                status as "status: FlagStatus",
                checksystem_response
               FROM flags ORDER BY id LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&*self.conn)
        .await?;

        Ok(flags_slice)
    }

    async fn get_limit_with_offset_from_end(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Flag>, FlagRepoError> {
        let flags_slice = sqlx::query_as!(
            Flag,
            r#"SELECT 
                id, 
                flag, 
                sploit,
                team,
                created_time,
                start_waiting_time,
                status as "status: FlagStatus",
                checksystem_response
               FROM flags ORDER BY id DESC LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&*self.conn)
        .await?;

        Ok(flags_slice)
    }

    async fn get_last_id(&self) -> Result<i32, FlagRepoError> {
        let result = sqlx::query!("SELECT id FROM flags ORDER BY id DESC LIMIT 1")
            .fetch_one(&*self.conn)
            .await?;

        Ok(result.id)
    }

    async fn get_limit_by_status(
        &self,
        flag_status: FlagStatus,
        limit: u32,
    ) -> Result<Vec<Flag>, FlagRepoError> {
        let flags_by_status = sqlx::query_as!(
            Flag,
            r#"SELECT 
                id, 
                flag, 
                sploit,
                team,
                created_time,
                start_waiting_time,
                status as "status: FlagStatus",
                checksystem_response
               FROM flags WHERE status = $1 LIMIT $2"#,
            flag_status as FlagStatus,
            limit as i64
        )
        .fetch_all(&*self.conn)
        .await?;

        Ok(flags_by_status)
    }

    async fn get_all_by_id(&self, ids: &[i32]) -> Result<Vec<Flag>, FlagRepoError> {
        let flags_by_ids = sqlx::query_as!(
            Flag,
            r#"SELECT 
                id, 
                flag, 
                sploit,
                team,
                created_time,
                start_waiting_time,
                status as "status: FlagStatus",
                checksystem_response
               FROM flags WHERE id = ANY($1)"#,
            ids
        )
        .fetch_all(&*self.conn)
        .await?;

        Ok(flags_by_ids)
    }

    async fn get_total_flags(&self) -> Result<i64, FlagRepoError> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM flags")
            .fetch_one(&*self.conn)
            .await?;

        Ok(result.count.unwrap_or(0))
    }

    async fn get_total_flags_by_status(
        &self,
        flag_status: FlagStatus,
    ) -> Result<i64, FlagRepoError> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM flags WHERE status = $1",
            flag_status as FlagStatus
        )
        .fetch_one(&*self.conn)
        .await?;

        Ok(result.count.unwrap_or(0))
    }
}
