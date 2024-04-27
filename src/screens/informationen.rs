use stylist::css;
use yew::prelude::*;

#[function_component]
pub fn InformationenScreen() -> Html {
    let class = css!(
        "
            padding: var(--MAIN-PAD);
        "
    );

    html! {
        <div {class}>
            <p>{ "Hier bekommen Sie wichtige Informationen zur Gesundheit Ihres Tieres." }</p>
            <br />
            <p>{ "Bitte klicken Sie auf einen der Links auf der rechten Seite für die \
                  benötigten Informationen." }</p>
        </div>
    }
}
