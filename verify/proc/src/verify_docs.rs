use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use serde_json::Value as JValue;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};
use toml::Value as TValue;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum VerificationStatus {
    LibraryAllAc,
    LibraryPartialAc,
    LibrarySomeWa,
    LibraryAllWa,
    LibraryNoTests,
    TestAccepted,
    TestWrongAnswer,
    TestWaitingJudge,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct VerifyStat {
    path: PathBuf,
    code: String,
    bundled_code: String,
    is_verification_file: bool,
    verification_status: String,
    timestamp: String,
    depends_on: Vec<PathBuf>,
    required_by: Vec<PathBuf>,
    verified_with: Vec<PathBuf>,
    attributes: HashMap<String, JValue>,
}

pub fn main() -> TokenStream {
    eprintln!("{:?}", std::env::current_dir());
    let stats = get_stats(".verify-helper/cache/stats.json").unwrap();
    let res = stats.into_iter().filter_map(|stat| {
        if stat.is_verification_file {
            stat_to_doc(&stat).into()
        } else {
            None
        }
    });
    quote! {
        #( #res )*
    }
    .into()
}

fn get_stats<P: AsRef<Path>>(path: P) -> Result<Vec<VerifyStat>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

fn stat_to_doc(stat: &VerifyStat) -> TokenStream2 {
    let problem = stat.attributes["PROBLEM"].as_str().unwrap().to_string();
    let problem = format!("[{0}]({0})", problem);
    let name = format_ident!(
        "{}",
        stat.path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_string_lossy()
    );
    let code = format!("# code\n```no_run\n{ }\n```", stat.code);
    let depends = stat
        .depends_on
        .iter()
        .filter_map(|depend| {
            if depend.starts_with("verify") {
                return None;
            }
            path_to_name(depend.clone()).ok()
        })
        .collect::<HashSet<_>>();
    let depends = depends.into_iter()
        .map(|depend| {
            let link = format!("- [{depend}]");
            quote! {
                #[doc = #link]
            }
        })
        .collect::<Vec<_>>();
    quote! {
        #[doc = "# problem"]
        #[doc = #problem]
        #[doc = "# depends"]
        #( #depends )*
        #[doc = #code]
        pub mod #name {}
    }
}

fn path_to_name(mut path: PathBuf) -> Result<String, Box<dyn Error>> {
    path.pop();
    path.pop();
    path.push("Cargo.toml");
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut toml = String::new();
    reader.read_to_string(&mut toml)?;
    let toml: TValue = toml::from_str(&toml)?;
    Ok(toml["package"]["name"]
        .as_str()
        .ok_or("bat Cargo.toml")?
        .to_string()
        .replace('-', "_"))
}
