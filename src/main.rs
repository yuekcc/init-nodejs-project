use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    process,
};

use handlebars::Handlebars;

const HELP: &str = "

USAGE:
  init-nodejs-project [OPTIONS] [PROJECT_NAME]

OPTIONS:
  -h, --help            Prints help information
  -p, --private         Set project as PRIVATE
  -a, --author NAME     Set author name
      --vue             Setup Vue and Vite

ARGS:
  [PROJECT_NAME]        Set project name (and create project folder). 
                        If not parent, use working dir name as project name.
";

struct Cli {
    name: Option<String>,
    is_private: bool,
    author: String,
    with_vue: bool,
}

fn mk_output_path(name: &str, dir: &Path) -> PathBuf {
    let mut buf = PathBuf::from(dir);
    buf.push(name);
    buf
}

fn print_version() {
    print!("init-nodejs-project {}", env!("GIT_HASH"));
}

fn print_help() {
    print!("{}", HELP);
}

fn parse_cli() -> Result<Cli, Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        print_version();
        print_help();
        process::exit(0);
    }

    if args.contains(["-v", "--version"]) {
        print_version();
        process::exit(0);
    }

    let mut cli = Cli {
        name: None,
        is_private: args.contains(["-p", "--private"]),
        with_vue: args.contains("--vue"),
        author: args
            .value_from_str(["-a", "--author"])
            .unwrap_or_else(|_| "no_name".to_string()),
    };

    let remaining = args.finish();
    if !remaining.is_empty() {
        cli.name = Some(remaining[0].to_str().unwrap().to_string())
    }

    Ok(cli)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = parse_cli()?;

    let mut tpl_registry = Handlebars::new();
    let mut tpl_table = HashMap::new();
    tpl_table.insert(".npmrc", include_str!("./templates/.npmrc.hbs"));
    tpl_table.insert(".editorconfig", include_str!("./templates/.editorconfig.hbs"));
    tpl_table.insert(".gitignore", include_str!("./templates/.gitignore.hbs"));
    tpl_table.insert("jsconfig.json", include_str!("./templates/jsconfig.json.hbs"));
    tpl_table.insert("LICENSE", include_str!("./templates/LICENSE.hbs"));
    tpl_table.insert("package.json", include_str!("./templates/package.json.hbs"));
    tpl_table.insert("vite.config.js", include_str!("./templates/vite.config.js.hbs"));

    tpl_table.iter().for_each(|(name, tpl)| {
        tpl_registry
            .register_template_string(name, tpl)
            .unwrap_or_else(|_| panic!("parse template error of {}", name));
    });

    // LICENSE 声明年份
    let this_year = time::OffsetDateTime::now_local()?.year();

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
        "author": cli.author,
        "private": cli.is_private,
        "withVue": cli.with_vue,
        "projectName": project_name,
        "thisYear": this_year,
    });

    tpl_table.into_keys().for_each(|name| {
        if cli.is_private && name == "LICENSE" {
            return;
        }

        if !cli.with_vue && name == "vite.config.js" {
            return;
        }

        let contents = tpl_registry
            .render(name, &model)
            .unwrap_or_else(|_| panic!("failed to render data for template '{}'", name));

        let output_path = mk_output_path(name, output_dir.as_path());
        fs::write(output_path, contents).unwrap_or_else(|_| panic!("failed to create file, path: {}", name));
    });

    Ok(())
}
