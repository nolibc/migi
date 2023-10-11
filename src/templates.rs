use crate::{
    cache::CacheData,
    source::html_file_name, CONTENT_CACHE,
    logging,
};
use std::{
    collections::HashSet,
    fs::{self, write, File},
    io::Read, path::{PathBuf, Path},
};
use {once_cell::sync::Lazy, regex::Regex};

pub fn template_engine(change_file: &PathBuf) {

    if change_file.is_dir() {
        return
    }

    let stemmed_template_dir = change_file.to_string_lossy().to_string().replace("templates/", "build/");
    let stemmed_path = Path::new(&stemmed_template_dir).parent().unwrap();

    if !stemmed_path.exists() {
        std::fs::create_dir_all(stemmed_path).unwrap_or_else(|e| {
            logging::error(format!("could not create {}\n {}", stemmed_path.display(), e).as_str());
        });
    }

    let mut buffer = String::new();

    File::open(CONTENT_CACHE)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let loaded_cache: Vec<CacheData> = serde_json::from_str(&buffer).unwrap();

    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{\{(.+?)\}\}").unwrap());

    let template_file_as_string = change_file.to_string_lossy().to_string();
    let template_file_content = fs::read_to_string(&template_file_as_string).unwrap();
    let mut formatted_template = template_file_content.clone();

    for mat in RE.captures_iter(&template_file_content) {
        let tag_section = &mat[1];
        let tags: Vec<&str> = tag_section.split_whitespace().collect();
        let mut tags_matched_file: Vec<(String, String)> = Vec::new();

        for tag in tags {
            for file in &loaded_cache {
                if file.tags.iter().any(|file_tag| file_tag == tag) {
                    let html_file = html_file_name(&file.name.clone());
                    tags_matched_file.push((html_file, file.title.clone()));
                }
            }
        }

        let replacement = li_href_generator(tags_matched_file);
        formatted_template = formatted_template.replace(&mat[0], &replacement);
    }
    let minification_config = minify_html_onepass::Cfg::new();
    let minified_template = minify_html_onepass::in_place_str(&mut formatted_template, &minification_config).unwrap();

    write(&template_file_as_string.replace("templates/", "build/"), &minified_template).unwrap();
}

fn li_href_generator(meta_data: Vec<(String, String)>) -> String {
    let mut unique_items = HashSet::new();
    let mut deduplicated_data = Vec::new();

    for item in meta_data {
        if unique_items.insert(item.1.clone()) {
            deduplicated_data.push(item);
        }
    }

    let mut container = String::from("<ul>\n");
    let output: String = deduplicated_data
        .iter()
        .map(|item| format!("<li><a href=\"page/{}\">{}</a></li>", item.0, item.1))
        .collect::<Vec<String>>()
        .join("\n");

    container.push_str(&output);
    container.push_str("\n</ul>");

    container
}
