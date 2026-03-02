//! Entity: tool_call_results
//!
//! Pre-computed index of tool result data extracted at indexing time.
//! Keyed by tool_call_id for O(1) batch lookups from ToolUseBlock.result resolver.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tool_call_results")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub tool_call_id: String,
    pub session_id: String,
    pub message_id: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub is_error: bool,
    pub has_image: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sessions::Entity",
        from = "Column::SessionId",
        to = "super::sessions::Column::Id"
    )]
    Session,
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
