use crate::backend::infrastructure::db::models::users::PersonaDB;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct FamiliaDB {
    id: Option<Thing>,
    apellido: String,
    padre: Option<PersonaDB>,
    madre: Option<PersonaDB>,
    hijos: Vec<PersonaDB>,
}
impl FamiliaDB {
    pub fn new(
        id: Option<String>,
        apellido: String,
        padre: Option<PersonaDB>,
        madre: Option<PersonaDB>,
        hijos: Vec<PersonaDB>,
    ) -> Self {
        Self {
            id: id.map(|s| Thing::from(("familias", s.as_str()))),
            apellido,
            padre,
            madre,
            hijos,
        }
    }
    pub fn id(&self) -> &Option<Thing> {
        &self.id
    }

    pub fn apellido(&self) -> &str {
        &self.apellido
    }

    pub fn padre(&self) -> &Option<PersonaDB> {
        &self.padre
    }

    pub fn madre(&self) -> &Option<PersonaDB> {
        &self.madre
    }

    pub fn hijos(&self) -> &Vec<PersonaDB> {
        &self.hijos
    }
}
