//! Usage: rust-script _scripts/check-templates.rs
//!
//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! toml = "0.8"
//! ignore = "0.4"
//! ```
use ignore::Walk;
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

    let mut manifests = Walk::new(".")
        .into_iter()
        .filter_map(|entry| {
            let path = entry.unwrap().into_path();
            if path.file_name().map(|s| s.to_str().unwrap()) != Some("Cargo.toml") {
                return None;
            }
            if std::fs::read_to_string(&path)
                .unwrap()
                .contains("[workspace]")
            {
                println!("workspace {}", path.display());
            }

            let s = format!("{}", path.display());
            Some(
                s.trim_start_matches("./")
                    .trim_end_matches("/Cargo.toml")
                    .to_owned(),
            )
        })
        .collect::<std::collections::BTreeSet<_>>();
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

    println!("Templates definitions verified")
}
