use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use handlebars::Handlebars;
use time;

#[derive(Parser)]
#[clap(version = env!("GIT_HASH"))]
struct Cli {
    /// Set project name (and create project folder).
    /// If not parent, use working dir name as project name.
    #[clap()]
    pub name: Option<String>,

    /// Set project as PRIVATE
    #[clap(short = 'p', long = "private")]
    pub is_private: bool,

    /// Set author name
    #[clap(short = 'a', default_value = "no_name")]
    pub author: String,
}

fn mk_output_path(name: &str, dir: &Path) -> PathBuf {
    let mut buf = PathBuf::from(dir);
    buf.push(name);
    buf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut tpl_registry = Handlebars::new();
    let mut tpl_table = HashMap::new();
    tpl_table.insert(
        ".editorconfig",
        include_str!("./templates/.editorconfig.hbs"),
    );
    tpl_table.insert(".gitignore", include_str!("./templates/.gitignore.hbs"));
    tpl_table.insert(
        "jsconfig.json",
        include_str!("./templates/jsconfig.json.hbs"),
    );
    tpl_table.insert("LICENSE", include_str!("./templates/LICENSE.hbs"));
    tpl_table.insert("package.json", include_str!("./templates/package.json.hbs"));

    tpl_table.iter().for_each(|(name, tpl)| {
        tpl_registry
            .register_template_string(name, tpl)
            .expect(&format!("parse template error of {}", name));
    });

    // LICENSE 声明年份
    let this_year = time::OffsetDateTime::now_utc().year();

    // 项目名
    let pwd = env::current_dir()?;
    let basename = pwd.file_name().unwrap().to_str().unwrap().to_string();

    let mut should_create_project_dir = false;
    let project_name = if cli.name.is_some() {
        should_create_project_dir = true;
        cli.name.unwrap()
    } else {
        basename
    };

    let mut output_dir = PathBuf::from(&pwd);
    if should_create_project_dir {
        output_dir.push(project_name.clone());
        fs::create_dir(output_dir.as_path())?;
    }

    // 模板数据
    let model = serde_json::json!({
        "author": cli.author.clone(),
        "private": cli.is_private,
        "projectName": project_name.clone(),
        "thisYear": this_year,
    });

    tpl_table.into_keys().for_each(|name| {
        if cli.is_private && name == "LICENSE" {
            return;
        }

        let contents = tpl_registry
            .render(name, &model)
            .expect(&format!("failed to render data for template '{}'", name));

        let output_path = mk_output_path(name, &output_dir.as_path());
        fs::write(output_path, contents).expect(&format!("failed to create file, path: {}", name));
    });

    Ok(())
}
