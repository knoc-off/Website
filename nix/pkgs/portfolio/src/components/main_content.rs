use yew::prelude::*;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <main>
            <section>
                <h2>{"Our Services"}</h2>
                <p>{"We offer high-quality services."}</p>
            </section>
            <section>
                <h2>{"Contact Us"}</h2>
                <p>{"Email us at contact@example.com"}</p>
            </section>
        </main>
    }
}
