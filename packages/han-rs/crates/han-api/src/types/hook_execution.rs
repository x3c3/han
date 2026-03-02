//! Hook execution GraphQL type.

use async_graphql::*;
use crate::node::encode_global_id;
use crate::connection::PageInfo;
use han_graphql_derive::GraphQLEntity;

/// Transform: i32 → bool (nonzero is true).
fn nonzero_i32_to_bool(v: i32) -> bool {
    v != 0
}

/// Hook execution data.
#[derive(Debug, Clone, SimpleObject, GraphQLEntity)]
#[graphql(complex, name = "HookExecution")]
#[graphql_entity(
    model = "han_db::entities::hook_executions::Model",
    entity = "han_db::entities::hook_executions::Entity",
    columns = "han_db::entities::hook_executions::Column",
    type_name = "HookExecution",
)]
pub struct HookExecution {
    #[graphql(skip)]
    #[graphql_entity(skip, source_field = "id")]
    pub raw_id: String,

    pub orchestration_id: Option<String>,
    pub session_id: Option<String>,
    pub task_id: Option<String>,
    pub hook_type: String,
    pub hook_name: String,
    pub hook_source: Option<String>,
    pub directory: Option<String>,
    pub duration_ms: i32,
    pub exit_code: i32,

    #[graphql_entity(transform = "nonzero_i32_to_bool")]
    pub passed: bool,

    pub output: Option<String>,
    pub error: Option<String>,
    pub command: Option<String>,
    pub executed_at: String,
    pub status: Option<String>,
}

#[ComplexObject]
impl HookExecution {
    /// Global ID.
    pub async fn id(&self) -> ID { encode_global_id("HookExecution", &self.raw_id) }
    /// Timestamp alias for executed_at (browse-client compat).
    async fn timestamp(&self) -> &str { &self.executed_at }
}

/// Hook execution edge.
#[derive(Debug, Clone, SimpleObject)]
pub struct HookExecutionEdge {
    pub node: HookExecution,
    pub cursor: String,
}

/// Hook execution connection.
#[derive(Debug, Clone, SimpleObject)]
pub struct HookExecutionConnection {
    pub edges: Vec<HookExecutionEdge>,
    pub page_info: PageInfo,
    pub total_count: i32,
}

/// Hook statistics for a session.
#[derive(Debug, Clone, SimpleObject)]
pub struct HookStats {
    pub total_hooks: Option<i32>,
    pub passed_hooks: Option<i32>,
    pub failed_hooks: Option<i32>,
    pub total_duration_ms: Option<i32>,
    pub pass_rate: Option<f64>,
    pub by_hook_type: Option<Vec<HookTypeStat>>,
}

/// Hook type statistics breakdown.
#[derive(Debug, Clone, SimpleObject)]
pub struct HookTypeStat {
    pub hook_type: Option<String>,
    pub total: Option<i32>,
    pub passed: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use han_db::entities::hook_executions;

    fn make_model(passed: i32) -> hook_executions::Model {
        hook_executions::Model {
            id: "he-1".into(),
            orchestration_id: Some("orch-1".into()),
            session_id: Some("sess-1".into()),
            task_id: Some("task-1".into()),
            hook_type: "Stop".into(),
            hook_name: "biome".into(),
            hook_source: Some("validation/biome".into()),
            directory: Some("/project".into()),
            duration_ms: 1500,
            exit_code: 0,
            passed,
            output: Some("All checks passed".into()),
            error: None,
            if_changed: None,
            command: Some("npx biome check".into()),
            executed_at: "2025-01-01T12:00:00Z".into(),
            status: Some("completed".into()),
            consecutive_failures: None,
            max_attempts: None,
            pid: None,
            plugin_root: None,
        }
    }

    #[test]
    fn from_model_maps_all_fields() {
        let he = HookExecution::from(make_model(1));
        assert_eq!(he.raw_id, "he-1");
        assert_eq!(he.orchestration_id, Some("orch-1".into()));
        assert_eq!(he.session_id, Some("sess-1".into()));
        assert_eq!(he.task_id, Some("task-1".into()));
        assert_eq!(he.hook_type, "Stop");
        assert_eq!(he.hook_name, "biome");
        assert_eq!(he.hook_source, Some("validation/biome".into()));
        assert_eq!(he.directory, Some("/project".into()));
        assert_eq!(he.duration_ms, 1500);
        assert_eq!(he.exit_code, 0);
        assert!(he.passed);
        assert_eq!(he.output, Some("All checks passed".into()));
        assert!(he.error.is_none());
        assert_eq!(he.command, Some("npx biome check".into()));
        assert_eq!(he.executed_at, "2025-01-01T12:00:00Z");
        assert_eq!(he.status, Some("completed".into()));
    }

    #[test]
    fn passed_true_when_nonzero() {
        assert!(HookExecution::from(make_model(1)).passed);
        assert!(HookExecution::from(make_model(42)).passed);
    }

    #[test]
    fn passed_false_when_zero() {
        assert!(!HookExecution::from(make_model(0)).passed);
    }

    #[test]
    fn optional_fields_none() {
        let m = hook_executions::Model {
            id: "h".into(),
            orchestration_id: None,
            session_id: None,
            task_id: None,
            hook_type: "Stop".into(),
            hook_name: "test".into(),
            hook_source: None,
            directory: None,
            duration_ms: 0,
            exit_code: 1,
            passed: 0,
            output: None,
            error: None,
            if_changed: None,
            command: None,
            executed_at: "".into(),
            status: None,
            consecutive_failures: None,
            max_attempts: None,
            pid: None,
            plugin_root: None,
        };
        let he = HookExecution::from(m);
        assert!(he.orchestration_id.is_none());
        assert!(he.session_id.is_none());
        assert!(he.task_id.is_none());
        assert!(he.hook_source.is_none());
        assert!(he.directory.is_none());
        assert!(he.output.is_none());
        assert!(he.error.is_none());
        assert!(he.command.is_none());
        assert!(he.status.is_none());
    }

    #[test]
    fn hook_stats_construction() {
        let stats = HookStats {
            total_hooks: Some(10),
            passed_hooks: Some(8),
            failed_hooks: Some(2),
            total_duration_ms: Some(5000),
            pass_rate: Some(0.8),
            by_hook_type: Some(vec![HookTypeStat {
                hook_type: Some("Stop".into()),
                total: Some(10),
                passed: Some(8),
            }]),
        };
        assert_eq!(stats.total_hooks, Some(10));
        assert_eq!(stats.passed_hooks, Some(8));
        assert_eq!(stats.failed_hooks, Some(2));
    }

    #[test]
    fn filter_default_is_empty() {
        let f = HookExecutionFilter::default();
        assert!(f.hook_type.is_none());
        assert!(f.hook_name.is_none());
        assert!(f.passed.is_none());
        assert!(f.and.is_none());
        assert!(f.or.is_none());
        assert!(f.not.is_none());
    }

    #[test]
    fn order_by_default_is_empty() {
        let o = HookExecutionOrderBy::default();
        assert!(o.hook_type.is_none());
        assert!(o.hook_name.is_none());
        assert!(o.duration_ms.is_none());
    }

    #[test]
    fn filter_to_condition_no_panic() {
        let f = HookExecutionFilter {
            hook_type: Some(crate::filters::types::StringFilter {
                eq: Some("Stop".into()),
                ..Default::default()
            }),
            passed: Some(crate::filters::types::BoolFilter {
                eq: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };
        let _cond = f.to_condition();
    }
}
