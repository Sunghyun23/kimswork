# kimswork structure migration — 2026-11-16

User-approved changes: repository and local folder renamed from mpc-plugins to kimswork; MPC work grouped under mpc-software; Groove Dice promoted to public source with only original core content; restricted Splat-to-Clat material retained as a private optional expansion.

```text
kimswork/
├── AGENTS.md
├── README.md
├── .github/workflows/pages.yml
├── docs/                          repository and website operations
├── site/                          kimswork.app Astro source
└── mpc-software/
    ├── README.md
    ├── requirements-dev.txt
    ├── plugins/
    │   ├── public/
    │   │   ├── DonutTheory/
    │   │   └── GrooveDice/
    │   └── private/               ignored individual project folders
    ├── apps/public/GrooveDicePackBuilder/
    ├── expansions/groove-dice/
    │   ├── public/original-workshop/
    │   │   ├── README.md
    │   │   ├── tools/build.py
    │   │   ├── work/              ignored generated audio/MIDI/catalogues
    │   │   └── dist/              ignored .gdexp packages
    │   └── private/
    │       ├── README.md
    │       └── splat-to-clat/     entirely ignored
    │           ├── research/
    │           ├── tools/
    │           ├── work/
    │           └── dist/
    ├── shared/groove-dice-expansion/
    ├── docs/
    │   ├── plugins/{donut-theory,groove-dice}/
    │   ├── apps/groove-dice-pack-builder/
    │   └── expansions/groove-dice/
    ├── scripts/
    ├── third_party/
    │   ├── sdk.lock.json
    │   └── mpc-plugin-builder-sdk/ ignored, single pinned SDK
    └── local-content/             ignored local research staging
```

The root site and GitHub workflow remain at their original top-level locations. The Pages manual import and path filters were changed to the relocated manual. Existing Donut Theory tag names and assets remain unchanged. GitHub repo ID is 1456587114; custom domain kimswork.app, approved HTTPS certificate and enforced HTTPS were confirmed after the rename.

Groove Dice has no dependency on the private tree for its standard build, parameter generation, default test or public packaging. Immutable legacy parameter choice captions/ranges are stored in legacy-catalogue.json. The optional --with-private tests and --personal package profile explicitly use the private expansion tree. Public source contains the small original core bank and its manifest/provenance. Runtime banks are separate files; payloads are not merged.

SDK source and plugin UID/parameter identities are preserved. Original Workshop authoring moved to its own expansion project with a compatibility wrapper. Shared expansion manager is standalone and packaged without a repo-relative dependency. Pack Builder is an approved design, not an implemented executable.

Historical artifacts/logs may contain earlier absolute paths; these are records, not build configuration. Existing MPC-installed plugins are unaffected by a source folder migration. No new device compatibility or worst-case CPU certification is implied.

Validation results are recorded below when complete. No new plugin Release is created by this migration.

## Verification completed

- Both plugins: native/ARM32/ARM64 builds, SDK validator0errors/0warnings; regression suites passed, host simulator19PASS/0WARN/0FAIL for each.
- Groove Dice build/parameter generation/public ZIP succeeded with the private pack path absent. Packaged shared CLI executed from a temporary directory outside the repository. ZIP contains only the original bank.
- Independent Original Workshop generator rebuilt the exact original payload SHA b98c4b813ff381fb5befca49bfa7385c5c7ace9d54355739569596cf8b81bb8f.
- Site Astro build and53 local page/image/CSS/anchor references passed; all checked local Markdown targets resolve. Groove Dice has a development-only catalogue entry with an actual core-only MPC screenshot; no fake release/download.
- Private expansion, working audio, SDK checkout, plugin build directories and attachments are ignored; public candidates were checked for restricted payload paths. SDK commit remains febd2ca5bd9eb16c4a412837196e145566bc981b and its tracked files are unmodified.
- Build logs and JSON verification record are retained locally under GrooveDice/build/qa/structure-*. No new hardware certification or plugin Release is implied.
