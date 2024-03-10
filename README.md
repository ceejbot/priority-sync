# priority-sync

`priority-sync` is a command-line tool for synchronizing MO2 mod priority orders across several profiles at once. It can sync mod order from a single hand-chosen profile to any other profile, or sync from the newest to all your other profiles. The enabled/disabled state of mods is preserved if possible. The previous order is preserved in a backup file.

It is stupid, simple, and quite fast because of how stupid it is. I mean, seriously, the sync algorithm is a vector clone. I don't think it can get any stupider.

## Usage

You can run this from the command-line or as an executable in MO2. `priority-sync help` gives full command help.

To sync one profile from the command line, you would run something like:

```text
priority-sync sync g:\SkyrimProfiles\NiceUpdatedProfile g:\SkyrimProfiles\OldUnsortedProfile
```

The `sync` subcommand applies the priority order of NiceUpdatedProfile to OldUnsortedProfile.

For usage in MO2, you'll want the `sync-newest` subcommand. This applies the mod order in the _most recently changed_ profile to all of the profiles found in the given directory. The current profile is extremely likely to be the profile you've edited most recently. This is the profile most likely to have all your current plugins. The full command line for this will look like:

```text
priority-sync sync-newest /path/to/MO2/profiles
```

For any command that touches a `modlist.txt` file, the older file is saved as a backup using the MO2 backup file name pattern.

## Building

This is a Rust language project. Install the Rust tools with [rustup](https://rustup.rs), then run `cargo build` to build.

## TODO

Make reporting useful. Make dry run report how many changes it would make, or maybe just save the potential new modlist.txt file as a proposed-modlist.txt file. Use miette's error reporting to its full advantage.

See the `TODO` in profile.rs for a potential high-cost feature that would prevent out-of-band renames from causing mod enabled status from being lost on sync.

## License

[The Parity Public License.](https://paritylicense.com) This license requires people who build on top of this source code to share their work with the community, too. In Skyrim modding language, this license allows "cathedral" modding, not "parlor" modding. Please see the text of the license for details.
