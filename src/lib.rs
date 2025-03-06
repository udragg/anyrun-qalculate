use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use serde::Deserialize;
use std::{
    fs,
    process::{Command, Stdio},
};

#[derive(Deserialize)]
struct Config {
    prefix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { prefix: "=".into() }
    }
}

#[init]
fn init(config_dir: RString) -> Config {
    match fs::read_to_string(format!("{config_dir}/qalculate.ron")) {
        Ok(content) => ron::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Qalc".into(),
        icon: "accessories-calculator".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, config: &Config) -> RVec<Match> {
    if let Some(query) = input.strip_prefix(&config.prefix) {
        // TODO calling twice with the same query is stupid
        // look for a way to get both full and terse output from a single call
        let qalc_output = Command::new("qalc")
            .arg(query)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .output()
            .expect("failed to run qalc");
        let qalc_result = RString::from_utf8(qalc_output.stdout).unwrap();
        let qalc_output_terse = Command::new("qalc")
            // .arg("-c")
            .arg("-t")
            .arg(query)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .output()
            .expect("failed to run qalc");
        let qalc_result_terse: RString = String::from_utf8(qalc_output_terse.stdout)
            .unwrap()
            // .escape_default()
            // .to_string()
            .into();
        vec![Match {
            title: RString::from(qalc_result_terse.trim()),
            description: ROption::RSome(qalc_result.trim().into()),
            use_pango: false,
            icon: ROption::RSome("accessories-calculator".into()),
            id: ROption::RNone,
        }]
        .into()
    } else {
        RVec::new()
    }
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    HandleResult::Copy(selection.title.into_bytes())
}
