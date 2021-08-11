use eyre::Result;
use pr_bump_lib::get_next_version;
use pr_bump_lib::GitHub;

// TODO:
// - Setup configuration (with serde_json and a config struct)
//   - Allow choosing which files to update the version number in
//   - Allow choosing which labels correspond to what version bump (e.g. link `fix` to `patch` and `breaking` to `major`)
//   - Allow choosing which base branch a PR needs to point to for it to be picked up
// - Get the repo and owner automatically (github actions env vars?)
// - Add docs to relevant public functions
// - Make sure the repo defines a valid action (https://docs.github.com/en/actions/creating-actions/creating-a-docker-container-action)
//   - Create the config file, the inputs, etc.
// - Make the README up to stuff, list all the inputs and outputs
//   - Give examples of how to integrate with the changelog builder (https://github.com/marketplace/actions/release-changelog-builder)
//   - Give examples of how to use stand-alone
// - (For the function to update the version in files), use any `Write` instead of a path to allow in-memory testing, or maybe just take a string slice and return a modified String

#[tokio::main]
async fn main() -> Result<()> {
    let github = GitHub::new("marier-nico", "venv-wrapper");

    let next_version = get_next_version(github).await?;

    println!("Next version: {}", next_version);
    /*if latest_releasse.get_version()? != next_version {
        println!("New version: {}", next_version);
        bump_version_in_file(
            "/home/nmarier/Documents/Software/Projects/venv-wrapper/Cargo.toml",
            "version = \"",
            &latest_releasse.get_version()?,
            &next_version,
        )?;
    }*/
    
    Ok(())
}
