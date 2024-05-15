use yew::prelude::*;
use super::markdown::MarkdownViewer;

#[function_component(AboutSection)]
pub fn about_section() -> Html {
    let markdown_content = r#"
# About Section
- this is a summary of me
- test
    "#.to_string();
    html! {
        <div>
            <MarkdownViewer markdown={markdown_content} />
        </div>
    }
}
