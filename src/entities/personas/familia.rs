#[cfg(feature = "ssr")]
use crate::backend::infrastructure::db::models::families::FamiliaDB;
use crate::backend::infrastructure::db::models::users::PersonaDB;
use crate::entities::Persona;
use crate::error::{AppError, AppRes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Familia {
    id: Option<String>,
    apellido: String,
    padre: Option<Persona>,
    madre: Option<Persona>,
    hijos: Vec<Persona>,
}

impl Familia {
    pub fn new(
        id: Option<String>,
        apellido: String,
        padre: Option<Persona>,
        madre: Option<Persona>,
        hijos: Vec<Persona>,
    ) -> Self {
        Self {
            id,
            apellido,
            padre,
            madre,
            hijos,
        }
    }

    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    pub fn apellido(&self) -> &str {
        &self.apellido
    }

    pub fn padre(&self) -> &Option<Persona> {
        &self.padre
    }

    pub fn madre(&self) -> &Option<Persona> {
        &self.madre
    }

    pub fn hijos(&self) -> &Vec<Persona> {
        &self.hijos
    }

    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    pub fn set_apellido(&mut self, apellido: String) {
        self.apellido = apellido;
    }

    pub fn set_madre(&mut self, madre: Option<Persona>) {
        self.madre = madre;
    }

    pub fn set_hijos(&mut self, hijos: Vec<Persona>) {
        self.hijos = hijos;
    }

    pub fn add_hijo(&mut self, hijo: Persona) {
        self.hijos.push(hijo);
    }

    pub fn remove_hijo(&mut self, id: String) -> AppRes<()> {
        let res = self
            .hijos
            .iter()
            .enumerate()
            .find_map(|(i, p)| p.id().map(|this_id| this_id.eq(&id).then_some(i)).flatten());
        match res {
            None => Err(AppError::NotFound(60)),
            Some(i) => {
                self.hijos.remove(i);
                Ok(())
            }
        }
    }
    #[cfg(feature = "ssr")]
    pub fn to_db(self) -> AppRes<FamiliaDB> {
        let mut hijos = vec![];
        for h in self.hijos {
            hijos.push(h.to_db()?);
        }
        Ok(FamiliaDB::new(
            self.id,
            self.apellido,
            match self.padre {
                None => None,
                Some(a) => Some(a.to_db()?),
            },
            match self.madre {
                None => None,
                Some(a) => Some(a.to_db()?),
            },
            hijos,
        ))
    }
    #[cfg(feature = "ssr")]
    pub fn from_db(
        familia: FamiliaDB,
    ) -> Self {
        Self {
            id: familia.id().as_ref().map(|id| id.id.to_string()),
            apellido: familia.apellido().to_string(),
            padre: familia.padre().as_ref().map(|p| Persona::from_db(p.to_owned())),
            madre: familia.madre().as_ref().map(|m| Persona::from_db(m.to_owned())),
            hijos: familia.hijos().into_iter().map(|h| Persona::from_db(h.to_owned())).collect(),
        }
    }
    // #[cfg(feature = "ssr")]
    // pub fn from_db_complete(
    //     familia: FamiliaDB,
    //     padre: Option<PersonaDB>,
    //     madre: Option<PersonaDB>,
    //     hijos: Vec<PersonaDB>,
    // ) -> Self {
    //     Self {
    //         id: familia.id().as_ref().map(|id| id.id.to_string()),
    //         apellido: familia.apellido().to_string(),
    //         padre: padre.map(|p| Persona::from_db(p)),
    //         madre: madre.map(|m| Persona::from_db(m)),
    //         hijos: hijos.into_iter().map(|h| Persona::from_db(h)).collect(),
    //     }
    // }
}
