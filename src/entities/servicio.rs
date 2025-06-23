use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Ministerio {
    Sonido,
    Tesoro,
    Letras,
    Bienvenida,
    Redes,
    Alabanza,
    Misericordia,
    Palabra,
    Ancianato,
}

impl ToString for Ministerio {
    fn to_string(&self) -> String {
        match self {
            Ministerio::Sonido => String::from("Sonido"),
            Ministerio::Tesoro => String::from("Tesoro"),
            Ministerio::Letras => String::from("Letras"),
            Ministerio::Bienvenida => String::from("Bienvenida"),
            Ministerio::Redes => String::from("Redes"),
            Ministerio::Alabanza => String::from("Alabanza"),
            Ministerio::Misericordia => String::from("Misericordia"),
            Ministerio::Palabra => String::from("Palabra"),
            Ministerio::Ancianato => String::from("Ancianato"),
        }
    }
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Servicio {
    leader: bool,
    area: Ministerio,
}

impl Servicio {
    pub fn new(leader: bool, area: Ministerio) -> Self {
        Self { leader, area }
    }
    pub fn area(&self) -> &Ministerio {
        &self.area
    }
}
