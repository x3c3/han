//! Project GraphQL type.

use async_graphql::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};

use crate::node::encode_global_id;
use han_graphql_derive::GraphQLEntity;

/// Transform: nonzero Option<i32> → bool.
fn nonzero_to_bool(v: Option<i32>) -> bool {
    v.unwrap_or(0) != 0
}

/// Project data for GraphQL resolution.
#[derive(Debug, Clone, SimpleObject, GraphQLEntity)]
#[graphql(complex, name = "Project")]
#[graphql_entity(
    model = "han_db::entities::projects::Model",
    entity = "han_db::entities::projects::Entity",
    columns = "han_db::entities::projects::Column",
    type_name = "Project",
)]
pub struct Project {
    #[graphql(skip)]
    #[graphql_entity(skip, source_field = "id")]
    pub raw_id: String,

    pub slug: String,
    pub path: String,
    pub name: String,
    pub repo_id: Option<String>,
    pub relative_path: Option<String>,

    #[graphql_entity(transform = "nonzero_to_bool", source_field = "is_worktree")]
    pub is_worktree: bool,

    pub created_at: String,
    pub updated_at: String,
}

#[ComplexObject]
impl Project {
    /// Global ID (required by Node interface).
    pub async fn id(&self) -> ID { encode_global_id("Project", &self.raw_id) }

    // Backwards-compatible fields for browse-client
    /// Alias for raw_id.
    async fn project_id(&self) -> &str { &self.raw_id }

    /// Total sessions count.
    async fn total_sessions(&self, ctx: &Context<'_>) -> Result<Option<i32>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let count = han_db::entities::sessions::Entity::find()
            .filter(han_db::entities::sessions::Column::ProjectId.eq(&self.raw_id))
            .count(db)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(Some(count as i32))
    }

    /// Session count.
    async fn session_count(&self, ctx: &Context<'_>) -> Result<Option<i32>> {
        self.total_sessions(ctx).await
    }

    /// Last activity timestamp.
    async fn last_activity(&self) -> Option<&str> { Some(&self.updated_at) }
    /// Worktrees (stub).
    async fn worktrees(&self) -> Option<Vec<Project>> { Some(vec![]) }
    /// Subdirectory projects (stub).
    async fn subdirs(&self) -> Option<Vec<Project>> { Some(vec![]) }
    /// Installed plugins (stub).
    async fn plugins(&self) -> Option<Vec<crate::types::plugin::Plugin>> { Some(vec![]) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use han_db::entities::projects;

    fn make_model(is_worktree: Option<i32>, repo_id: Option<String>) -> projects::Model {
        projects::Model {
            id: "proj-1".into(),
            slug: "my-project".into(),
            path: "/home/user/project".into(),
            name: "My Project".into(),
            repo_id,
            relative_path: Some("packages/core".into()),
            is_worktree,
            source_config_dir: None,
            created_at: "2025-01-01T00:00:00Z".into(),
            updated_at: "2025-01-02T00:00:00Z".into(),
        }
    }

    #[test]
    fn from_model_maps_all_fields() {
        let m = make_model(Some(1), Some("repo-1".into()));
        let p = Project::from(m);
        assert_eq!(p.raw_id, "proj-1");
        assert_eq!(p.slug, "my-project");
        assert_eq!(p.path, "/home/user/project");
        assert_eq!(p.name, "My Project");
        assert_eq!(p.repo_id, Some("repo-1".into()));
        assert_eq!(p.relative_path, Some("packages/core".into()));
        assert!(p.is_worktree);
        assert_eq!(p.created_at, "2025-01-01T00:00:00Z");
        assert_eq!(p.updated_at, "2025-01-02T00:00:00Z");
    }

    #[test]
    fn is_worktree_true_when_nonzero() {
        assert!(Project::from(make_model(Some(1), None)).is_worktree);
        assert!(Project::from(make_model(Some(42), None)).is_worktree);
    }

    #[test]
    fn is_worktree_false_when_zero() {
        assert!(!Project::from(make_model(Some(0), None)).is_worktree);
    }

    #[test]
    fn is_worktree_false_when_none() {
        assert!(!Project::from(make_model(None, None)).is_worktree);
    }

    #[test]
    fn optional_fields_none() {
        let m = projects::Model {
            id: "p".into(),
            slug: "s".into(),
            path: "/p".into(),
            name: "n".into(),
            repo_id: None,
            relative_path: None,
            is_worktree: None,
            source_config_dir: None,
            created_at: "".into(),
            updated_at: "".into(),
        };
        let p = Project::from(m);
        assert!(p.repo_id.is_none());
        assert!(p.relative_path.is_none());
    }

    #[test]
    fn filter_default_is_empty() {
        let f = ProjectFilter::default();
        assert!(f.slug.is_none());
        assert!(f.name.is_none());
        assert!(f.and.is_none());
        assert!(f.or.is_none());
        assert!(f.not.is_none());
    }

    #[test]
    fn order_by_default_is_empty() {
        let o = ProjectOrderBy::default();
        assert!(o.slug.is_none());
        assert!(o.name.is_none());
    }

    #[test]
    fn filter_to_condition_no_panic() {
        let f = ProjectFilter {
            slug: Some(crate::filters::types::StringFilter {
                eq: Some("test".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let _cond = f.to_condition();
    }

    #[test]
    fn filter_logical_combinators() {
        let f = ProjectFilter {
            and: Some(vec![
                ProjectFilter {
                    name: Some(crate::filters::types::StringFilter {
                        contains: Some("test".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ]),
            or: Some(vec![ProjectFilter::default()]),
            not: Some(Box::new(ProjectFilter::default())),
            ..Default::default()
        };
        let _cond = f.to_condition();
    }
}
