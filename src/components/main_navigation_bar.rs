use crate::services::routing::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn MainNavigationBar() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick_0 = Callback::from(move |_| navigator.push(&Route::Home));
    let navigator = use_navigator().unwrap();
    let onclick_1 = Callback::from(move |_| navigator.push(&Route::Leistungen));
    let navigator = use_navigator().unwrap();
    let onclick_2 = Callback::from(move |_| navigator.push(&Route::Notfall));
    let navigator = use_navigator().unwrap();
    let onclick_3 = Callback::from(move |_| navigator.push(&Route::Informationen));
    let navigator = use_navigator().unwrap();
    let onclick_4 = Callback::from(move |_| navigator.push(&Route::ServiceLinks));
    let navigator = use_navigator().unwrap();
    let onclick_5 = Callback::from(move |_| navigator.push(&Route::Bilder));
    let navigator = use_navigator().unwrap();
    let onclick_6 = Callback::from(move |_| navigator.push(&Route::Anfahrt));
    let navigator = use_navigator().unwrap();
    let onclick_7 = Callback::from(move |_| navigator.push(&Route::Impressum));

    html! {
        <div>
            <button onclick={onclick_0}>{ "Willkommen" }</button>
            <button onclick={onclick_1}>{ "Leistungen" }</button>
            <button onclick={onclick_2}>{ "Notfall" }</button>
            <button onclick={onclick_3}>{ "Informationen" }</button>
            <button onclick={onclick_4}>{ "Service / Links" }</button>
            <button onclick={onclick_5}>{ "Bilder" }</button>
            <button onclick={onclick_6}>{ "Anfahrt" }</button>
            <button onclick={onclick_7}>{ "Impressum" }</button>
        </div>
    }
}
