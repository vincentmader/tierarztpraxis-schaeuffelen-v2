use crate::services::routing::Route;
use stylist::css;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub route: Route,
}

#[function_component]
pub fn MainHero(props: &Props) -> Html {
    let class = css!(
        "
            background-color: var(--PRIMARY-COLOR-GREEN);
            color: white;
            font-weight: bold;

            span {
                display: inline-block;
            }
            span.left {
                width: 40%;
            }
            span.left p {
                padding: var(--MAIN-PAD);
            }
            span.right {
                width: 60%;
                text-align: right;
                padding: 0px;
            }
            span.right img {
                margin: 0px;
                padding: 0px;
            }
        "
    );

    let img_src = match props.route {
        Route::Notfall => "NotfallKoffer400_300.jpg",
        Route::Bilder => "PraxiseingangOst.jpg",
        Route::Anfahrt => "PraxiseingangWest.jpg",
        _ => "G16_cropped.png",
    };
    let img_src = format!("img/{}", img_src);
    let img_width = match props.route {
        Route::Notfall | Route::Bilder | Route::Anfahrt => "300",
        _ => "200",
    };

    html! {
        <div {class}>
            <span class="left">
                <p>{ "Tierarztpraxis Ulm" }</p>
                <p>{ "Dr. med. vet." }<br />{ "Angelika Schäuffelen" }</p>
            </span>
            <span class="right">
                <img
                    width={ img_width }
                    src={ img_src }
                    alt="http://tierarztpraxis-schäuffelen.de"
                />
            </span>
        </div>
    }
}
