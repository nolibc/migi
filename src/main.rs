use std::env;
use anyhow::Result;
use walkdir::WalkDir;

mod cache;
mod default;
mod markdown;
mod source;
mod templates;
mod logging;

pub const BUILD_DIR: &str = "build/";
pub const PAGE_BUILD_DIR: &str = "build/page/";
pub const TEMPLATES_DIR: &str = "templates/";
pub const PAGE_TEMPLATE: &str = "templates/page.html";
pub const CONTENT_CACHE: &str = "cache/content.json";
pub const CONTENT: &str = "content/";
pub const ASSETS: &str = "assets/";

fn usage(program: &str) {
    eprintln!(
"Usage: {program} [command]
An extremely minimal static site generator.

Commands:
    new <directory>     create new project directory
    build               build project outputting html
    version             show the version of migi\n");
}

fn main() -> Result<()> {
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
            source::prechecks()?;
            let content_cache = source::scan_cache()?;
            let work_count = source::markdown_to_html_export(content_cache)?;
            for entry in WalkDir::new(TEMPLATES_DIR) {
                templates::template_engine(&entry?.into_path());
            }
            source::copy_assets(ASSETS)?;
            match work_count {
                0 => {
                    logging::info("All files are already up to date.");
                }
                _ => {
                    logging::info(format!("{} files were affected.", work_count).as_str());
                }
            }
        }
        "version" => {
            println!("migi v0.1.2")
        }
        _ => {
            usage(&program);
            logging::error(format!("{} not found.", subcommand).as_str());
            std::process::exit(1);
        }
    }
    Ok(())
}
