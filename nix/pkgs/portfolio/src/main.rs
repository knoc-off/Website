use yew::prelude::*;
use web_sys::HtmlElement;
use web_sys::wasm_bindgen::JsCast;

struct Portfolio {
    name: String,
    github_url: String,
    linkedin_url: String,
}

impl Component for Portfolio {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Portfolio {
            name: "Nicholas Selby".to_string(),
            github_url: "https://github.com/knoc-off".to_string(),
            linkedin_url: "https://www.linkedin.com/in/niko-selby/".to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let cv_onclick = Callback::from(|_| {
            let document = web_sys::window().unwrap().document().unwrap();
            let link = document.create_element("a").unwrap();
            let link_elem = link.dyn_into::<HtmlElement>().unwrap();
            link_elem.set_attribute("href", "static/cv.pdf").unwrap();
            link_elem.set_attribute("download", "cv.pdf").unwrap();
            link_elem.click();
        });

        html! {
            <div class="main-container">
                <div class="photo-container">
                    <img src="static/img.png" alt={ self.name.clone() } />
                    <nav>
                        <ul>
                            <li>
                                <a href={ self.github_url.clone() } target="_blank">
                                    <img src="icons/share/icons/SuperTinyIcons/svg/github.svg" alt="GitHub" width="60" height="60" />
                                </a>
                            </li>
                            <li>
                                <a href={ self.linkedin_url.clone() } target="_blank">
                                    <img src="icons/share/icons/SuperTinyIcons/svg/linkedin.svg" alt="LinkedIn" width="60" height="60" />
                                </a>
                            </li>
                            <li>
                                <a href="#" onclick={ cv_onclick }>
                                    <img src="icons/share/icons/SuperTinyIcons/svg/pdf.svg" alt="CV" width="60" height="60" />
                                </a>
                            </li>
                        </ul>
                    </nav>
                </div>
                <div class="container">
                <header>
                    <h1>{ &self.name }</h1>
                    <button id="theme-toggle">{"Toggle Theme" } </button>
                    <script src="theme.js"></script>
                </header>

                <main>
                    <section class="about">
                        <h2>{ "About Me" }</h2>
                        <div class="about-content">
                            <div class="about-text">
                                <p>{"I am a self-taught programmer passionate about technology. I have been developing my skills in various areas of computing from a young age. My interests include Rust programming and security systems, where I have gained practical expertise."}</p>

                                <p>{"I completed an IT course at the Technical University of Berlin. Though I excel in programming and security, I have cultivated a broad skill set that encompasses system administration, network management, and web development."}</p>

                                <p>{"I have a focus on NixOS, which has greatly enhanced my proficiency in Linux and system/server management. I use my computer as a testbed for experimentation and am keen to explore the potential applications of NixOS in CI/CD environments."}</p>

                            </div>
                        </div>
                    </section>

                    <section class="projects">
                        <h2>{"Projects"}</h2>
                        <ul>
                            <li>
                                <a href="https://github.com/knoc-off/Website" target="_blank">
                                    {"Yew Website"}
                                </a>
                                {": This website, built using the Yew framework, which I aim to grow into a more substantial project"}
                            </li>
                            <li>
                                <a href="https://github.com/knoc-off/nixos" target="_blank">
                                    {"NixOS Dotfiles"}
                                </a>
                                {": A collection of packaged apps and solutions for various tasks, showcasing my experience with NixOS and system configuration."}
                            </li>
                            <li>
                                <a href="https://github.com/knoc-off/DiscordGPT-rs" target="_blank">
                                    {"DiscordGPT-rs"}
                                </a>
                                {": A Discord bot built for fun during the peak of the LLM hype"}
                            </li>
                        </ul>
                    </section>
                </main>
                <footer>
                    <p>{ format!("© 2024 {}. All rights reserved.", &self.name) }</p>
                </footer>
            </div>
        </div>
        }
    }
}

fn main() {
    yew::Renderer::<Portfolio>::new().render();
}
