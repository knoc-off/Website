use crate::components::link_item::LinkItem;
use crate::components::social_links::*;
use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {

    let logos = vec![
        LogoLinkProps {
            link: "https://www.linkedin.com/in/niko-selby/".to_string(),
            img_src: "/icons/share/icons/SuperTinyIcons/svg/linkedin.svg".to_string(),
            alt_text: "LinkedIn".to_string(),
        },
        LogoLinkProps {
            link: "https://github.com/knoc-off".to_string(),
            img_src: "/icons/share/icons/SuperTinyIcons/svg/github.svg".to_string(),
            alt_text: "GitHub".to_string(),
        },
        LogoLinkProps {
            link: "/static/cv.pdf".to_string(),
            img_src: "/icons/share/icons/SuperTinyIcons/svg/pdf.svg".to_string(),
            alt_text: "GitHub".to_string(),
        },
    ];

    html! {
        <nav>
            <div style="display: flex; align-items: center;">
                <LogoLinks logos={logos} />
            </div>

            <div style="display: flex; justify-content: center;">
                <LinkItem text={"Home".to_string()} href={"/".to_string()} />
                <LinkItem text={"About".to_string()} href={"/about".to_string()} />
                // Add additional navigation links here
            </div>
        </nav>
    }
}
