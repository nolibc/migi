use walkdir::WalkDir;

use crate::{default, PAGE_TEMPLATE, logging, CONTENT};
use std::{
    fs::{self},
    io,
    path::{PathBuf, Path},
};

pub fn setup_new_project(project_name: &str) -> Result<(), io::Error> {
    let root_directory = PathBuf::from(project_name);
    let directories = vec!["content", "assets/syntax", "assets/css", "templates"];

    fs::create_dir(&root_directory)?;
    logging::info(format!("created directory `./{}`", &root_directory.to_string_lossy()).as_str());

    for dir in directories {
        let dir_path = root_directory.join(&PathBuf::from(dir));
        fs::create_dir_all(&dir_path)?;

        logging::info(format!("created directory `./{}`", &dir_path.to_string_lossy()).as_str());
    }

    let config_path = format!("{}/config.toml", &root_directory.to_string_lossy());
    let theme_path = format!(
        "{}/assets/syntax/Tomorrow-Night.tmTheme",
        &root_directory.to_string_lossy()
    );
    let page_template = format!("{}/{}", &root_directory.to_string_lossy(), PAGE_TEMPLATE);
    let index_path = format!("{}/templates/index.html", &root_directory.to_string_lossy());
    let css_path = format!("{}/assets/css/style.css", &root_directory.to_string_lossy());
    let generic_post = format!("{}/content/first_post.md", &root_directory.to_string_lossy());

    fs::write(config_path, default::get_config())?;
    fs::write(theme_path, default::get_theme())?;
    fs::write(page_template, default::get_page_template())?;
    fs::write(index_path, default::get_index_template())?;
    fs::write(css_path, default::get_css())?;
    fs::write(generic_post, default::get_generic_post())?;

    // We exit due to how the arguments are handled. If we did not do this,
    // the user would see an error about an invalid argument (i.e. the name of the project).
    std::process::exit(1);
}

pub fn markdown_file_names() -> Result<Vec<PathBuf>, io::Error> {
    let mut captured_vec: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(CONTENT) {
        if let Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                if ext.to_str().unwrap() == "md" {
                    captured_vec.push(entry.into_path());
                } else {
                    logging::warn(format!("file {} is not a markdown file and has been ignored.", entry.path().display()).as_str());
                }
            }
        }
    }

    Ok(captured_vec)
}

#[derive(Default, Debug)]
pub struct HeaderParser {
    pub title: String,
    pub tags: Vec<String>,
}

impl HeaderParser {
    fn get_header(text: &str) -> Option<Vec<String>> {
        let mut yielded: Vec<String> = Vec::with_capacity(text.len());
        let mut in_block_toggle = 0;

        for line in text.lines() {
            if line.starts_with("---") {
                in_block_toggle += 1;
                continue;
            }
            if in_block_toggle > 1 {
                break;
            }
            if in_block_toggle == 1 {
                yielded.push(line.to_string());
            }
        }
        if yielded.is_empty() {
            None
        } else {
            Some(yielded)
        }
    }
    pub fn get_data(text: &str) -> Option<HeaderParser> {
        let mut metadata = HeaderParser::default();
        let mut has_valid_title = true;
        if let Some(header_lines) = Self::get_header(&text) {
            for line in header_lines {
                if line.starts_with("title:") {
                    match HeaderParser::get_title(&line) {
                        Some(title) => {
                            metadata.title = title;
                        }
                        None => {
                            has_valid_title = false;
                            logging::error("Error: All files must contain a valid title.");
                        }
                    }
                }
                if line.starts_with("tags:") {
                    if let Some(tag) = HeaderParser::get_tags(&line) {
                        metadata.tags = tag;
                    }
                }
            }
        };
        if has_valid_title {
            return Some(metadata);
        } else {
            None
        }
    }

    fn get_title(header: &str) -> Option<String> {
        let index = header.find("title:").unwrap();
        let title = header[(index + 6)..].trim();
        if !title.is_empty() {
            return Some(title.to_owned());
        }
        None
    }

    fn get_tags(header: &str) -> Option<Vec<String>> {
        if let Some(index) = header.find("tags:") {
            let tags: Vec<String> = header[(index + 5)..]
                .split_ascii_whitespace()
                .map(|t| t.to_string())
                .collect();
            return Some(tags);
        }

        None
    }
}

pub fn html_file_name(md_file_name: &PathBuf) -> PathBuf {
    let file_stemmed = md_file_name.file_stem().and_then(|stem| Some(stem.to_string_lossy().to_string()));
    match file_stemmed {
        Some(file) => {
            let path = Path::new(&file).to_path_buf();
            return path;
        },
        None => {
            logging::warn("issues stemming a file... skipping for now.");
            return Path::new("").to_path_buf();
        }
    }
}
