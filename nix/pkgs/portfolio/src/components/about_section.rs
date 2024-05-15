use yew::prelude::*;

#[function_component(AboutSection)]
pub fn about_section() -> Html {
    html! {
        <div>
            <h1>{"About Us"}</h1>
            <p>{"We are a company that cares about our customers."}</p>
        </div>
    }
}
