use stylist::css;
use yew::prelude::*;

#[function_component]
pub fn LeistungenScreen() -> Html {
    let class = css!(
        "
            padding: var(--MAIN-PAD);

            ul {
                margin-left: 1rem;
                list-style-type: square;
                padding-left: var(--MAIN-PAD);
            }
            ul li {
                padding-top: calc(0.5 * var(--MAIN-PAD));
                padding-bottom: calc(0.5 * var(--MAIN-PAD));
            }
        "
    );

    html! {
        <div {class}>
            <p>{ "Das Leistungsspektrum der Kleintierpraxis umfasst unter anderem:" }</p>
            <ul>
                <li><p>{ "Schutzimpfungen mit Erinnerungsservice, Gesundheitszeugnisse \
                          sowie Beratung bei Auslandsreisen" }</p></li>
                <li><p>{ "Chirurgie: Im separaten Operationsraum werden Kastrationen, Tumor- \
                          und andere Weichteiloperationen durchgeführt" }</p></li>
                <li><p>{ "Tierkennzeichnung mit Mikrochip, Erstellen eines Heimtierausweises" }</p></li>
                <li><p>{ "Gesundheitsvorsorge" }</p></li>
                <li><p>{ "Fütterungsberatung" }</p></li>
                <li><p>{ "Parasitenprophylaxe" }</p></li>
                <li><p>{ "Sofortlabordiagnostik" }</p></li>
                <li><p>{ "Röntgen" }</p></li>
                <li><p>{ "Zahnbehandlung, Zahnsteinentfernung, Polieren der Zähne, \
                          Zahnextraktionen" }</p></li>
                <li><p>{ "Innere Medizin" }</p></li>
                <li><p>{ "Homöopathie, Naturheilkunde" }</p></li>
                <li><p>{ "Lasertherapie, die sanfte Methode zur Beschleunigung des Heilprozesses" }</p></li>
            </ul>
        </div>
    }
}
