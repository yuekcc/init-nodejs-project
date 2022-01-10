use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    process,
};

use anyhow::Result;
use chrono::{Datelike, Local};
use handlebars::Handlebars;

gflags::define! {
    /// set the module in private
    -p, --private = false
}

gflags::define! {
    /// set module author
    -a, --author: &str
}

gflags::define! {
    /// show help
    -h, --help = false
}

gflags::define! {
    /// show version
    -v, --version = false
}

gflags::define! {
    /// set module name, and will create a dir
    -n, --name: &str
}

fn get_output(name: &str, dir: &Path) -> PathBuf {
    let mut buf = PathBuf::from(dir);
    buf.push(name);
    buf
}

fn main() -> Result<()> {
    let _ = gflags::parse();

    if HELP.flag {
        gflags::print_help_and_exit(0)
    }

    if VERSION.flag {
        println!("{}-{}", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"));
        process::exit(0);
    }

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

    let private_pkg = if PRIVATE.is_present() {
        PRIVATE.flag
    } else {
        false
    };

    let author = if AUTHOR.is_present() {
        AUTHOR.flag.to_string()
    } else {
        env::var("INIT_NODEJS_PROJECT_AUTHOR").unwrap_or_else(|_| "yuekcc".to_string())
    };

    let this_year = Local::now().year();

    let pwd = env::current_dir()?;
    let basename = pwd.file_name().unwrap().to_str().unwrap();
    let project_name = if NAME.is_present() {
        NAME.flag
    } else {
        basename
    };

    let mut output_dir = PathBuf::from(&pwd);
    if NAME.is_present() {
        output_dir.push(NAME.flag);
        fs::create_dir(output_dir.as_path())?;
    }

    let model = serde_json::json!({
        "author": author,
        "thisYear": this_year,
        "projectName": project_name,
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

        let output_path = get_output(name, &output_dir.as_path());
        fs::write(output_path, contents).expect(&format!("failed to create file, path: {}", name));
    });

    Ok(())
}
