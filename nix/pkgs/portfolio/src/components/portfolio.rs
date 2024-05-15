use yew::prelude::*;
use super::{navbar::Navbar, about_section::AboutSection};

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    html! {
        <>
            <Navbar />
            <AboutSection />
            // Include other components as needed
        </>
    }
}
