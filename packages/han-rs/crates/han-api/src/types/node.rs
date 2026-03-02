//! Relay Node interface for global ID lookups.

use async_graphql::*;
use crate::types::sessions::SessionData;
use crate::types::repo::Repo;
use crate::types::project::Project;
use crate::types::config_dir::ConfigDir;
use crate::types::native_task::NativeTask;
use crate::types::hook_execution::HookExecution;

/// Relay Node interface - any type with a globally unique ID.
#[derive(Debug, Clone, Interface)]
#[graphql(
    field(name = "id", ty = "ID"),
)]
pub enum Node {
    Session(SessionData),
    Repo(Repo),
    Project(Project),
    ConfigDir(ConfigDir),
    NativeTask(NativeTask),
    HookExecution(HookExecution),
}
