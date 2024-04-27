use crate::services::routing::Route;
use chrono::Local;
use stylist::css;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn MainFooter() -> Html {
    let class = css!(
        "
            p {
                font-size: 8pt;
                padding: var(--MAIN-PAD);
            }
            span.left {
                width: 70%;
            }
            span.left p {
                padding-left: var(--MAIN-PAD);
            }
            span.right {
                width: 30%;
                background-color: var(--PRIMARY-COLOR-GREEN);
                text-align: right;
            }
            span.right p {
                padding-left: var(--MAIN-PAD);
            }
        "
    );

    let navigator = use_navigator().unwrap();
    let onclick_0 = Callback::from(move |_| navigator.push(&Route::Home));
    let navigator = use_navigator().unwrap();
    let onclick_1 = Callback::from(move |_| navigator.push(&Route::Anfahrt));
    let navigator = use_navigator().unwrap();
    let onclick_2 = Callback::from(move |_| navigator.push(&Route::Impressum));

    let now = Local::now();
    let year = now.format("%Y");
    let today = now.format("%d.%m.%Y");

    html! {
        <div {class}>
            <span class="left">
                <p>
                    <br />
                    <br />
                    <span>
                        <button onclick={onclick_0}>{ "Startseite" }</button>
                        { " | " }
                        <button onclick={onclick_1}>{ "Anfahrt" }</button>
                        { " | " }
                        <button onclick={onclick_2}>{ "Impressum" }</button>
                    </span>
                </p>
            </span>
            <span class="right">
                <p>
                    <span>
                        { "© " }{ year }{" Tierarztpraxis Ulm"}
                        <br />
                        { "Dr. med. vet. Angelika Schäuffelen" }
                        <br />
                        { "Letzte Aktualisierung am " }{ today }
                    </span>
                </p>
            </span>
        </div>
    }
}
