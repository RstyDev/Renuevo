use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct IglesiaDB{
    pub id: Option<Thing>,
    pub nombre: String,
    pub denominacion: String,
    pub presbiterio: String
}

impl IglesiaDB {
    pub fn new(id: Option<String>, nombre: String, denominacion: String, presbiterio: String) -> Self {
        Self { id: id.map(|id|Thing::from(("iglesia",id.as_str()))), nombre, denominacion, presbiterio }
    }
}