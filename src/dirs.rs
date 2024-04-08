use std::path::PathBuf;

pub fn cache_dir() -> Option<PathBuf> {
    match dirs::cache_dir() {
        None => { None }
        Some(mut path) => {
            path.push("tweakria2");
            Some(path)
        }
    }
}