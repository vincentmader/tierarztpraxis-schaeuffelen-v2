use crate::services::routing::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn MainFooter() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick_0 = Callback::from(move |_| navigator.push(&Route::Home));
    let navigator = use_navigator().unwrap();
    let onclick_1 = Callback::from(move |_| navigator.push(&Route::Anfahrt));
    let navigator = use_navigator().unwrap();
    let onclick_2 = Callback::from(move |_| navigator.push(&Route::Impressum));

    html! {
        <div>
            <button onclick={onclick_0}>{ "Startseite" }</button>
            <button onclick={onclick_1}>{ "Anfahrt" }</button>
            <button onclick={onclick_2}>{ "Impressum" }</button>
        </div>
    }
}
