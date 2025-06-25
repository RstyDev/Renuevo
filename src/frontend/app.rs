use async_std::task::block_on;
use crate::error::AppError;
use reqwest::StatusCode;
use reqwest::Method;
use crate::{entities::LoginResult ,frontend::{
    components::header::Header,
    pages::main_page::MainPage,
    structs::{Auth, Tabs},
    lib::{request, HOST, log, rfc_7231},
}};
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use crate::entities::RefreshResult;
use chrono::{prelude::*, Days, Months};

#[component]
pub fn App() -> View {

    log("App",19,&rfc_7231(Utc::now()));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();

    let logged = create_signal(Auth::NotLogged);
    let error_message = create_signal(String::new());
    let tab = create_signal(Tabs::Inicio);
    let persona = create_signal(None);
    // create_memo(move ||{
    //     console_log!("Tab: {:#?}",tab.get_clone())
    // });
    let cookie = html_document.cookie().unwrap();
    create_memo(move || match logged.get_clone() {
        Auth::NotLogged => {
            html_document.set_cookie(&format!("token={}","")).unwrap();
            html_document.set_cookie(&format!("refresh={}","")).unwrap();
            tab.set(Tabs::Inicio);
            persona.set(None);
        }
        Auth::Logged(login) => {
            html_document.set_cookie(&format!("token={}; expires={}; path=/",&login.token,rfc_7231(Utc::now().checked_add_days(Days::new(1)).unwrap()))).unwrap();
            html_document.set_cookie(&format!("refresh={}; expires={}; path=/",&login.refresh,rfc_7231(Utc::now().checked_add_days(Days::new(1)).unwrap()))).unwrap();
        }
    });
    log("App",39,&cookie);
    let logged2 = logged.clone();
    block_on(async move {
        match cookie.split("refresh=").nth(1) {
            Some(first_part) => {
                let token = first_part.split(";").nth(0).unwrap();
                if token.len()>10 {
                    log("App",43,&token);
                    let client = reqwest::Client::builder().build().unwrap();
                    let res = client
                        .request(Method::POST, &format!("{}/refresh_token", HOST.as_str()))
                        .header("Authorization", format!("Bearer {}", token)).send().await;
                    let res = match res {
                        Ok(r) => match r.status() {
                            StatusCode::OK => r
                                .json::<RefreshResult>()
                                .await
                                .map_err(|e| AppError::HttpErr(13, e.to_string())),
                            _ => Err(AppError::HttpErr(19, r.json::<String>().await.unwrap())),
                        },
                        Err(e) => Err(AppError::HttpErr(13, e.to_string())),
                    };
                    if let Ok(refresh_result) = res {
                        logged2.set(
                            Auth::Logged(LoginResult {
                                id: refresh_result.id,
                                token: refresh_result.token,
                                refresh: token.to_string(),
                            })
                        );
                    }
                }
            },
            None => (),
        }

    });



    view! {
        article(id="main"){
            Header(auth = logged.clone(),tabs = tab.clone(), hermano = persona.clone())
            MainPage(auth = logged.clone(), tab = tab, resource = persona, error_message = error_message)
        }
    }
}
