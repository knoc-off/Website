use yew::prelude::*;

mod components;
use crate::components::{navbar::Navbar, footer::Footer, main_content::MainContent};

// Example usage within the App component
#[function_component(App)]
pub fn app() -> Html {

    html! {
        <div>
            <Navbar />
            <MainContent />
            <Footer />
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}




