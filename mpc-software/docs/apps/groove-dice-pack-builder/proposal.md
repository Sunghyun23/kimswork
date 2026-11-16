# Groove Dice Pack Builder — proposal

Status: proposed UI/technology, not implemented or approved as a finished application. User-required scope: macOS and Windows desktop app; choose an input root folder with separated instrument folders; automatically build an independent expansion pack. Existing CLI is not the desktop application.

Proposed workflow: choose folder → recursive scan/role mapping review → generated kit audition/edit → .gdexp export/install. Support role-pool folders Kick/Snare/ClosedHat/OpenHat/Clap/Tom/Cymbal/Perc plus MIDI/Beats and MIDI/Grooves; alternatively recognize Kits/<kit-name>/<role>. Folder roles take priority over filename guesses. Only use inputs inside the selected source root; never copy arbitrary sounds from installed packs to fill missing roles. Ambiguous or missing roles must be visible for review. Partial/sample-only packs need explicit runtime-format handling; do not silently imply that the current full-kit writer already supports all partial layouts.

Proposed UI: left pack metadata/name/vendor/cover/privacy; center eight role cards, source file list, waveform and audition; right generated kits with individual part locks/regeneration; bottom input/output/actions/progress. Preserve source audio by default; balancing in non-destructive kit settings. Save a project mapping and stable pack ID for repeatable updates. Privacy/redistribution classification is a user declaration, not automatic legal clearance.

Initial scope: WAV/SMF input validation, aliases/recursive folder matching, kit generation/editing, audition, project save/reopen, .gdexp export, local install/list/remove. Original procedural synthesis is a separate optional future feature. No cloud upload required.

Proposed implementation: PySide6 + existing Python packaging backend; deliver standalone macOS .app and Windows .exe with runtime bundled. Build/test separately on macOS and Windows; do not claim Windows compatibility based on Mac execution. Official deployment reference: https://doc.qt.io/qtforpython-6/deployment/deployment-pyside6-deploy.html . Name and layout remain proposals.


Current locations (2026-11-16): app project `mpc-software/apps/public/GrooveDicePackBuilder/`; shared standalone CLI `mpc-software/shared/groove-dice-expansion/`; packs `mpc-software/expansions/groove-dice/{public,private}/`. GUI remains unimplemented.
