//! Base for Backend Model Controllers.
//! Essentially a generic CRUD

use sea_query::SeaRc;

use crate::{context::Context, model::ModelManager};


/// Shared column identifiers.
/// These columns are guaranteed to exist on every table this generic CRUD works with.
#[derive(sea_query::Iden)]
pub enum SharedColums {
    /// All tables must have the identifier column 'Id'.
    Id,
}


/// This Table Identifier is the intentional trust boundary that the table name actually exists.
struct TableIdentifier(&'static str);

impl sea_query::Iden for TableIdentifier {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        s.write_str(self.0).unwrap()
    }
}


/// All backend model controllers that are database related must implement this trait.
/// This trait knows the database table.
pub trait DatabaseBmc {
    /// Name of the database table.
    const TABLE: &'static str;

    /// Reference to the SeaQuery Table.
    fn table_ref() -> sea_query::TableRef {
        sea_query::TableRef::Table(SeaRc::new(TableIdentifier(Self::TABLE)))
    }
}


pub async fn create<MC, E>(_ctx: &Context, mm: &ModelManager, data: E) -> Result<i64>
where
    MC: DatabaseBmc,
    E: modql::HasSeaFields, // knows
{
    let db = mm.db();
}
