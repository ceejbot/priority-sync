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
    /// Print out more information as the tool runs.
    #[clap(long, short, global = true)]
    verbose: bool,
    /// Print out only very important information.
    #[clap(long, short, global = true)]
    quiet: bool,
    /// Report what the tool would do, without saving files.
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

fn synchronize(args: &Args, source: &String, t: &String) -> Result<()> {
    let profile = Profile::new(source.as_str())?;
    profile.report();
    let mut target = Profile::new(t.as_str())?;
    target.report();
    log::info!(
        "{} ➜ {}",
        profile.name().bold().yellow(),
        target.name().bold().blue()
    );
    target.synchronize(&profile, false);

    if !args.dry_run {
        target.write()?;
    }

    Ok(())
}

fn sync_newest(args: &Args, profile_dir: &String) -> Result<()> {
    let mut profiles: Vec<Profile> = std::fs::read_dir(profile_dir)
        .into_diagnostic()?
        .flatten()
        .filter_map(|f| {
            if f.path().is_dir() {
                match Profile::from_path(f.path()) {
                    Ok(p) => Some(p),
                    Err(e) => {
                        log::warn!("Skipping profile {f:?}; error={e:?}");
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
        log::warn!("No profiles found!");
        return Ok(());
    };

    println!(
        "Newest profile: {}\n last modified: {}",
        source.name().bold().yellow(),
        source.modified_human().bold()
    );
    for mut target in profiles {
        log::info!("    syncing to {}", target.name().bold().blue());
        target.synchronize(&source, true);
        if !args.dry_run {
            target.write()?;
        }
    }

    Ok(())
}

fn report(_args: &Args, source: &String) -> Result<()> {
    let profile = Profile::new(source.as_str())?;
    profile.report();
    Ok(())
}

/// Process command-line options and act on them.
fn main() -> Result<()> {
    let args = Args::parse();
    let level = if args.verbose {
        // Debug-level logging should be designed for modders to read when they
        // are trying to debug problems.
        log::Level::Debug
    } else if args.quiet {
        // Error- and warn-level logging should be designed to inform modders about truly important
        // problems or results.
        log::Level::Warn
    } else {
        // Info-level logging should be designed for modders to read normally.
        log::Level::Info
    };

    loggerv::Logger::new()
        .max_level(level)
        .line_numbers(false)
        .module_path(false)
        .colors(true)
        .init()
        .unwrap();

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
