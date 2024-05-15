use yew::prelude::*;

use crate::components::{ about_section::AboutSection  , projects::*};

#[function_component(MainContent)]
pub fn main_content() -> Html {
    let projects = vec![
        Project {
            name: "My Website".to_string(),
            image_url: "/icons/share/icons/SuperTinyIcons/svg/webassembly.svg".to_string(),
            summary_md: "
Website leveraging the Yew framework, which compiles to wasm. I aim to grow this project into something more substantial.

".to_string(),
            link: "https://github.com/knoc-off/nixos/".to_string()
        },
        Project {
            name: "Discord-GPT".to_string(),
            image_url: "/icons/share/icons/SuperTinyIcons/svg/discord.svg".to_string(),
            summary_md: "
The ChatGPT-Discord-Bot is a Discord bot utilizing OpenAI's GPT to enable dynamic, sentiment-based conversations within Discord channels. It features context-aware interactions, semi-persistent channel-specific conversations, and automatic resets to maintain relevance and performance.
".to_string(),
            link: "https://github.com/knoc-off/nixos/".to_string()
        },
        Project {
            name: "My Nixos Configs".to_string(),
            image_url: "/icons/share/icons/SuperTinyIcons/svg/nixos.svg".to_string(),
            summary_md: "
My nixos configs, this defines all of my systems.
server management, custom pc, raspberry pi, etc.

".to_string(),
            link: "https://github.com/knoc-off/nixos/".to_string()
        },
    ];



    html! {
        <main>
            <section>
                <AboutSection />
                <Projects projects={projects} />
            </section>
        </main>
    }
}
