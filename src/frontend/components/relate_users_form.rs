use crate::entities::{Estado, Familia};

use crate::{
    entities::{Persona, Sexo},
    frontend::{
        lib::{log, request},
        structs::Auth,
    },
};
use async_std::task::block_on;
use chrono::{Local, Months};
use reqwest::Method;
use sycamore::prelude::*;

const NAME: &'static str = "Relate Users";

#[component(inline_props)]
pub fn RelateUsersForm(auth: Signal<Auth>, miembros: Signal<Option<Vec<Persona>>>) -> View {
    let i_hombres = create_signal(usize::MAX);
    let i_mujeres = create_signal(usize::MAX);
    let hombres = create_signal(miembros.get_clone().map(|m|{m.into_iter().filter(|p|match p.estado(){Estado::Miembro {..}|Estado::Diacono {..}|Estado::Presbitero {..}=>true,_=>false}&&p.sexo() == Sexo::Masculino&&p.nacimiento()<Local::now().date_naive().checked_sub_months(Months::new(12*18)).unwrap()).collect::<Vec<Persona>>()}));
    let opt = hombres.clone();
    let mujeres = create_signal(miembros.get_clone().map(|m|{m.into_iter().filter(|p|match p.estado(){Estado::Miembro {..}=>true,_=>false}&&p.sexo() == Sexo::Femenino&&p.nacimiento()<Local::now().date_naive().checked_sub_months(Months::new(12*18)).unwrap()).collect::<Vec<Persona>>()}));
    let padre_actual = create_signal(None);
    let madre_actual = create_signal(None);
    let familia_actual = create_signal(None);
    let padre_id = create_signal(String::new());
    let madre_id = create_signal(String::new());
    create_effect(move || log(NAME,29,&madre_id.get_clone()));
    let options = match &hombres.get_clone() {
        Some(h) => h
            .into_iter()
            .cloned()
            .map(|p| {
                let id = format!("{}", p.id().unwrap());
                view! {
                    option(value=id){(p.nombre().to_string())}
                }
            })
            .collect::<Vec<View>>(),
        None => return view! {},
    };
    let options_mujeres = match &mujeres.get_clone() {
        Some(h) => h
            .into_iter()
            .cloned()
            .map(|p| {
                let id = p.id().unwrap().clone();
                let id2 = id.clone();
                view! {
                    option(value=id.clone(),selected=id2.eq(&madre_id.get_clone())){(p.nombre().to_string())}
                }
            })
            .collect::<Vec<View>>(),
        None => return view! {},
    };
    let hs = hombres.clone();
    create_effect(move ||{
        // padre_id.track();
        if let Some(hombres) = hs.get_clone(){
            let padre_id = padre_id.get_clone();
            let padre = hombres.into_iter().enumerate().find_map(move |(i,p)|padre_id.eq(p.id().unwrap()).then_some((i,p))).clone();
            padre_actual.set(padre.as_ref().map(|(_,p)|p.clone()));
            i_hombres.set(padre.map(|(i,_)|i).unwrap_or(usize::MAX));
        }
    });
    let mjs = mujeres.clone();
    create_effect(move ||{
        if let Some(mujeres) = mjs.get_clone(){
            let madre_id = madre_id.get_clone();
            let madre = mujeres.into_iter().enumerate().find_map(move |(i,m)|madre_id.eq(m.id().unwrap()).then_some((i,m))).clone();
            madre_actual.set(madre.as_ref().map(|(_,m)|m.clone()));
            i_mujeres.set(madre.map(|(i,_)|i).unwrap_or(usize::MAX));
        }
    });
    let select_father = move |_| {
        block_on(async move {
            log(NAME,30,&padre_id);
            let familia = match request::<Familia>(format!("api/v1/families/{}",padre_id.get_clone()),auth.clone(),Method::GET,None::<bool>,true).await{
                Ok(f)=>f,
                Err(_)=>{
                    let mut f = Familia::default();
                    let id = padre_id.get_clone();
                    let padre = hs.get_clone().unwrap().iter().find(move |p|{id.eq(p.id().unwrap())}).cloned();
                    f.set_padre(padre.clone());
                    log(NAME,90,&f);
                    Some(f)
                }
            };
            log(NAME,31,&familia);
            let madre = familia.as_ref().map(|f|{f.madre().as_ref().map(|m|m.nombre().to_string())}).flatten().unwrap_or_default();
            log(NAME,85,&madre);
            madre_id.set(madre);
            familia_actual.set(familia);
        })};
    let select_mother = move|_|{
        block_on(async move {
            let mut familia = familia_actual.get_clone().unwrap_or_default();
            match familia.padre(){
                Some(_) => {
                    familia.set_madre(mjs.get_clone().unwrap().iter().find(move |m|madre_id.get_clone().eq(m.id().unwrap())).cloned());
                }
                None => {
                    familia = match request::<Familia>(format!("api/v1/families/{}",padre_id.get_clone()),auth.clone(),Method::GET,None::<bool>,true).await{
                        Ok(f)=>f.unwrap_or_default(),
                        Err(_)=>{
                            let mut f = Familia::default();
                            let id = padre_id.get_clone();
                            let madre = mjs.get_clone().unwrap().iter().find(move |m|{id.eq(m.id().unwrap())}).cloned();
                            f.set_madre(madre.clone());
                            log(NAME,103,&f);
                            f
                        }
                    };
                }
            }
            familia_actual.set(Some(familia));
        })
    };
    log(NAME, 18, &options);
    create_memo(move || log(NAME, 21, &opt));
    // let options = hombres.map(|p|{p.iter().map(|p|view!{option(value=(p.id())){(p.nombre())}}).collect::<Vec<View>>()});
    view! {
        section(id="padres"){
            article(){
                label(r#for="select_hombre"){"Padre: "}
                select(name="select_hombre", on:change = select_father, bind:value=padre_id){
                    option(value=""){"-"}
                    (options)
                }
            }
            article(){
                label(r#for="select_mujer"){"Madre: "}
                select(name="select_mujer",on:change=select_mother,bind:value=madre_id){
                    option(value=""){"-"}
                    (options_mujeres)
                }
            }
        }
    }
}