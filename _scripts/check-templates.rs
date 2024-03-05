//! Usage: rust-script _scripts/check-templates.rs
//!
//! ```cargo
//! [dependencies]
//! shuttle-common = "*" # always use latest version
//! toml = "0.8"
//! ignore = "0.4"
//! ```

use shuttle_common::templates::TemplatesSchema;

fn main() {
    let s = std::fs::read_to_string("templates.toml").expect("to find file");
    let toml: TemplatesSchema = toml::from_str(&s).expect("to parse toml file");

    let (tx, rx) = std::sync::mpsc::channel::<String>();

    let t = std::thread::spawn(move || rx.into_iter().collect::<std::collections::BTreeSet<_>>());

    let walker = ignore::WalkBuilder::new(".").build_parallel();
    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            use ignore::WalkState::*;
            // join directory with filename so that this directory can be skipped in the case of a workspace
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
    let mut manifests = t.join().unwrap();

    let mut set = std::collections::BTreeSet::<_>::new();
    for (name, t) in toml.templates {
        if t.community.is_some_and(|c| c) {
            continue;
        }

        let path = t.path.unwrap_or_default();

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
    }

    if !manifests.is_empty() {
        println!("Missing template definitions:");
        for s in manifests {
            println!("{s}");
        }
        std::process::exit(1);
    }

    println!("Template definitions verified")
}
