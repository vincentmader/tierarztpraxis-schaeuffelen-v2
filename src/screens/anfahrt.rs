use stylist::css;
use yew::prelude::*;

const API_KEY: &str = "AIzaSyCRUvGiYkIXJMRiUeshAk4OQvuM7xJECMU";

#[function_component]
pub fn AnfahrtScreen() -> Html {
    let class = css!(
        "
            padding: var(--MAIN-PAD);

            iframe {
                width: 100%;
                height: 60vh;
            }
        "
    );

    let src = format!(
        "https://www.google.de/maps/embed/v1/place?key={}&language=de&q=Tierarztpraxis+Schäuffelen+Ulm",
        API_KEY
    );
    html! {
        <div {class}>
            // <p>{ "Tierarzt Ulm Dr. Angelika Schäuffelen 89073 Ulm Bessererstraße 17" }</p>
            // <p>{ "Tel.: 0731 / 65836" }</p>
            <iframe
                style="border:0"
                loading="lazy" scrolling="yes"
                referrerpolicy="no-referrer-when-downgrade"
                {src}>
            </iframe>
            // <div>
                // <a href="https://www.google.de/maps/search/?api=1&query=Dr.+Angelika+Schäuffelen+Bessererstraße+17+Ulm+89073+Ulm">{ "Größere Kartenansicht" }</a>
            // </div>
        </div>
    }
}
