use stylist::css;
use tierarzt_server::services::routing::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn App() -> Html {
    let class = css!(
        "
            --PRIMARY-COLOR-GREEN: rgb(52, 204, 0);
            --PRIMARY-COLOR-GRAY: rgb(56, 58, 58);
            --MAIN-PAD: 1em;

            width: 100%;
            font-family: Verdana;
            color: var(--PRIMARY-COLOR-GRAY);

            * {
                padding: 0;
                margin: 0;
            }
            button {
                border: none;
                background: none;
            }
            button:hover {
                cursor: pointer;
            }
            .bold {
                font-weight: bold;
            }
        "
    );

    html! {
        <div {class}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
