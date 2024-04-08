use std::path::Path;

pub fn pickup_files_from_repo(url: &str, files: &[&str]) -> anyhow::Result<Vec<String>> {
    let mut cache_dir = crate::dirs::cache_dir().unwrap();
    cache_dir.push("aria2");

    let repo = match git2::Repository::open(&cache_dir) {
        Ok(repo) => { repo }
        Err(_) => {
            let mut builder = git2::build::RepoBuilder::new();
            builder.fetch_options({
                let mut opts = git2::FetchOptions::new();
                opts.depth(1);
                opts
            });
            builder.bare(true);
            builder.clone(url, cache_dir.as_path())?
        }
    };
    let tree = repo.head()?.peel_to_tree()?;

    let mut result = Vec::with_capacity(files.len());
    for path in files {
        let obj = tree.get_path(Path::new(path))?
            .to_object(&repo)?;
        let blob = obj.as_blob()
            .ok_or(git2::Error::from_str("Failed to read blob of file"))?;
        let content = String::from_utf8(blob.content().into())?;
        result.push(content);
    }

    Ok(result)
}