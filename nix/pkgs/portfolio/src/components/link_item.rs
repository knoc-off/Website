use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkItemProps {
    pub text: String,
    pub href: String,
}

#[function_component(LinkItem)]
pub fn link_item(props: &LinkItemProps) -> Html {
    html! {
        <a href={props.href.clone()}>{ &props.text }</a>
    }
}
