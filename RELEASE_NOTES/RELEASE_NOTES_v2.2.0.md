# XIMOD Architect 2.2.0

XIMOD Architect builds FOMOD installers for Bethesda mods (Skyrim, Fallout,
Starfield, Oblivion, Morrowind) on Windows, Linux and macOS. Version 2.2.0
makes it easier to reuse installer pieces across mods and to keep file
references right when files are renamed or moved — ideas from Wenderer, author
of the original FOMOD Creation Tool, and from one of its users.

## Highlights

**Cut, copy and paste between FOMODs.** Steps, groups and options can be cut,
copied and pasted (Edit menu, right-click in the tree, or Ctrl+X / Ctrl+C /
Ctrl+V). The clipboard is shared by all open FOMOD tabs and even by two XIMOD
windows. The pasted node lands next to the selection, as one undoable edit, and
XIMOD tells you when it uses files that are not in this mod's folder or tests
flags that nothing in this project sets.

**Rename on disk.** In the files tables, Rename on disk… renames the selected
file or folder in the mod folder and updates every reference to it — for a
folder, everything inside it too.

**Find missing files.** Project → Find missing files… lists everything the
project uses that no longer exists and proposes where it went: the same name
elsewhere (moved) or a similar name in the same folder (renamed). Pick the
right one, or browse, then apply all the fixes in one go.

## Notes

- Projects and FOMODs saved by 2.1.0 open unchanged.
- An empty Destination cell now reads "(root of Data)", with a tooltip: click the cell to type where a file or folder goes.
- The generated XML files are signed with the application's name, version and Nexus page, as FOMOD Creation Tool did.
- Interface labels corrected in every language, and the user manuals aligned on them.
- Interface in 33 languages; every new string is translated in every locale.
- Building from source requires Rust 1.99.0 or later.
