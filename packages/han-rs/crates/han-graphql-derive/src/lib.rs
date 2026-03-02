//! Proc macros for auto-generating GraphQL types from SeaORM entity definitions.
//!
//! Two derive macros:
//! - `GraphQLEntity`: Full type generation (SimpleObject + From + Filter + OrderBy)
//! - `EntityFilter`: Filter + OrderBy generation only (for hand-written types)

mod entity_filter;
mod graphql_entity;

use proc_macro::TokenStream;

/// Derive macro that generates Filter and OrderBy input types for an entity.
///
/// Applied to a "source" struct that mirrors the entity's filterable columns.
/// Generates `{Name}Filter` and `{Name}OrderBy` InputObject types plus
/// `to_condition()` and `apply()` methods.
///
/// # Attributes
///
/// ## Container
///
/// - `#[entity_filter(entity = "...", columns = "...")]` — required, specifies
///   the SeaORM Entity and Column paths.
///
/// ## Fields
///
/// - `#[entity_filter(skip)]` — exclude from filter/order generation.
/// - `#[entity_filter(assoc(...))]` — declare an association (relationship)
///   filter. The field type must be `()` (marker). Generates a nested filter
///   field and `IN (SELECT ...)` subquery condition.
///
///   Required attributes inside `assoc(...)`:
///   - `filter = "path::to::ForeignFilter"` — the filter type for the related entity
///   - `local_column = "ColumnVariant"` — column on *this* entity used for the join
///   - `foreign_entity = "path::to::Entity"` — SeaORM entity for the related table
///   - `foreign_column = "path::to::Column::Variant"` — column on the related entity to select
///
/// # Example
///
/// ```ignore
/// #[derive(EntityFilter)]
/// #[entity_filter(entity = "sessions::Entity", columns = "sessions::Column")]
/// pub struct SessionFilterSource {
///     pub id: String,
///     pub project_id: Option<String>,
///     pub status: Option<String>,
///
///     /// Association: sessions.project_id IN (SELECT id FROM projects WHERE ...)
///     #[entity_filter(assoc(
///         filter = "crate::types::project::ProjectFilter",
///         local_column = "ProjectId",
///         foreign_entity = "han_db::entities::projects::Entity",
///         foreign_column = "han_db::entities::projects::Column::Id",
///     ))]
///     pub project: (),
/// }
/// ```
#[proc_macro_derive(EntityFilter, attributes(entity_filter))]
pub fn derive_entity_filter(input: TokenStream) -> TokenStream {
    entity_filter::derive(input)
}

/// Derive macro that generates a full GraphQL type from an entity mirror struct.
///
/// Generates:
/// - `SimpleObject` impl with `#[graphql(complex)]`
/// - `From<Model>` impl
/// - Filter + OrderBy types (same as `EntityFilter`)
/// - Optional `ComplexObject` impl with `id()` resolver
///
/// # Attributes
///
/// - `#[graphql_entity(model = "...", entity = "...", columns = "...", type_name = "...")]`
/// - `#[graphql_entity(skip)]` — exclude field from SimpleObject
/// - `#[graphql_entity(id_field, encode = "TypeName")]` — generate id() resolver
/// - `#[graphql_entity(transform = "fn_name", source_type = "Type")]` — apply transform in From
/// - `#[graphql_entity(source_field = "other")]` — map from different Model field
///
/// # Example
///
/// ```ignore
/// #[derive(GraphQLEntity)]
/// #[graphql_entity(
///     model = "projects::Model",
///     entity = "projects::Entity",
///     columns = "projects::Column",
///     type_name = "Project",
/// )]
/// pub struct Project {
///     #[graphql_entity(skip, id_field, encode = "Project")]
///     pub raw_id: String,
///     pub slug: String,
///     pub name: String,
///     #[graphql_entity(transform = "nonzero_to_bool", source_type = "Option<i32>")]
///     pub is_worktree: bool,
/// }
/// ```
#[proc_macro_derive(GraphQLEntity, attributes(graphql_entity))]
pub fn derive_graphql_entity(input: TokenStream) -> TokenStream {
    graphql_entity::derive(input)
}
