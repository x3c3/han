//! Config directory GraphQL type.

use async_graphql::*;
use crate::node::encode_global_id;
use han_graphql_derive::GraphQLEntity;

/// Transform: i32 → bool (nonzero is true).
fn nonzero_i32_to_bool(v: i32) -> bool {
    v != 0
}

/// Config directory data.
#[derive(Debug, Clone, SimpleObject, GraphQLEntity)]
#[graphql(complex, name = "ConfigDir")]
#[graphql_entity(
    model = "han_db::entities::config_dirs::Model",
    entity = "han_db::entities::config_dirs::Entity",
    columns = "han_db::entities::config_dirs::Column",
    type_name = "ConfigDir",
)]
pub struct ConfigDir {
    #[graphql(skip)]
    #[graphql_entity(skip, source_field = "id")]
    pub raw_id: String,

    pub path: String,
    pub name: Option<String>,

    #[graphql_entity(transform = "nonzero_i32_to_bool")]
    pub is_default: bool,

    pub registered_at: String,
    pub last_indexed_at: Option<String>,
    pub session_count: Option<i32>,
}

#[ComplexObject]
impl ConfigDir {
    /// Global ID.
    pub async fn id(&self) -> ID { encode_global_id("ConfigDir", &self.raw_id) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use han_db::entities::config_dirs;

    fn make_model(is_default: i32) -> config_dirs::Model {
        config_dirs::Model {
            id: "cd-1".into(),
            path: "/home/user/.claude".into(),
            name: Some("default".into()),
            is_default,
            registered_at: "2025-01-01".into(),
            last_indexed_at: Some("2025-01-02".into()),
            session_count: Some(5),
        }
    }

    #[test]
    fn from_model_maps_all_fields() {
        let cd = ConfigDir::from(make_model(1));
        assert_eq!(cd.raw_id, "cd-1");
        assert_eq!(cd.path, "/home/user/.claude");
        assert_eq!(cd.name, Some("default".into()));
        assert!(cd.is_default);
        assert_eq!(cd.registered_at, "2025-01-01");
        assert_eq!(cd.last_indexed_at, Some("2025-01-02".into()));
        assert_eq!(cd.session_count, Some(5));
    }

    #[test]
    fn is_default_true_when_nonzero() {
        assert!(ConfigDir::from(make_model(1)).is_default);
        assert!(ConfigDir::from(make_model(99)).is_default);
    }

    #[test]
    fn is_default_false_when_zero() {
        assert!(!ConfigDir::from(make_model(0)).is_default);
    }

    #[test]
    fn optional_fields_none() {
        let m = config_dirs::Model {
            id: "c".into(),
            path: "/p".into(),
            name: None,
            is_default: 0,
            registered_at: "".into(),
            last_indexed_at: None,
            session_count: None,
        };
        let cd = ConfigDir::from(m);
        assert!(cd.name.is_none());
        assert!(cd.last_indexed_at.is_none());
        assert!(cd.session_count.is_none());
    }

    #[test]
    fn filter_default_is_empty() {
        let f = ConfigDirFilter::default();
        assert!(f.path.is_none());
        assert!(f.name.is_none());
        assert!(f.is_default.is_none());
        assert!(f.and.is_none());
        assert!(f.or.is_none());
        assert!(f.not.is_none());
    }

    #[test]
    fn order_by_default_is_empty() {
        let o = ConfigDirOrderBy::default();
        assert!(o.path.is_none());
        assert!(o.name.is_none());
    }

    #[test]
    fn filter_to_condition_no_panic() {
        let f = ConfigDirFilter {
            path: Some(crate::filters::types::StringFilter {
                contains: Some("/claude".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let _cond = f.to_condition();
    }
}
