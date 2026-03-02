//! Migration: Add performance indexes for dashboard queries.
//!
//! Addresses three slow query patterns:
//! 1. metrics() sentiment aggregation - full table scan on 1.4M rows for 11K matching
//! 2. sessions() ordering by last activity - requires MAX(timestamp) per session
//! 3. dashboardAnalytics() tool/cost queries - need timestamp + session filtering

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Covering index for sentiment aggregation queries.
        // Only 0.8% of messages have sentiment_score, so partial index is tiny.
        // Covers: AVG(sentiment_score), COUNT(frustration_level IN (...))
        let db = manager.get_connection();
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_messages_sentiment_cover \
             ON messages(sentiment_score, frustration_level) \
             WHERE sentiment_score IS NOT NULL",
        )
        .await?;

        // 2. Index for session ordering by latest message timestamp.
        // Used by sessions() resolver to avoid loading all sessions.
        // The (session_id, timestamp DESC) order lets SQLite find MAX(timestamp)
        // per session by reading just the first entry in each group.
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_messages_session_ts_desc \
             ON messages(session_id, timestamp DESC)",
        )
        .await?;

        // 3. Index for role-filtered session summary lookups.
        // Used by enrich_sessions() to find first user message per session.
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_messages_role_session_ts \
             ON messages(role, session_id, timestamp) \
             WHERE role = 'user'",
        )
        .await?;

        // 4. Run ANALYZE to update query planner statistics
        db.execute_unprepared("ANALYZE").await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_messages_sentiment_cover")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_messages_session_ts_desc")
            .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_messages_role_session_ts")
            .await?;
        Ok(())
    }
}
