use diesel_migrations::{EmbeddedMigrations, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub mod postgres_repository;

