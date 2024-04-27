use stylist::css;
use yew::prelude::*;

#[function_component]
pub fn HomeScreen() -> Html {
    let class = css!(
        "
            padding: var(--MAIN-PAD);
        "
    );

    html! {
        <div {class}>
            <p class="bold">{ "Herzlich willkommen!" }</p>
            <br />
            <p>{ "In meiner Praxis steht Ihr Tier im Mittelpunkt." }</p>
            <br />
            <p>{ "Für mich ist meine Arbeit nicht Beruf, sondern Berufung." }</p>
            <br />
            <p>{ "Schon als Kind stand bei mir der Berufswunsch Tierärztin fest. \
                  Konnte ich doch schon früh meinen Vater in die Praxis begleiten, \
                  ihn beobachten und ihm helfen. Nach dem Abitur begann ich 1985 mein \
                  Studium der Tiermedizin an der FU Berlin, das ich 1991 dort abschloss. \
                  Während des Schreibens meiner Dissertation arbeitete ich bereits in einer \
                  großen Berliner Tierarztpraxis. 1995 kehrte ich nach Ulm zurück und übernahm \
                  die Praxis meines Vaters." }</p>
            <br />
            <p>{ "Das Leistungsspektrum der Kleintierpraxis umfasst die komplette \
                  Rundumversorgung aller Kleintiere." }</p>
            <br />
            <p>{ "Durch Terminsprechstunde keine oder nur geringe Wartezeiten." }</p>
            <br />
            <p>{ "Die Praxis liegt zentral und ist gut mit öffentlichen \
                  Verkehrsmitteln zu erreichen (Linien 1, 3, 5, 6, 7, 8)." }</p>
            <br />
            <p>{ "Parkmöglichkeit besteht vor der Praxis." }</p>
            <br />
            <p>{ "🇩🇪🇮🇹🇬🇧" }</p>
        </div>
    }
}
