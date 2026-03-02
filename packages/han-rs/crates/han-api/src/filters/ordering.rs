//! Order direction enum for GraphQL ordering.

use async_graphql::Enum;

/// Sort direction for ordering query results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum OrderDirection {
    /// Ascending order (A-Z, 0-9, oldest first).
    #[graphql(name = "ASC")]
    Asc,
    /// Descending order (Z-A, 9-0, newest first).
    #[graphql(name = "DESC")]
    Desc,
}

impl From<OrderDirection> for sea_orm::Order {
    fn from(dir: OrderDirection) -> Self {
        match dir {
            OrderDirection::Asc => sea_orm::Order::Asc,
            OrderDirection::Desc => sea_orm::Order::Desc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_direction_converts_to_sea_orm() {
        assert_eq!(sea_orm::Order::Asc, OrderDirection::Asc.into());
        assert_eq!(sea_orm::Order::Desc, OrderDirection::Desc.into());
    }
}
