//! ApplyFilter trait for converting filter types to SeaORM conditions.
//!
//! Each filter type implements ApplyFilter to generate a SeaORM `Condition`
//! for a given column.

use sea_orm::{ColumnTrait, Condition};

use super::types::{BoolFilter, FloatFilter, Int64Filter, IntFilter, StringFilter};

/// Trait for applying a filter to a SeaORM column.
pub trait ApplyFilter<C: ColumnTrait> {
    /// Convert this filter into a SeaORM condition for the given column.
    fn apply(&self, column: C) -> Condition;
}

impl<C: ColumnTrait> ApplyFilter<C> for StringFilter {
    fn apply(&self, column: C) -> Condition {
        let mut cond = Condition::all();
        if let Some(ref v) = self.eq {
            cond = cond.add(column.eq(v.as_str()));
        }
        if let Some(ref v) = self.ne {
            cond = cond.add(column.ne(v.as_str()));
        }
        if let Some(ref v) = self.contains {
            cond = cond.add(column.contains(v.as_str()));
        }
        if let Some(ref v) = self.starts_with {
            cond = cond.add(column.starts_with(v.as_str()));
        }
        if let Some(ref v) = self.ends_with {
            cond = cond.add(column.ends_with(v.as_str()));
        }
        if let Some(ref v) = self.in_list {
            let values: Vec<&str> = v.iter().map(|s| s.as_str()).collect();
            cond = cond.add(column.is_in(values));
        }
        if let Some(ref v) = self.not_in {
            let values: Vec<&str> = v.iter().map(|s| s.as_str()).collect();
            cond = cond.add(column.is_not_in(values));
        }
        if let Some(is_null) = self.is_null {
            if is_null {
                cond = cond.add(column.is_null());
            } else {
                cond = cond.add(column.is_not_null());
            }
        }
        cond
    }
}

impl<C: ColumnTrait> ApplyFilter<C> for IntFilter {
    fn apply(&self, column: C) -> Condition {
        let mut cond = Condition::all();
        if let Some(v) = self.eq {
            cond = cond.add(column.eq(v));
        }
        if let Some(v) = self.ne {
            cond = cond.add(column.ne(v));
        }
        if let Some(v) = self.gt {
            cond = cond.add(column.gt(v));
        }
        if let Some(v) = self.gte {
            cond = cond.add(column.gte(v));
        }
        if let Some(v) = self.lt {
            cond = cond.add(column.lt(v));
        }
        if let Some(v) = self.lte {
            cond = cond.add(column.lte(v));
        }
        if let Some(ref v) = self.in_list {
            cond = cond.add(column.is_in(v.clone()));
        }
        if let Some(is_null) = self.is_null {
            if is_null {
                cond = cond.add(column.is_null());
            } else {
                cond = cond.add(column.is_not_null());
            }
        }
        cond
    }
}

impl<C: ColumnTrait> ApplyFilter<C> for Int64Filter {
    fn apply(&self, column: C) -> Condition {
        let mut cond = Condition::all();
        if let Some(v) = self.eq {
            cond = cond.add(column.eq(v));
        }
        if let Some(v) = self.ne {
            cond = cond.add(column.ne(v));
        }
        if let Some(v) = self.gt {
            cond = cond.add(column.gt(v));
        }
        if let Some(v) = self.gte {
            cond = cond.add(column.gte(v));
        }
        if let Some(v) = self.lt {
            cond = cond.add(column.lt(v));
        }
        if let Some(v) = self.lte {
            cond = cond.add(column.lte(v));
        }
        if let Some(is_null) = self.is_null {
            if is_null {
                cond = cond.add(column.is_null());
            } else {
                cond = cond.add(column.is_not_null());
            }
        }
        cond
    }
}

impl<C: ColumnTrait> ApplyFilter<C> for FloatFilter {
    fn apply(&self, column: C) -> Condition {
        let mut cond = Condition::all();
        if let Some(v) = self.eq {
            cond = cond.add(column.eq(v));
        }
        if let Some(v) = self.ne {
            cond = cond.add(column.ne(v));
        }
        if let Some(v) = self.gt {
            cond = cond.add(column.gt(v));
        }
        if let Some(v) = self.gte {
            cond = cond.add(column.gte(v));
        }
        if let Some(v) = self.lt {
            cond = cond.add(column.lt(v));
        }
        if let Some(v) = self.lte {
            cond = cond.add(column.lte(v));
        }
        if let Some(is_null) = self.is_null {
            if is_null {
                cond = cond.add(column.is_null());
            } else {
                cond = cond.add(column.is_not_null());
            }
        }
        cond
    }
}

impl<C: ColumnTrait> ApplyFilter<C> for BoolFilter {
    fn apply(&self, column: C) -> Condition {
        let mut cond = Condition::all();
        if let Some(v) = self.eq {
            cond = cond.add(column.eq(v));
        }
        if let Some(is_null) = self.is_null {
            if is_null {
                cond = cond.add(column.is_null());
            } else {
                cond = cond.add(column.is_not_null());
            }
        }
        cond
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::types::*;

    // We can't test the actual SQL generation without a database,
    // but we can test that filters convert without panicking.

    // Use messages::Column as a concrete test column type
    use han_db::entities::messages;

    #[test]
    fn string_filter_eq_produces_condition() {
        let f = StringFilter {
            eq: Some("user".into()),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::MessageType);
    }

    #[test]
    fn string_filter_ne_produces_condition() {
        let f = StringFilter {
            ne: Some("system".into()),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::MessageType);
    }

    #[test]
    fn string_filter_contains_produces_condition() {
        let f = StringFilter {
            contains: Some("search".into()),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::Content);
    }

    #[test]
    fn string_filter_starts_with_produces_condition() {
        let f = StringFilter {
            starts_with: Some("prefix".into()),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::Content);
    }

    #[test]
    fn string_filter_ends_with_produces_condition() {
        let f = StringFilter {
            ends_with: Some("suffix".into()),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::Content);
    }

    #[test]
    fn string_filter_in_list_produces_condition() {
        let f = StringFilter {
            in_list: Some(vec!["a".into(), "b".into(), "c".into()]),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::MessageType);
    }

    #[test]
    fn string_filter_not_in_produces_condition() {
        let f = StringFilter {
            not_in: Some(vec!["x".into()]),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::MessageType);
    }

    #[test]
    fn string_filter_is_null_true_produces_condition() {
        let f = StringFilter {
            is_null: Some(true),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::ToolName);
    }

    #[test]
    fn string_filter_is_null_false_produces_condition() {
        let f = StringFilter {
            is_null: Some(false),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::ToolName);
    }

    #[test]
    fn string_filter_combined_operators() {
        let f = StringFilter {
            ne: Some("system".into()),
            contains: Some("text".into()),
            is_null: Some(false),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::Content);
    }

    #[test]
    fn int_filter_eq_produces_condition() {
        let f = IntFilter {
            eq: Some(42),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::LineNumber);
    }

    #[test]
    fn int_filter_range_produces_condition() {
        let f = IntFilter {
            gt: Some(10),
            lte: Some(100),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::LineNumber);
    }

    #[test]
    fn int_filter_in_list_produces_condition() {
        let f = IntFilter {
            in_list: Some(vec![1, 2, 3]),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::LineNumber);
    }

    #[test]
    fn float_filter_range_produces_condition() {
        let f = FloatFilter {
            gt: Some(0.5),
            lte: Some(1.0),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::SentimentScore);
    }

    #[test]
    fn bool_filter_eq_produces_condition() {
        let f = BoolFilter {
            eq: Some(true),
            ..Default::default()
        };
        // Use a column that can accept bool - bool filters are for
        // columns that store boolean values
        let _cond = f.apply(messages::Column::Content);
    }

    #[test]
    fn empty_filter_produces_empty_condition() {
        let f = StringFilter::default();
        let _cond = f.apply(messages::Column::MessageType);
        // Should not panic with all-None fields
    }

    #[test]
    fn int64_filter_produces_condition() {
        let f = Int64Filter {
            gt: Some(1000),
            lte: Some(99999),
            ..Default::default()
        };
        let _cond = f.apply(messages::Column::LineNumber);
    }
}
