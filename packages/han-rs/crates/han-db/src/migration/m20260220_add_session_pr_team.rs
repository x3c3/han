//! Migration: Add PR and team fields to sessions table.
//!
//! New columns: pr_number, pr_url, team_name
//! New indexes: idx_sessions_pr_number, idx_sessions_team_name

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add pr_number column
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .add_column(ColumnDef::new(Sessions::PrNumber).integer().null())
                    .to_owned(),
            )
            .await?;

        // Add pr_url column
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .add_column(ColumnDef::new(Sessions::PrUrl).string().null())
                    .to_owned(),
            )
            .await?;

        // Add team_name column
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .add_column(ColumnDef::new(Sessions::TeamName).string().null())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_sessions_pr_number")
                    .table(Sessions::Table)
                    .col(Sessions::PrNumber)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_sessions_team_name")
                    .table(Sessions::Table)
                    .col(Sessions::TeamName)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes first
        manager
            .drop_index(
                Index::drop()
                    .name("idx_sessions_team_name")
                    .table(Sessions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_sessions_pr_number")
                    .table(Sessions::Table)
                    .to_owned(),
            )
            .await?;

        // SQLite doesn't support DROP COLUMN in older versions,
        // but SeaORM handles this via table recreation if needed.
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .drop_column(Sessions::TeamName)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .drop_column(Sessions::PrUrl)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .drop_column(Sessions::PrNumber)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    PrNumber,
    PrUrl,
    TeamName,
}
