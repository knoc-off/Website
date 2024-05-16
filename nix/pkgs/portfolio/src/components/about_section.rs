use yew::prelude::*;
use super::markdown::MarkdownViewer;

use super::social_links::*;

#[function_component(AboutSection)]
pub fn about_section() -> Html {
    let markdown_content = r#"
I am a self-taught programmer with skills in Rust programming and security systems.

In 2023, I completed an IT course at the Technical University of Berlin, covering general topics within programming, security, system administration, network management, and web development.

Through my usage/focus of NixOS, I enhanced my Linux and system/server management skills and am most interested in exploring the potential applications of NixOS in CI/CD environments.
"#.to_string();


    html! {
        <div class="about-container">
            <img class="about-image" src="/static/img.png" alt="Description of Image" />
            <MarkdownViewer markdown={markdown_content} />
        </div>
    }
}
