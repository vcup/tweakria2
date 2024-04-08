mod git_op;
mod dirs;

fn main() -> anyhow::Result<()> {
    let files = git_op::pickup_files_from_repo("https://github.com/aria2/aria2.git", &[
        "src/usage_text.h",
    ])?;
    println!("{}", files[0]);

    Ok(())
}
