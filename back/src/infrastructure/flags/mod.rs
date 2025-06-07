use diesel_migrations::{embed_migrations, EmbeddedMigrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub mod postgres_repository;
