use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LogoLinkProps {
    pub link: String,
    pub img_src: String,
    pub alt_text: String,
}

#[function_component(LogoLink)]
pub fn logo_link(props: &LogoLinkProps) -> Html {
    html! {
        <a href={props.link.clone()} target="_blank">
            <img src={props.img_src.clone()} alt={props.alt_text.clone()} style="width: 100px; height: auto;"/>
        </a>
    }
}

#[derive(Properties, PartialEq)]
pub struct LogoLinksProps {
    pub logos: Vec<LogoLinkProps>,
}


#[function_component(LogoLinks)]
pub fn logo_links(props: &LogoLinksProps) -> Html {
    html! {
        <div class="logo-links-container">
            { for props.logos.iter().map(|logo| html! {
                <LogoLink link={logo.link.clone()} img_src={logo.img_src.clone()} alt_text={logo.alt_text.clone()} />
            })}
        </div>
    }
}
