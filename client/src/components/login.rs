use web_sys::HtmlInputElement;
// use yew::html::InputData;
use yew::prelude::*;
use yew_router::utils::fetch_base_url;

pub struct Model {
    pub value: i64,
    pub password: String,
    pub email: String,
}

pub enum Msg {
    Login,
    PasswordUpdate(String),
    EmailUpdate(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            password: String::new(),
            email: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login => {
                // self.value += 1;
                true
            }
            Msg::PasswordUpdate(s) => {
                self.password = s;
                true
            }
            Msg::EmailUpdate(s) => {
                self.email = s;
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
            <p>{ "Please enter your email and password" }</p>
            // <input class="field" type="password" placeholder="Password" id="password"/>
            <input class="field"
                type="text"
                placeholder="email"
                id="email"
                value={self.email.clone()}
                oninput={link.callback(|e: InputEvent| {
                    let s: String = e.target_unchecked_into::<HtmlInputElement>().value();
                    Msg::EmailUpdate(s)
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
            </div>
        }
    }
}
