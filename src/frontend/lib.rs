use crate::entities::{LoginResult, RefreshResult};
use crate::error::{AppError, AppRes};
use crate::frontend::structs::Auth;
use reqwest::{Body, Method, StatusCode};
use serde::de::{Deserialize, DeserializeOwned};
use std::fmt::Debug;
use sycamore::prelude::Signal;
// use lazy_static::lazy_static;
use serde::Serialize;
use std::sync::LazyLock;
use sycamore::prelude::*;

pub static HOST: LazyLock<String> = LazyLock::new(|| std::env!("BACKEND").to_string());

//const HOST: &str = "http://localhost:8088/";

async fn fetch<T: DeserializeOwned>(
    url: &str,
    token: String,
    method: Method,
    body: Option<impl Serialize + ?Sized + Clone>,
) -> AppRes<Option<T>> {
    let client = reqwest::Client::builder().build().unwrap();
    let req = client
        .request(method.clone(), url)
        .header("Authorization", format!("Bearer {}", token));
    let res = match body {
        None => req.send().await,
        Some(body) => req.json(&body).send().await,
    };
    match res {
        Ok(r) => match r.status() {
            StatusCode::OK => r
                .json::<T>()
                .await
                .map_err(|e| AppError::HttpErr(13, e.to_string()))
                .map(|t| Some(t)),
            StatusCode::NO_CONTENT => Ok(None),
            _ => Err(AppError::HttpErr(19, r.json::<String>().await.unwrap())),
        },
        Err(e) => Err(AppError::HttpErr(13, e.to_string())),
    }
}

pub async fn request<T: DeserializeOwned>(
    url: impl AsRef<str>,
    login: Signal<Auth>,
    method: Method,
    body: Option<impl Serialize + ?Sized + Clone>,
) -> AppRes<Option<T>> {
    match login.get_clone_untracked() {
        Auth::NotLogged => Err(AppError::HttpErr(21, String::from("Not logged in."))),
        Auth::Logged(_) => {
            let auth = login.get_clone_untracked().unwrap().clone();
            match fetch::<T>(
                &format!("{}/{}", HOST.as_str(), url.as_ref()),
                auth.token.clone(),
                method.clone(),
                body.clone(),
            )
            .await
            {
                Ok(res) => Ok(res),
                Err(_) => {
                    match fetch::<RefreshResult>(
                        &format!("{}/refresh_token", HOST.as_str()),
                        auth.refresh.clone(),
                        Method::POST,
                        None::<bool>,
                    )
                    .await
                    {
                        Ok(refresh) => {
                            log("LIB", 77, &refresh);
                            login.set_fn(|result| {
                                let result = result.unwrap();
                                Auth::Logged(LoginResult {
                                    token: refresh.as_ref().unwrap().token.clone(),
                                    ..result.clone()
                                })
                            });
                            fetch::<T>(url.as_ref(), refresh.unwrap().token, method, body)
                                .await
                                .map_err(|e| e.into())
                        }
                        Err(e) => Err(e),
                    }
                }
            }
        }
    }
}

pub fn log<T: Debug>(file: &str, pos: u16, data: &T) {
    console_log!("{} {}:\n{:?}", file, pos, data);
}
