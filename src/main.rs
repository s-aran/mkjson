use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};

use clap::Parser;
use libmll::Mll;

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    template_path: PathBuf,
    values_path: PathBuf,
    #[arg(short, long, default_value = "false")]
    pretty: bool,
}

fn main() {
    let args = Args::parse();

    if !args.template_path.exists() {
        panic!("Template file does not exist");
    }

    if !args.values_path.exists() {
        panic!("Values file does not exist");
    }

    let template = std::fs::read_to_string(args.template_path.clone()).unwrap();
    let values = std::fs::read_to_string(args.values_path.clone()).unwrap();

    let mut mll = Mll::new();
    mll.set_template(template);
    // mll.set_pre_process_script(values);

    let rendered = match mll.render_with_lua(&values) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return ();
        }
    };

    let result = match args.pretty {
        true => {
            let obj: serde_json::Value = serde_json::from_str(&rendered).unwrap();
            serde_json::to_string_pretty(&obj).unwrap()
        }
        _ => rendered,
    };

    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let _ = out.write_all(result.as_bytes());
    let _ = out.flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let template = r#"
{
    "name": "{{name}}",
    "age": {{age}}
}
        "#;
        let values = r#"
name = "hoge"
age = 21
    "#;

        let mut mll = Mll::new();
        mll.set_template(template.to_string());
        // mll.set_pre_process_script(values);

        let actual = mll.render_with_lua(&values.to_string()).unwrap();

        let expected = r#"
{
    "name": "hoge",
    "age": 21
}
        "#;

        assert_eq!(expected, actual);
    }
}
