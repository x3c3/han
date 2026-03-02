//! Filter input types for GraphQL query filtering.
//!
//! Each filter type corresponds to a column data type and provides
//! comparison operators as optional fields.

use async_graphql::InputObject;

/// Filter for String columns.
#[derive(InputObject, Default, Clone, Debug)]
#[graphql(name = "StringFilter")]
pub struct StringFilter {
    /// Exact match.
    #[graphql(name = "_eq")]
    pub eq: Option<String>,
    /// Not equal.
    #[graphql(name = "_ne")]
    pub ne: Option<String>,
    /// Contains substring (case-sensitive).
    #[graphql(name = "_contains")]
    pub contains: Option<String>,
    /// Starts with prefix.
    #[graphql(name = "_startsWith")]
    pub starts_with: Option<String>,
    /// Ends with suffix.
    #[graphql(name = "_endsWith")]
    pub ends_with: Option<String>,
    /// Value is one of the given strings.
    #[graphql(name = "_in")]
    pub in_list: Option<Vec<String>>,
    /// Value is NOT one of the given strings.
    #[graphql(name = "_notIn")]
    pub not_in: Option<Vec<String>>,
    /// Whether the value is null.
    #[graphql(name = "_isNull")]
    pub is_null: Option<bool>,
}

/// Filter for i32 columns.
#[derive(InputObject, Default, Clone, Debug)]
#[graphql(name = "IntFilter")]
pub struct IntFilter {
    /// Exact match.
    #[graphql(name = "_eq")]
    pub eq: Option<i32>,
    /// Not equal.
    #[graphql(name = "_ne")]
    pub ne: Option<i32>,
    /// Greater than.
    #[graphql(name = "_gt")]
    pub gt: Option<i32>,
    /// Greater than or equal.
    #[graphql(name = "_gte")]
    pub gte: Option<i32>,
    /// Less than.
    #[graphql(name = "_lt")]
    pub lt: Option<i32>,
    /// Less than or equal.
    #[graphql(name = "_lte")]
    pub lte: Option<i32>,
    /// Value is one of the given integers.
    #[graphql(name = "_in")]
    pub in_list: Option<Vec<i32>>,
    /// Whether the value is null.
    #[graphql(name = "_isNull")]
    pub is_null: Option<bool>,
}

/// Filter for i64 columns.
#[derive(InputObject, Default, Clone, Debug)]
#[graphql(name = "Int64Filter")]
pub struct Int64Filter {
    /// Exact match.
    #[graphql(name = "_eq")]
    pub eq: Option<i64>,
    /// Not equal.
    #[graphql(name = "_ne")]
    pub ne: Option<i64>,
    /// Greater than.
    #[graphql(name = "_gt")]
    pub gt: Option<i64>,
    /// Greater than or equal.
    #[graphql(name = "_gte")]
    pub gte: Option<i64>,
    /// Less than.
    #[graphql(name = "_lt")]
    pub lt: Option<i64>,
    /// Less than or equal.
    #[graphql(name = "_lte")]
    pub lte: Option<i64>,
    /// Whether the value is null.
    #[graphql(name = "_isNull")]
    pub is_null: Option<bool>,
}

/// Filter for f64/f32 columns.
#[derive(InputObject, Default, Clone, Debug)]
#[graphql(name = "FloatFilter")]
pub struct FloatFilter {
    /// Exact match.
    #[graphql(name = "_eq")]
    pub eq: Option<f64>,
    /// Not equal.
    #[graphql(name = "_ne")]
    pub ne: Option<f64>,
    /// Greater than.
    #[graphql(name = "_gt")]
    pub gt: Option<f64>,
    /// Greater than or equal.
    #[graphql(name = "_gte")]
    pub gte: Option<f64>,
    /// Less than.
    #[graphql(name = "_lt")]
    pub lt: Option<f64>,
    /// Less than or equal.
    #[graphql(name = "_lte")]
    pub lte: Option<f64>,
    /// Whether the value is null.
    #[graphql(name = "_isNull")]
    pub is_null: Option<bool>,
}

/// Filter for boolean columns.
#[derive(InputObject, Default, Clone, Debug)]
#[graphql(name = "BoolFilter")]
pub struct BoolFilter {
    /// Exact match.
    #[graphql(name = "_eq")]
    pub eq: Option<bool>,
    /// Whether the value is null.
    #[graphql(name = "_isNull")]
    pub is_null: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_filter_default_is_empty() {
        let f = StringFilter::default();
        assert!(f.eq.is_none());
        assert!(f.ne.is_none());
        assert!(f.contains.is_none());
        assert!(f.starts_with.is_none());
        assert!(f.ends_with.is_none());
        assert!(f.in_list.is_none());
        assert!(f.not_in.is_none());
        assert!(f.is_null.is_none());
    }

    #[test]
    fn int_filter_default_is_empty() {
        let f = IntFilter::default();
        assert!(f.eq.is_none());
        assert!(f.gt.is_none());
        assert!(f.lte.is_none());
        assert!(f.in_list.is_none());
    }

    #[test]
    fn int64_filter_default_is_empty() {
        let f = Int64Filter::default();
        assert!(f.eq.is_none());
        assert!(f.gt.is_none());
    }

    #[test]
    fn float_filter_default_is_empty() {
        let f = FloatFilter::default();
        assert!(f.eq.is_none());
        assert!(f.gt.is_none());
    }

    #[test]
    fn bool_filter_default_is_empty() {
        let f = BoolFilter::default();
        assert!(f.eq.is_none());
        assert!(f.is_null.is_none());
    }

    #[test]
    fn string_filter_with_eq() {
        let f = StringFilter {
            eq: Some("hello".into()),
            ..Default::default()
        };
        assert_eq!(f.eq, Some("hello".into()));
    }

    #[test]
    fn string_filter_with_in_list() {
        let f = StringFilter {
            in_list: Some(vec!["a".into(), "b".into()]),
            ..Default::default()
        };
        assert_eq!(f.in_list.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn int_filter_with_range() {
        let f = IntFilter {
            gt: Some(10),
            lte: Some(100),
            ..Default::default()
        };
        assert_eq!(f.gt, Some(10));
        assert_eq!(f.lte, Some(100));
    }
}
