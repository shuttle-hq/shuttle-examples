//! Usage: rust-script _scripts/check-templates.rs
//!
//! ```cargo
//! [dependencies]
//! shuttle-common = "*" # always use latest version
//! # for local use
//! # shuttle-common = { path = "../../common" }
//! toml = "0.9"
//! ignore = "0.4"
//! ```

use shuttle_common::templates::TemplatesSchema;

fn main() {
    let s = std::fs::read_to_string("templates.toml").expect("to find file");
    let schema: TemplatesSchema = toml::from_str(&s).expect("to parse toml file");

    let print_template_repos = std::env::args().any(|a| a == "--list-template-repos");

    let (tx, rx) = std::sync::mpsc::channel::<String>();

    let thread =
        std::thread::spawn(move || rx.into_iter().collect::<std::collections::BTreeSet<_>>());

    let walker = ignore::WalkBuilder::new(".").build_parallel();
    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            use ignore::WalkState::*;
            // Join directory with filename so that this directory can
            // be skipped in the case of a workspace manifest.
            let path = result.unwrap().into_path().join("Cargo.toml");
            if !path.exists() {
                return Continue;
            }

            let s = format!("{}", path.display());
            let s = s
                .trim_start_matches("./")
                .trim_end_matches("/Cargo.toml")
                .to_owned();
            tx.send(s).unwrap();

            if std::fs::read_to_string(&path)
                .unwrap()
                .contains("[workspace]")
            {
                // don't walk into subdirectories of workspaces
                return Skip;
            }

            Continue
        })
    });
    drop(tx);
    let mut manifests = thread.join().unwrap();

    let mut set = std::collections::BTreeSet::<_>::new();
    // don't validate community templates
    for (name, t) in schema
        .starters
        .into_iter()
        .chain(schema.templates.into_iter())
        .chain(schema.examples.into_iter())
        .chain(schema.tutorials.into_iter())
    {
        const MAX_LENGTH: usize = 100;
        let len = t.title.chars().count() + t.description.chars().count();
        if len > MAX_LENGTH {
            eprintln!(
                "The title + description of '{}' is too long (length {}; max {})",
                name, len, MAX_LENGTH
            );
            std::process::exit(1);
        }

        let path = t.path.expect("template to have a path field");

        if !set.insert(path.clone()) {
            eprintln!("Path '{}' referenced in two places", path);
            std::process::exit(1);
        }

        if !manifests.remove(&path) {
            eprintln!(
                "Template {} referenced non-existent manifest file at {}",
                name, path
            );
            std::process::exit(1);
        }

        if print_template_repos && t.has_template_repo.is_some_and(|t| t) {
            println!("{}|{}", name, path);
        }
    }

    if !manifests.is_empty() {
        println!("Missing template definitions:");
        for s in manifests {
            println!("{s}");
        }
        std::process::exit(1);
    }

    eprintln!("Template definitions verified ✅")
}
