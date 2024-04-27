use stylist::css;
use yew::prelude::*;

#[function_component]
pub fn MainSideBar() -> Html {
    let class = css!(
        "
            min-height: calc(100vh - 3em - 3em - 3em);
            background-color: var(--PRIMARY-COLOR-GREEN);
            padding: var(--MAIN-PAD);
        "
    );

    html! {
        <div {class}>
            <p class="bold">{ "Telefon" }</p>
            <p>{ "0731 / 65836" }</p>
            <br />

            <p class="bold">{ "Notruf" }</p>
            <p>{ "0177 / 20 65 0 66" }</p>
            <br />

            <p class="bold">{ "Sprechzeiten" }</p>
            <p>
                { "Montag bis Freitag:" }<br />
                { "10:00 - 15:00 Uhr" }
            </p>
            <br />
            <p>
                { "Donnerstag:" }<br/>
                { "17:00 - 19:00 Uhr" }
            </p>
            <br />

            <p class="bold">{ "Anschrift" }</p>
            <p>
                { "Bessererstraße 17" }<br />
                { "89073 Ulm" }
            </p>
        </div>
    }
}
