use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use yew::virtual_dom::{VNode, VTag, VText};
use yew::{html, Classes, Html};

fn add_class(vtag: &mut VTag, class: impl Into<Classes>) {
    let mut classes: Classes = vtag
        .attributes
        .iter()
        .find(|(k, _)| *k == "class")
        .map(|(_, v)| Classes::from(v.to_owned()))
        .unwrap_or_default();
    classes.push(class);
    vtag.add_attribute("class", classes.to_string());
}

pub fn render_markdown(src: &str) -> Html {
    let mut elems = vec![];
    let mut spine = vec![];

    macro_rules! add_child {
        ($child:expr) => {{
            let l = spine.len();
            assert_ne!(l, 0);
            spine[l - 1].add_child($child);
        }};
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);

    let mut table_aligns = vec![];

    for ev in Parser::new_ext(src, options) {
        match ev {
            Event::Start(tag) => {
                if let Tag::Table(aligns) = &tag {
                    table_aligns.push(aligns.clone());
                }
                spine.push(make_tag(tag));
            }
            Event::End(tag_end) => {
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap();
                match tag_end {
                    TagEnd::CodeBlock => {
                        let mut pre = VTag::new("pre");
                        pre.add_child(top.into());
                        top = pre;
                    }
                    TagEnd::Table => {
                        if let Some(aligns) = table_aligns.pop() {
                            if let Some(top_children) = top.children_mut() {
                                for r in top_children.to_vlist_mut().iter_mut() {
                                    if let VNode::VTag(ref mut vtag) = r {
                                        if let Some(vtag_children) = vtag.children_mut() {
                                            for (i, c) in vtag_children.to_vlist_mut().iter_mut().enumerate() {
                                                if let VNode::VTag(ref mut vtag) = c {
                                                    match aligns[i] {
                                                        Alignment::None => {}
                                                        Alignment::Left => add_class(vtag, "text-left"),
                                                        Alignment::Center => add_class(vtag, "text-center"),
                                                        Alignment::Right => add_class(vtag, "text-right"),
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    TagEnd::TableHead => {
                        if let Some(top_children) = top.children_mut() {
                            for c in top_children.to_vlist_mut().iter_mut() {
                                if let VNode::VTag(ref mut vtag) = c {
                                    vtag.add_attribute("scope", "col");
                                }
                            }
                        }
                    }
                    _ => {}
                }
                if l == 1 {
                    elems.push(top);
                } else {
                    spine[l - 2].add_child(top.into());
                }
            }
            Event::Text(text) => add_child!(VText::new(text.to_string()).into()),
            Event::Rule => add_child!(VTag::new("hr").into()),
            Event::SoftBreak => add_child!(VText::new("\n").into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),
            _ => println!("Unknown event: {ev:#?}"),
        }
    }

    if elems.len() == 1 {
        VNode::VTag(Box::new(elems.pop().unwrap()))
    } else {
        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

fn make_tag(t: Tag) -> VTag {
    match t {
        Tag::Paragraph => VTag::new("p"),
        Tag::Heading { level, .. } => VTag::new(format!("h{}", level)),
        Tag::BlockQuote => {
            let mut el = VTag::new("blockquote");
            el.add_attribute("class", "blockquote");
            el
        }
        Tag::CodeBlock(code_block_kind) => {
            let mut el = VTag::new("code");

            if let CodeBlockKind::Fenced(lang) = code_block_kind {
                match lang.as_ref() {
                    "html" => el.add_attribute("class", "html-language"),
                    "rust" => el.add_attribute("class", "rust-language"),
                    "java" => el.add_attribute("class", "java-language"),
                    "c" => el.add_attribute("class", "c-language"),
                    _ => {} // Add your own language highlighting support
                };
            }

            el
        }
        Tag::List(None) => VTag::new("ul"),
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(ref start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start.to_string());
            el
        }
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => {
            let mut el = VTag::new("table");
            el.add_attribute("class", "table");
            el
        }
        Tag::TableHead => VTag::new("thead"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "font-italic");
            el
        }
        Tag::Strong => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "font-weight-bold");
            el
        }
        Tag::Link { dest_url, title, .. } => {
            let mut el = VTag::new("a");
            el.add_attribute("href", dest_url.to_string());
            let title = title.into_string();
            if !title.is_empty() {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::Image { dest_url, title, .. } => {
            let mut el = VTag::new("img");
            el.add_attribute("src", dest_url.to_string());
            let title = title.into_string();
            if !title.is_empty() {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::FootnoteDefinition(_) => VTag::new("span"),
        Tag::Strikethrough => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "text-decoration-strikethrough");
            el
        }
        _ => VTag::new("span"), // Default case for any other tags
    }
}
