use yew::prelude::*;
use super::markdown::MarkdownViewer;

use super::social_links::*;

#[function_component(AboutSection)]
pub fn about_section() -> Html {
    let markdown_content = r#"
I am a self-taught programmer. I have developed skills in Rust programming and security systems.

I completed an IT course at the Technical University of Berlin, excelling in programming, security, system administration, network management, and web development.

I have a focus on NixOS, enhancing my proficiency in Linux and system/server management.

I am interested in exploring the potential applications of NixOS in CI/CD environments.
    "#.to_string();


    html! {
        <div class="about-container">
            <img class="about-image" src="/static/img.png" alt="Description of Image" />
            <MarkdownViewer markdown={markdown_content} />
        </div>
    }
}
