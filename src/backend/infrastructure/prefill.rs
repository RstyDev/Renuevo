use crate::backend::application::use_cases::users::register_user::RegisterUserUseCase;
use crate::backend::infrastructure::repositories::surreal_user_repository::SurrealUserRepository;
use crate::entities::{
    Bautismo, Estado, EstadoCivil, Ministerio, Persona, Servicio, Sexo, TipoPresbitero,
};
use chrono::Local;
use std::sync::Arc;

pub async fn prefill(repo: Arc<SurrealUserRepository>) {
    let mut personas = vec![];
    personas.push(Persona::new(
        None,
        Some(String::from("121212")),
        String::from("Lucas"),
        String::from("Igarzabal"),
        Sexo::Masculino,
        Local::now().date_naive(),
        EstadoCivil::Casado,
        Estado::Miembro {
            conversion: Local::now().date_naive(),
            servicio: vec![
                Servicio::new(true, Ministerio::Tesoro),
                Servicio::new(false, Ministerio::Sonido),
            ],
            bautismo: Bautismo::new(
                Local::now().date_naive(),
                Some(Local::now().date_naive()),
                String::from("Vida Sobrenatural"),
            ),
        },
        Local::now().naive_local().date(),
    ));
    personas.push(Persona::new(
        None,
        Some(String::from("232323")),
        String::from("Rafael"),
        String::from("De Lima"),
        Sexo::Masculino,
        Local::now().date_naive(),
        EstadoCivil::Casado,
        Estado::Presbitero {
            conversion: Local::now().date_naive(),
            servicio: vec![
                Servicio::new(true, Ministerio::Palabra),
                Servicio::new(false, Ministerio::Presbiterado),
            ],
            bautismo: Bautismo::new(
                Local::now().date_naive(),
                Some(Local::now().date_naive()),
                String::from("Pentecostal de Misiones"),
            ),
            tipo: TipoPresbitero::Maestro,
        },
        Local::now().naive_local().date(),
    ));
    personas.push(Persona::new(
        None,
        Some(String::from("w98g7sd8")),
        String::from("María José"),
        String::from("Cortés Alarcón"),
        Sexo::Masculino,
        Local::now().date_naive(),
        EstadoCivil::Casado,
        Estado::Miembro {
            conversion: Local::now().date_naive(),
            servicio: vec![Servicio::new(false, Ministerio::Bienvenida)],
            bautismo: Bautismo::new(
                Local::now().date_naive(),
                Some(Local::now().date_naive()),
                String::from("Iglesia Reformada Renuevo"),
            ),
        },
        Local::now().naive_local().date(),
    ));
    personas.push(Persona::new(
        None,
        Some(String::from("9r7fg8g76y")),
        String::from("Jordi"),
        String::from("Fajardo"),
        Sexo::Masculino,
        Local::now().date_naive(),
        EstadoCivil::Soltero,
        Estado::PreMiembro {
            conversion: Local::now().date_naive(),
            bautismo: None,
        },
        Local::now().naive_local().date(),
    ));
    personas.push(Persona::new(
        None,
        Some(String::from("9d78f6sd")),
        String::from("Matias"),
        String::from("Díaz"),
        Sexo::Masculino,
        Local::now().date_naive(),
        EstadoCivil::Soltero,
        Estado::Nuevo,
        Local::now().naive_local().date(),
    ));
    let use_case = RegisterUserUseCase::new(repo);

    for persona in personas {
        use_case.execute(persona).await.unwrap();
    }
}
