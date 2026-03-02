//! CRUD operations for tool_call_results.

use crate::entities::tool_call_results;
use crate::error::{DbError, DbResult};
use sea_orm::*;

/// Insert a batch of tool call results, ignoring conflicts (idempotent).
pub async fn insert_batch(
    db: &DatabaseConnection,
    results: Vec<tool_call_results::ActiveModel>,
) -> DbResult<u64> {
    if results.is_empty() {
        return Ok(0);
    }

    let count = results.len() as u64;

    for chunk in results.chunks(50) {
        let result = tool_call_results::Entity::insert_many(chunk.to_vec())
            .on_conflict(
                sea_query::OnConflict::column(tool_call_results::Column::ToolCallId)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(db)
            .await;
        match result {
            Ok(_) => {}
            Err(DbErr::RecordNotInserted) => {}
            Err(e) => return Err(DbError::Database(e)),
        }
    }

    Ok(count)
}

/// Batch-load tool call results by their IDs.
/// Simple PK lookup — no LIKE scans, no JSON parsing.
pub async fn get_batch(
    db: &DatabaseConnection,
    tool_call_ids: Vec<String>,
) -> DbResult<Vec<tool_call_results::Model>> {
    if tool_call_ids.is_empty() {
        return Ok(vec![]);
    }

    tool_call_results::Entity::find()
        .filter(tool_call_results::Column::ToolCallId.is_in(tool_call_ids))
        .all(db)
        .await
        .map_err(DbError::Database)
}
