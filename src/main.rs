use std::{
    env,
    fs::{self, create_dir_all, write},
    io,
    path::Path,
};

use walkdir::WalkDir;

mod cache;
mod default;
mod markdown;
mod source;
mod templates;
mod logging;

pub const BUILD_DIR: &str = "build";
pub const PAGE_BUILD_DIR: &str = "build/page";
pub const TEMPLATES_DIR: &str = "templates/";
pub const PAGE_TEMPLATE: &str = "templates/page.html";
pub const CONTENT_CACHE: &str = "cache/content.json";

fn usage(program: &str) {
    eprintln!(
"Usage: {program} [command]
An extremely minimal static site generator.

Commands:
    new <directory>     create new project directory
    build               build project outputting html\n");
}

fn main() -> Result<(), io::Error> {
    let mut arguments = env::args();
    let program = arguments.next().unwrap();

    let subcommand = arguments.next().unwrap_or_else(|| {
        usage(&program);

        logging::error("no subcommands were provided");
        std::process::exit(1);
    });

    match subcommand.as_ref() {
        "new" => {
            match arguments.next() {
                Some(project_name) => {
                    if let Err(err) = source::setup_new_project(&project_name) {
                        logging::error(format!("{}", err).as_str());
                        std::process::exit(1);
                    }
                },
                None => {
                    logging::error("expected a project.");
                    std::process::exit(1);
                }
            }
        }
        "build" => {
            if !Path::new(&"config.toml").is_file() {
                logging::error("failed to find: `config.toml` file.");
                std::process::exit(1);
            }

            if !Path::new(PAGE_BUILD_DIR).exists() {
                create_dir_all(PAGE_BUILD_DIR)?;
            }

            let mut work_count = 0;

            let markdown_files = source::markdown_file_names()?;
            let mut content_cache = cache::CacheData::create_manager(markdown_files, CONTENT_CACHE)?;
            content_cache.process_data()?;
            content_cache.write_to_json()?;

            let html_file_template = fs::read_to_string(PAGE_TEMPLATE).unwrap_or_else(|_| {
                logging::error("the template `page.html` could not be found.");
                std::process::exit(1);
            });


            for change_file in content_cache.required_changes.into_inner() {
                let mut file_contents = fs::read_to_string(&change_file)?;
                let headerless_file_contents = markdown::remove_header(&change_file, &mut file_contents);

                let html_output = markdown::compile(&headerless_file_contents);
                let file_name = source::html_file_name(&change_file);
                work_count += 1;

                let mut output_templates = html_file_template.replace("{{ content }}", &html_output);
                
                let minify_html = minify_html_onepass::in_place_str(&mut output_templates, &minify_html_onepass::Cfg::new());

                logging::info(format!("converted {} -> {}", &change_file.display(), &file_name).as_str());
                write(format!("{PAGE_BUILD_DIR}/{file_name}"), minify_html.unwrap())?
            }

            for entry in WalkDir::new(TEMPLATES_DIR) {
                templates::template_engine(&entry?.into_path());
            }

            for asset in WalkDir::new("assets") {
                let handle = asset?.path().to_owned();
                let build_dir_path = format!("{BUILD_DIR}/{}", handle.to_string_lossy());

                if handle.is_dir() {
                    create_dir_all(&build_dir_path)?;
                }

                if handle.is_file() {
                    fs::copy(&handle, build_dir_path)?;
                }
            }

            match work_count {
                0 => {
                    logging::info("All files are already up to date.");
                }
                _ => {
                    logging::info(format!("{} files were affected.", &work_count).as_str());
                }
            }
        },
        "serve" => {
            unimplemented!()
        }
        _ => {
            usage(&program);
            logging::error(format!("{} not found.", subcommand).as_str());
            std::process::exit(1);
        }
    }

    Ok(())
}
