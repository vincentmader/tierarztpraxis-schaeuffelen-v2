use crate::screens::{
    AnfahrtScreen, BilderScreen, HomeScreen, ImpressumScreen, InformationenScreen,
    LeistungenScreen, NotfallScreen, PageNotFoundScreen, ServiceLinksScreen,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/anfahrt")]
    Anfahrt,
    #[at("/bilder")]
    Bilder,
    #[at("/impressum")]
    Impressum,
    #[at("/informationen")]
    Informationen,
    #[at("/leistungen")]
    Leistungen,
    #[at("/notfall")]
    Notfall,
    #[at("/service-links")]
    ServiceLinks,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    let screen = match routes {
        Route::Anfahrt => html! { <AnfahrtScreen /> },
        Route::Bilder => html! { <BilderScreen /> },
        Route::Home => html! { <HomeScreen /> },
        Route::Impressum => html! { <ImpressumScreen /> },
        Route::Informationen => html! { <InformationenScreen /> },
        Route::Leistungen => html! { <LeistungenScreen /> },
        Route::Notfall => html! { <NotfallScreen /> },
        Route::NotFound => html! { <PageNotFoundScreen /> },
        Route::ServiceLinks => html! { <ServiceLinksScreen /> },
    };

    html!(
        <div>
            { screen }
        </div>
    )
}
