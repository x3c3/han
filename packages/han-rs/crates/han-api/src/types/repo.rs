//! Repo (git repository) GraphQL type.

use async_graphql::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};

use crate::node::encode_global_id;
use han_graphql_derive::GraphQLEntity;

/// Repo data for GraphQL resolution.
#[derive(Debug, Clone, SimpleObject, GraphQLEntity)]
#[graphql(complex, name = "Repo")]
#[graphql_entity(
    model = "han_db::entities::repos::Model",
    entity = "han_db::entities::repos::Entity",
    columns = "han_db::entities::repos::Column",
    type_name = "Repo",
)]
pub struct Repo {
    #[graphql(skip)]
    #[graphql_entity(skip, source_field = "id")]
    pub raw_id: String,

    pub remote: String,
    pub name: String,
    pub default_branch: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[ComplexObject]
impl Repo {
    /// Repo global ID (required by Node interface).
    pub async fn id(&self) -> ID {
        encode_global_id("Repo", &self.raw_id)
    }

    // Backwards-compatible fields for browse-client
    /// Alias for raw_id.
    async fn repo_id(&self) -> &str { &self.raw_id }
    /// Repo path (uses remote).
    async fn path(&self) -> &str { &self.remote }

    /// Total sessions count across all projects in this repo.
    async fn total_sessions(&self, ctx: &Context<'_>) -> Result<Option<i32>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let project_ids: Vec<String> = han_db::entities::projects::Entity::find()
            .filter(han_db::entities::projects::Column::RepoId.eq(&self.raw_id))
            .select_only()
            .column(han_db::entities::projects::Column::Id)
            .into_tuple::<String>()
            .all(db)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        if project_ids.is_empty() {
            return Ok(Some(0));
        }

        let count = han_db::entities::sessions::Entity::find()
            .filter(han_db::entities::sessions::Column::ProjectId.is_in(project_ids))
            .count(db)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(Some(count as i32))
    }

    /// Last activity timestamp.
    async fn last_activity(&self) -> Option<&str> { Some(&self.updated_at) }

    /// Projects in this repo.
    async fn projects(&self, ctx: &Context<'_>) -> Result<Option<Vec<crate::types::project::Project>>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let models = han_db::entities::projects::Entity::find()
            .filter(han_db::entities::projects::Column::RepoId.eq(&self.raw_id))
            .all(db)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(Some(models.into_iter().map(crate::types::project::Project::from).collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use han_db::entities::repos;

    #[test]
    fn from_model_maps_all_fields() {
        let m = repos::Model {
            id: "repo-1".into(),
            remote: "https://github.com/org/repo.git".into(),
            name: "repo".into(),
            default_branch: Some("main".into()),
            created_at: "2025-01-01".into(),
            updated_at: "2025-01-02".into(),
        };
        let r = Repo::from(m);
        assert_eq!(r.raw_id, "repo-1");
        assert_eq!(r.remote, "https://github.com/org/repo.git");
        assert_eq!(r.name, "repo");
        assert_eq!(r.default_branch, Some("main".into()));
        assert_eq!(r.created_at, "2025-01-01");
        assert_eq!(r.updated_at, "2025-01-02");
    }

    #[test]
    fn default_branch_none() {
        let m = repos::Model {
            id: "r".into(),
            remote: "git@host:r.git".into(),
            name: "r".into(),
            default_branch: None,
            created_at: "".into(),
            updated_at: "".into(),
        };
        assert!(Repo::from(m).default_branch.is_none());
    }

    #[test]
    fn filter_default_is_empty() {
        let f = RepoFilter::default();
        assert!(f.remote.is_none());
        assert!(f.name.is_none());
        assert!(f.and.is_none());
        assert!(f.or.is_none());
        assert!(f.not.is_none());
    }

    #[test]
    fn order_by_default_is_empty() {
        let o = RepoOrderBy::default();
        assert!(o.remote.is_none());
        assert!(o.name.is_none());
    }

    #[test]
    fn filter_to_condition_no_panic() {
        let f = RepoFilter {
            name: Some(crate::filters::types::StringFilter {
                eq: Some("my-repo".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let _cond = f.to_condition();
    }
}
