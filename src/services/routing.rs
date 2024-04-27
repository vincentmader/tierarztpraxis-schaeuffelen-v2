use crate::{
    components::{MainFooter, MainHero, MainNavigationBar, MainSideBar},
    screens::{
        AnfahrtScreen, BilderScreen, HomeScreen, ImpressumScreen, InformationenScreen,
        LeistungenScreen, NotFoundScreen, NotfallScreen, ServiceLinksScreen,
    },
};
use stylist::css;
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

impl Route {
    pub fn to_title_string(&self) -> String {
        match self {
            Self::Home => "Willkommen",
            Self::Anfahrt => "Anfahrt",
            Self::Bilder => "Bilder",
            Self::Impressum => "Impressum",
            Self::Informationen => "Informationen",
            Self::Leistungen => "Leistungen",
            Self::Notfall => "Notfall",
            Self::ServiceLinks => "Service / Links",
            Self::NotFound => "404",
        }
        .to_owned()
    }
}

pub fn switch(route: Route) -> Html {
    let screen = match route {
        Route::Anfahrt => html! { <AnfahrtScreen /> },
        Route::Bilder => html! { <BilderScreen /> },
        Route::Home => html! { <HomeScreen /> },
        Route::Impressum => html! { <ImpressumScreen /> },
        Route::Informationen => html! { <InformationenScreen /> },
        Route::Leistungen => html! { <LeistungenScreen /> },
        Route::Notfall => html! { <NotfallScreen /> },
        Route::NotFound => html! { <NotFoundScreen /> },
        Route::ServiceLinks => html! { <ServiceLinksScreen /> },
    };

    let class = css!(
        "
            width: 100%;

            .content {
            }
            span.left {
                display: inline-block;
                width: 70%;
                float: left;
            }
            span.right {
                display: inline-block;
                width: 30%;
            }
        "
    );

    html!(
        <div {class}>
            <MainNavigationBar route={route.clone()} />
            <span class="left">
                <MainHero route={route} />
                <div class="content">
                    { screen }
                </div>
            </span>
            <span class="right"><MainSideBar /></span>
            <MainFooter />
        </div>
    )
}
