use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Bautismo {
    fecha: NaiveDate,
    profesion_de_fe: Option<NaiveDate>,
    iglesia: String,
}

impl Bautismo {
    pub fn new(fecha: NaiveDate, profesion_de_fe: Option<NaiveDate>, iglesia: String) -> Self {
        Self {
            fecha,
            profesion_de_fe,
            iglesia,
        }
    }

    pub fn fecha(&self) -> NaiveDate {
        self.fecha
    }

    pub fn profesion_de_fe(&self) -> Option<NaiveDate> {
        self.profesion_de_fe
    }

    pub fn iglesia(&self) -> &str {
        &self.iglesia
    }

    pub fn set_profesion_de_fe(&mut self, profesion_de_fe: Option<NaiveDate>) {
        self.profesion_de_fe = profesion_de_fe;
    }
}
