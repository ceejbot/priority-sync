//! Documentation comment here please.

#![deny(future_incompatible, clippy::unwrap_used)]
#![warn(rust_2018_idioms, trivial_casts)]

use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, Result};
use owo_colors::OwoColorize;

pub mod profile;
pub use profile::*;

#[derive(Parser, Debug)]
#[clap(name = "mcm-meta-helper", version)]
pub struct Args {
    /// Force a sync even if the source is older than the target.
    #[clap(long, short, global = true)]
    force: bool,
    /// Don't replace the modlist.txt file, but save the new file next to it.
    #[clap(long, short, global = true)]
    dry_run: bool,
    /// Wait until you press enter; for running as MO2 executable.
    #[clap(long, short, global = true)]
    wait: bool,
    /// What to do.
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Apply the mod order in the given profile to the target profiles.
    Sync {
        /// The source profile path.
        source: String,
        /// The profile to bring into sync with the source.
        target: String,
    },
    /// Synchronize all profiles with the most-recently-edited profile.
    SyncNewest {
        /// The directory containing all your profiles.
        profile_dir: String,
    },
    /// Give a pretty useless report on what's in an existing profile.
    Report {
        /// The path to the profile to report on.
        source: String,
    },
}

fn synchronize(args: &Args, s: &str, t: &str) -> Result<()> {
    let source = Profile::new(s)?;
    let mut target = Profile::new(t)?;
    println!("\nsource: {}", source.name().bold().yellow());
    source.report();
    println!("\ntarget: {}", target.name().bold().blue());
    target.report();
    println!();

    if !args.force && !args.dry_run && target.modified() > source.modified() {
        // Here we warn the user and halt rather than deleting mods from its list.
        // Plugins in MO2 avoid this problem because the active profiles always has
        // the full mod list.
        // The dry run is always safe, and the force option says do it anyway.
        println!("⚠️ The target profile is newer than the source. Not syncing!");
        println!(
            "Use the {} flag to perform the sync anyway.",
            "--force".bold().bright_magenta()
        );
        return Ok(());
    }

    target.synchronize(&source);
    target.write(args.dry_run)?;

    Ok(())
}

fn sync_newest(args: &Args, profile_dir: &str) -> Result<()> {
    let mut profiles: Vec<Profile> = std::fs::read_dir(profile_dir)
        .into_diagnostic()?
        .flatten()
        .filter_map(|f| {
            if f.path().is_dir() {
                match Profile::from_path(f.path()) {
                    Ok(p) => Some(p),
                    Err(e) => {
                        println!("Skipping profile {f:?}; error={e:?}");
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect();

    profiles.sort_by_key(|p| p.modified());
    let Some(source) = profiles.pop() else {
        println!("⚠️ No profiles found!");
        return Ok(());
    };

    println!(
        "Newest profile: {}\n last modified: {}",
        source.name().bold().yellow(),
        source.modified_human().bold()
    );
    for mut target in profiles {
        target.synchronize(&source);
        target.write(args.dry_run)?;
    }

    Ok(())
}

fn report(_args: &Args, source: &str) -> Result<()> {
    let profile = Profile::new(source)?;
    profile.report();
    Ok(())
}

/// Process command-line options and act on them.
fn main() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        Command::Sync {
            ref source,
            ref target,
        } => synchronize(&args, source, target)?,
        Command::SyncNewest { ref profile_dir } => sync_newest(&args, profile_dir)?,
        Command::Report { ref source } => report(&args, source)?,
    };

    if args.wait {
        let mut buf = String::new();
        println!("\nPress enter to quit...");
        std::io::stdin().read_line(&mut buf).into_diagnostic()?;
    }

    Ok(())
}
