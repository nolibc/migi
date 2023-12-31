use crate::logging;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
use std::{borrow::Cow, path::{PathBuf, Path}, panic};
use syntect::{
    highlighting::ThemeSet,
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Deserialize, Serialize)]
pub struct Config {
    themes: Themes,
    pub tagging: Tagging,
}

#[derive(Deserialize, Serialize)]
struct Themes {
    syntax: String,
}

#[derive(Deserialize, Serialize)]
pub struct Tagging {
    pub sorted: bool,
}

fn syntect_highlight<'a>( code_snippet: String,
    language_name: &str,
) -> Result<Event<'a>> {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set
        .find_syntax_by_token(language_name)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);

    for line in LinesWithEndings::from(&code_snippet) {
        html_generator
            .parse_html_for_line_which_includes_newline(line)
            .unwrap();
    }

    let config_file = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config_file).unwrap();
    let theme_name = format!("assets/syntax/{}", config.themes.syntax);

    if !Path::new(&format!("./assets/syntax/{}", &theme_name)).is_file() {
        let theme_ref = theme_name.to_owned();
        panic::set_hook(Box::new(move |_| {
            logging::error(format!("Could not find syntax-theme: {}", &theme_ref).as_str());
        }));
    }

    let user_theme = ThemeSet::get_theme(theme_name);
    let highlighted_result_type = syntect::html::highlighted_html_for_string(
        &code_snippet,
        &syntax_set,
        &syntax,
        &user_theme.unwrap(),
    );

    Ok(Event::Html(
        Cow::from(highlighted_result_type.unwrap()).into(),
    ))
}

pub fn compile(markdown_input: &str) -> String {
    let parser_options = Options::all();
    let parser = Parser::new_ext(markdown_input, parser_options);

    let mut is_code_block = false;
    let mut event_parser: Vec<Event> = Vec::new();

    let mut language_name = String::new();

    for event in parser.into_iter() {
        match event {
            Event::Start(Tag::CodeBlock(fenced_snippet)) => {
                match fenced_snippet {
                    CodeBlockKind::Fenced(language) => {
                        language_name = language.to_string();
                    }
                    _ => {}
                }
                is_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                is_code_block = false;
            }
            Event::Text(text_object) => {
                if is_code_block {
                    let highlighted = syntect_highlight(text_object.to_string(), &language_name);
                    event_parser.push(highlighted.unwrap());
                } else {
                    event_parser.push(Event::Text(text_object));
                }
            }
            _ => {
                event_parser.push(event);
            }
        }
    }

    let mut string_holder = String::new();
    html::push_html(&mut string_holder, event_parser.into_iter());

    string_holder
}

pub fn remove_header(file_name: &PathBuf, file_content: &mut String) {
    let mut in_block_toggle = 0;
    let mut cursor = 0;

    for line in file_content.lines() {
        if line.starts_with("---") {
            in_block_toggle += 1;
            cursor += line.len();
            continue;
        }
        if in_block_toggle >= 2 {
            cursor += 4;
            break;
        }
        if in_block_toggle == 1 {
            cursor += line.len();
        }
    }

    if in_block_toggle == 0 {
        logging::warn(format!("`{}` does not contain a valid header", &file_name.to_string_lossy()).as_str());
    }

    *file_content = file_content.split_off(cursor)
}
