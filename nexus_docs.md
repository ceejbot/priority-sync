`priority-sync` is a command-line tool for synchronizing MO2 mod priority orders across several profiles at once. It can sync mod order from a single hand-chosen profile to any other profile, or sync from the newest to all your other profiles. The enabled/disabled state of mods is preserved if possible. The previous order is preserved in a backup file.

It is stupid, simple, and quite fast because of how stupid it is. I mean, seriously, the sync algorithm is a vector clone. I don't think it can get any stupider.

## Usage

You can run this from the command-line or as an executable in MO2. `priority-sync help` gives full command help.

The `sync` subcommand applies the priority order of NiceUpdatedProfile to OldUnsortedProfile.
To sync one profile from the command line, you would run something like:

```text
priority-sync sync g:\SkyrimProfiles\NiceUpdatedProfile g:\SkyrimProfiles\OldUnsortedProfile
```

The command you probably want to use routinely is `sync-newest`, like this:

```text
priority-sync sync-newest /path/to/MO2/profiles
```

This applies the mod order in the _most recently changed_ profile to all the other profiles found in the given directory. The current profile is extremely likely to be the profile you've edited most recently. This is the profile most likely to have all your current plugins. The full command line for this will look like:

For usage in MO2, you'll want the `sync-newest` subcommand with the `--wait` option to show you what the tool did. The arguments for the executable will look like: `sync-newest /path/to/MO2/profiles --wait`.

For any command that touches a `modlist.txt` file, the older file is saved as a backup using the same file name pattern that MO2 uses when it saves a backup of your profile. The file swap is done in two steps: the old file is copied into the backup, then the new file is copied into place. It's still possible that something might go wrong, so make your own profile backups as well!

## Infrequently-asked questions

**Q:** Why isn't this an MO2 plugin?

**A:** I didn't want to write it in Python and I was too lazy to write Rust bindings to the C++ plugin interface for MO2 before I got started. [Even the White House](https://www.whitehouse.gov/oncd/briefing-room/2024/02/26/press-release-technical-report/) doesn't want people to write new things in C++ and who am I to argue with the POTUS?

**Q:** It's kinda annoying as an executable.

**A:** Run it from the command-line. I keep a terminal window or ten open at all times, like a normal person. That's normal, right?

**Q:** It messed up and disabled a mod that I had renamed in an explorer window.

**A:** Yeah, this is a weakness of the approach. I know how I would fix it. It requires maintaining a little db to map mod info in the `meta.ini` file with folder names, and back again, so I can detect renames that happen out of band of MO2. If I get energized, I'll do this.

## Alternatives

[Kezyma's Profile Sync for Mod Organizer](https://www.nexusmods.com/skyrimspecialedition/mods/60690) is a proper MO2 plugin that tries to keep groups of plugins in sync with each other. This is a _lot_ more convenient to use than an MO2 executable, but I found that it stops working periodically. And because it's in Python, it's slower than this tool.

## Permissions and credits

[Source on GitHub](https://github.com/ceejbot/priority-sync). This is pretty simple Rust project.

This mod is released under [the Parity Public License](https://paritylicense.com). This license requires people who build on top of this source code to share their work with the community, too. In Skyrim modding language, this license allows "cathedral" modding, not "parlor" modding. Please see the text of the license for details.

Translation: Permissions are open, so long as your fork also has open permissions.
