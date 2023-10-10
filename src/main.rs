use std::{
    env,
    fs::{self, create_dir_all, write},
    io,
    path::Path, process::Command,
};

mod cache;
mod default;
mod markdown;
mod source;
mod templates;

pub const BUILD_DIR: &str = "build/page";
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
        eprintln!("ERROR: no subcommands were provided");
        std::process::exit(1);
    });

    match subcommand.as_ref() {
        "new" => {
            match arguments.next() {
                Some(project_name) => {
                    if let Err(err) = source::setup_new_project(&project_name) {
                        println!("ERROR: {}", err);
                        std::process::exit(1);
                    }
                },
                None => {
                    println!("ERROR: expected a project.");
                    std::process::exit(1);
                }
            }
        }
        "build" => {
            if !Path::new(&"config.toml").is_file() {
                eprintln!("ERROR: failed to find: `config.toml` file.\n");
                std::process::exit(1);
            }

            if !Path::new(BUILD_DIR).exists() {
                create_dir_all(BUILD_DIR)?;
            }

            let mut work_count = 0;

            let markdown_files = source::markdown_file_names()?;
            let mut content_cache = cache::CacheData::create_manager(markdown_files, CONTENT_CACHE)?;
            content_cache.process_data()?;
            content_cache.write_to_json()?;

            let html_file_template = fs::read_to_string(PAGE_TEMPLATE).unwrap_or_else(|_| {
                eprintln!("ERROR: the template `page.html` could not be found.");
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

                println!("{} -> {}", &change_file.display(), &file_name);
                write(format!("{BUILD_DIR}/{file_name}"), minify_html.unwrap())?
            }

            for template in templates::file_names()? {
                templates::template_engine(&template);
            }

            // TODO: this is a horrible solution and assumes only Unix..
            Command::new("cp")
                .arg("-r")
                .arg("assets")
                .arg("build")
                .output()?;

            match work_count {
                0 => {
                    println!("All files are already up to date.");
                }
                _ => {
                    println!("{} files were affected.", &work_count);
                }
            }
        }
        _ => {
            usage(&program);
            eprintln!("\nERROR: {} not found.", subcommand);
            std::process::exit(1);
        }
    }

    Ok(())
}
