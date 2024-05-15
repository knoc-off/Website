use crate::components::link_item::LinkItem;
use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav>
            <LinkItem text={"Home".to_string()} href={"/".to_string()} />
            <LinkItem text={"About".to_string()} href={"/about".to_string()} />
            // Add additional navigation links here
        </nav>
    }
}
