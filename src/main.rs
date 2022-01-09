use std::{collections::HashMap, env, fs};

use anyhow::Result;
use chrono::{Datelike, Local};
use handlebars::Handlebars;

fn main() -> Result<()> {
    let mut reg = Handlebars::new();

    let mut templates = HashMap::new();
    templates.insert("package.json", include_str!("./templates/package.json"));
    templates.insert("LICENSE", include_str!("./templates/LICENSE"));
    templates.insert(".editorconfig", include_str!("./templates/.editorconfig"));
    templates.insert(".gitignore", include_str!("./templates/.gitignore"));

    templates.iter().for_each(|(name, tpl)| {
        reg.register_template_string(name, tpl)
            .expect(&format!("parse template error of {}", name));
    });

    let pwd = env::current_dir()?;
    let basename = pwd.file_name().unwrap().to_str().unwrap();
    let private_pkg = false;
    let this_year = Local::now().year();
    let author = env::var("INIT_NODEJS_PROJECT_AUTHOR").unwrap_or_else(|_| "yuekcc".to_string());

    let model = serde_json::json!({
        "author": author,
        "thisYear": this_year,
        "projectName": basename,
        "private": private_pkg,
        "nonPrivate": !private_pkg,
    });

    templates.into_keys().for_each(|name| {
        if private_pkg && name == "LICENSE" {
            return;
        }

        let contents = reg
            .render(name, &model)
            .expect(&format!("failed to render data for template '{}'", name));

        fs::write(name, contents).expect(&format!("failed to create file, path: {}", name));
    });

    Ok(())
}
