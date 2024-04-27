use stylist::css;
use yew::prelude::*;

#[function_component]
pub fn NotfallScreen() -> Html {
    let class = css!(
        "
            padding: var(--MAIN-PAD);
        "
    );

    html! {
        <div {class}>
            <p>{ "In dringenden Notfällen versuchen Sie bitte, mich unter der Mobilfunknummer " }
                <span class="bold">{ "0177 / 20 65 066" }</span>{ " zu erreichen." }</p>
            <br />
            <p>{ "Sollte das nicht gelingen, wenden Sie sich bitte an die zentrale Notrufnummer \
                  der tierärztlichen Notdienstgemeinschaft " }<span class="bold">{ "0700 / 12 16 16 16" }
                  </span>{ "." }</p>
        </div>
    }
}
