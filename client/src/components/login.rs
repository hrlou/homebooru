use reqwasm::http::{Request, Response};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
// use yew::html::InputData;
use yew::prelude::*;
use yew_router::utils::fetch_base_url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub login: String,
    pub password: String,
}

// #[derive(Debug, Serialize)]
// pub struct Lo

pub enum Msg {
    Login,
    Logout,
    PasswordUpdate(String),
    LoginUpdate(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            login: String::new(),
            password: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login => {
                let body = serde_json::ser::to_string(&self).unwrap();
                spawn_local(async move {
                    Request::post("http://localhost:8080/api/auth")
                        .body(body)
                        .header("Content-Type", "application/json")
                        .send()
                        .await
                        .unwrap();
                });
                true
            }
            Msg::Logout => {
                spawn_local(async move {
                    Request::delete("http://localhost:8080/api/auth")
                        .send()
                        .await
                        .unwrap();
                });
                true
            }
            Msg::PasswordUpdate(s) => {
                self.password = s;
                true
            }
            Msg::LoginUpdate(s) => {
                self.login = s;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        html! {
            <div class="login">
            <h1>{ "Login" }</h1>
            <p>{ "Please enter your login and password" }</p>
            // <input class="field" type="password" placeholder="Password" id="password"/>
            <input class="field"
                type="text"
                placeholder="login"
                id="login"
                value={self.login.clone()}
                oninput={link.callback(|e: InputEvent| {
                    let s: String = e.target_unchecked_into::<HtmlInputElement>().value();
                    Msg::LoginUpdate(s)
                })}
            />
            <input class="field" type="password" placeholder="Password" id="password"
                value={self.password.clone()}
                oninput={link.callback(|e: InputEvent| {
                    let s: String = e.target_unchecked_into::<HtmlInputElement>().value();
                    Msg::PasswordUpdate(s)
                })}
            />
            <button class="btn" onclick={link.callback(|_| Msg::Login)}>{ "Sign in" }</button>
            <button class="btn" onclick={link.callback(|_| Msg::Logout)}>{ "Sign Out" }</button>
            </div>
        }
    }
}
