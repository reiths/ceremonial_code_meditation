/// For all backend model controllers that are database related.
pub trait BackendModelController {
    /// Name of the database table.
    const TABLE: &'static str;

    /* WIP
    /// Reference to the SeaQuery Table.
    fn table_ref() -> sea_query::TableRef {
        sea_query::TableRef::Table(Self::TABLE.into())
    }
    */
}
