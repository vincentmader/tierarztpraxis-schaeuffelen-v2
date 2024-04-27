use stylist::css;
use yew::prelude::*;

#[function_component]
pub fn ServiceLinksScreen() -> Html {
    let class = css!(
        "
            padding: var(--MAIN-PAD);

            a {
                color: var(--PRIMARY-COLOR-GREEN);
                text-decoration: none;
            }
            a:hover {
                color: green;
                text-decoration: underline;
            }
        "
    );

    html! {
        <div {class}>
            <p><a href="http://hamster-info.net">{ "http://hamster-info.net" }</a></p>
            <p><a href="http://pro-igel.de/">{ "http://pro-igel.de/" }</a></p>
            <p><a href="http://esccap.de">{ "http://esccap.de" }</a></p>
            <br />
            <p>{ "Hundeausbildung:" }</p>
            <p><a href="http://family-dog.com/">{ "http://family-dog.com/" }</a></p>
            <p><a href="http://ulmer-hundeschule.de/">{ "http://ulmer-hundeschule.de/" }</a></p>
            <p><a href="http://hundezentrum-ulm.com/">{ "http://hundezentrum-ulm.com/" }</a></p>
            <br />
            <p>{" Rosengarten Tierbestattung:  "}
            <a href="http://www.tierbestatter-ulm.de/">{ "http://www.tierbestatter-ulm.de/" }</a></p>
            <br />
            <p>{ "Tierheim Ulm, Örlinger Tal Weg 46, 89073 Ulm, " }
            <a href="http://tierheim-ulm.de/">{ "http://tierheim-ulm.de/" }</a></p>
            <br />
            <p>{ "Veterinärsamt Ulm, Zeughausgasse 14, 89083 Ulm, 0731 / 185 38 30" }</p>
        </div>
    }
}
