use yew::prelude::*;
use super::markdown::MarkdownViewer;

use super::social_links::*;

#[function_component(AboutSection)]
pub fn about_section() -> Html {
    let markdown_content = r#"
# About Me
- This is a summary of me

    "#.to_string();


    html! {
        <div class="about-container">
            <img class="about-image" src="/static/img.png" alt="Description of Image" />
            <MarkdownViewer markdown={markdown_content} />
        </div>
    }
}
