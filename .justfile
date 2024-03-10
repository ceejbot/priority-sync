# help
@help:
	just --list

# copy fixtures
@fixtures:
	cp -p /Volumes/SkyrimProfiles/2021-04\ simonrim/modlist.txt fixtures/profile_months
	cp -p /Volumes/SkyrimProfiles/2021-04\ simonrim/modlist.txt.2022_02_13_11_22_35 fixtures/profile_2years/modlist.txt
	cp -p /Volumes/SkyrimProfiles/2023-07\ 1.6.640\ mod\ dev/modlist.txt fixtures/profile_week
	cp -p /Volumes/SkyrimProfiles/2023-12\ Minimal\ 1.5.97/modlist.txt fixtures/profile_engine
	cp -p /Volumes/SkyrimProfiles/2024-01\ Smol\ List\ 640/modlist.txt fixtures/profile_newest
	chmod a-x fixtures/*/modlist.txt

# Test sync-newest
@sync-newest:
	cargo run -- sync-newest --dry-run ./fixtures

# Test sync warning
@warning:
	cargo run -- sync ./fixtures/profile_months ./fixtures/profile_newest

# Test sync single
@sync:
	cargo run -- sync --dry-run ./fixtures/profile_newest ./fixtures/profile_week

# Set the crate version and tag the repo to match. Requires bash.
[unix]
tag VERSION:
    #!/usr/bin/env bash
    set -e
    tomato set package.version {{VERSION}} Cargo.toml
    cargo check
    git commit Cargo.toml Cargo.lock -m "{{VERSION}}"
    git tag "{{VERSION}}"
    echo "Release tagged for version {{VERSION}}"

# Build a mod archive for the Nexus.
[unix]
archive:
    #!/usr/bin/env bash
    set -e
    version=$(tomato get package.version Cargo.toml)
    toolname=$(tomato get package.name Cargo.toml)
    release_name="${toolname}_v${version}"
    rm -f "$release_name".7z
    cd target/release
    7z a "$release_name".7z "$toolname".{exe,pdb}
    cd ../..
    mv target/release/"$release_name".7z .
    echo "Mod archive for v${version} ready at ${release_name}.7z"

# Remind you to run this in WSL.
[windows]
@archive:
	write-host "You need to run this in WSL to get bash."

# Remind you to run this in WSL.
[windows]
@tag:
	write-host "You need to run this in WSL to get bash."
