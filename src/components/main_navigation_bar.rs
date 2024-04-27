use crate::services::routing::Route;
use stylist::css;
use yew::prelude::*;
use yew_router::prelude::*;

const ROUTES: [Route; 8] = [
    Route::Home,
    Route::Leistungen,
    Route::Notfall,
    Route::Informationen,
    Route::ServiceLinks,
    Route::Bilder,
    Route::Anfahrt,
    Route::Impressum,
];

#[derive(PartialEq, Properties)]
pub struct Props {
    pub route: Route,
}

#[function_component]
pub fn MainNavigationBar(props: &Props) -> Html {
    let class = css!(
        "
            width: 100%;
            text-align: center;
            background-color: var(--PRIMARY-COLOR-GREEN);

            button {
                padding: 1em;
                padding-top: 2em;
                padding-bottom: 2em;
                background-color: var(--PRIMARY-COLOR-GREEN);
                color: rgb(114, 138, 113);
                text-transform: uppercase;
                font-weight: bold;
            }
            button.active {
                color: white;
            }
            button:hover {
                background-color: rgb(50, 150, 0);
                color: white;
            }
        "
    );

    let navigator = use_navigator().unwrap();
    html! {
        <div {class}>
            {
                ROUTES.iter().map(|route| {
                    let navigator = navigator.clone();
                    let onclick = Callback::from(move |_| navigator.push(route));
                    let class = if *route == props.route {
                        "active"
                    } else {""};
                    html!{
                        <button {onclick} {class}>
                            { route.to_title_string() }
                        </button>
                    }
                }).collect::<Vec<Html>>()
            }
        </div>
    }
}
