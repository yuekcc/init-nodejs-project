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
    /// 设置为私有模块
    -p, --private = false
}

gflags::define! {
    /// 设置模块名称，并创建同名目录
    -n, --name: &str
}

gflags::define! {
    /// 设置模块作者
    -a, --author: &str
}

gflags::define! {
    /// 显示帮助信息
    -h, --help = false
}

gflags::define! {
    /// 显示版本号
    -v, --version = false
}

fn mk_output_path(name: &str, dir: &Path) -> PathBuf {
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

    let mut tpl_registry = Handlebars::new();
    let mut tpl_table = HashMap::new();
    tpl_table.insert("package.json", include_str!("./templates/package.json"));
    tpl_table.insert("LICENSE", include_str!("./templates/LICENSE"));
    tpl_table.insert(".editorconfig", include_str!("./templates/.editorconfig"));
    tpl_table.insert(".gitignore", include_str!("./templates/.gitignore"));

    tpl_table.iter().for_each(|(name, tpl)| {
        tpl_registry
            .register_template_string(name, tpl)
            .expect(&format!("parse template error of {}", name));
    });

    // 是否私有模块
    let is_private_pkg = if PRIVATE.is_present() {
        PRIVATE.flag
    } else {
        false
    };

    // 作者
    let author = if AUTHOR.is_present() {
        AUTHOR.flag.to_string()
    } else {
        env::var("INP_AUTHOR").unwrap_or_else(|_| "no_name".to_string())
    };

    // LICENSE 声明年份
    let this_year = Local::now().year();

    // 项目名
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

    // 模板数据
    let model = serde_json::json!({
        "author": author,
        "nonPrivate": !is_private_pkg,
        "private": is_private_pkg,
        "projectName": project_name,
        "thisYear": this_year,
    });

    tpl_table.into_keys().for_each(|name| {
        if is_private_pkg && name == "LICENSE" {
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
