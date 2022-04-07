use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct List {
    directories: Vec<String>,
    files: Vec<String>,
}

fn get_list(version: &str, path: &str) -> reqwest::Result<List> {
    let url = format!(
        "https://github.com/InventivetalentDev/minecraft-assets/raw/{}/{}/_list.json",
        version, path
    );
    reqwest::blocking::get(url)?.json::<List>()
}

fn get_list_all(version: &str, path: &str) -> Result<Vec<String>, reqwest::Error> {
    let mut files = vec![];
    let mut directories = vec![path.to_string()];
    while !directories.is_empty() {
        let parent_path = directories.pop().unwrap();
        let list = get_list(version, parent_path.as_str())?;
        for file in list.files {
            files.push(format!("{}/{}", parent_path, file));
        }
        for directory in list.directories {
            directories.push(format!("{}/{}", parent_path, directory));
        }
    }
    Ok(files)
}

pub fn get_assets(
    version: &str,
    namespace: &str,
    path: &str,
    extension: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let path = vec!["assets", namespace, path].join("/");
    let path_len = path.len() + 1; // +1 to include '/' after the path
    let extension_len = extension.len();
    let assets = get_list_all(version, path.as_str())?
        .iter()
        .map(|s| {
            let mut indices = s.char_indices();
            let begin = indices.nth(path_len).unwrap().0;
            let end = indices.nth_back(extension_len).unwrap().0;
            format!("{}:{}", namespace, &s[begin..end])
        })
        .collect();
    Ok(assets)
}
