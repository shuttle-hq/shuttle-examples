//! Usage: rust-script _scripts/check-templates.rs
//!
//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! toml = "0.8"
//! ignore = "0.4"
//! ```

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Template {
    /// Title of the template
    title: String,
    /// A short description of the template
    description: Option<String>,
    /// Path relative to the repo root
    path: Option<String>,
    /// GitHub username of the main author of the template
    author: Option<String>,
    /// "starter" OR "template" OR "tutorial"
    r#type: TemplateType,
    /// List of areas where this template is useful. Examples: "Web app", "Discord bot", "Monitoring", "Automation", "Utility"
    use_cases: Vec<String>,
    /// List of keywords that describe the template. Examples: "axum", "serenity", "typescript", "saas", "fullstack", "database"
    tags: Vec<String>,
    /// URL to a live instance of the template (if relevant)
    live_demo: Option<String>,

    /// If this template is available in the `cargo shuttle init --template` short-hand options, add that name here
    template: Option<String>,

    /// Set this to true if this is a community template outside of the shuttle-examples repo
    community: Option<bool>,
    /// URL to the repo of the community template
    repo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TemplateType {
    Starter,
    Template,
    Tutorial,
}

#[derive(Debug, Serialize, Deserialize)]
struct Schema {
    templates: std::collections::HashMap<String, Template>,
}

fn main() {
    let s = std::fs::read_to_string("templates.toml").expect("to find file");
    let toml: Schema = toml::from_str(&s).expect("to parse toml file");

    let (tx, rx) = std::sync::mpsc::channel();

    let t = std::thread::spawn(move || {
        rx.into_iter().collect::<std::collections::BTreeSet<_>>()
    });

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
            let s = s.trim_start_matches("./")
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
    
    // println!("{:?}", manifests);

    for (name, t) in toml.templates {
        // println!("{}  {:?}", name, t);
        let path = t.path.unwrap_or_default();

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
