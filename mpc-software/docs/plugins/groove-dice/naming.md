# Groove Dice display names — 1.1 development

Factory kits, sample pickers and pack/folder captions now use readable aliases. This is a display change: raw bank paths, WAV/MIDI bytes, sample SHA identities, kit IDs, parameter IDs/order and audio settings stay unchanged. Existing saved projects reconnect through their original identities.

- Original Workshop:96 kit titles, such as Velvet Basement, Amber Room and Dusty Sunday. The same title is used by MPC native factory PRESET and the in-plugin kit browser.
- Imported code-style kits:96 distinct titles, such as Crate Basement, After Hours Room and Lowlight Sunday. New packs with already-readable authored titles retain those titles. A conflicting expansion title receives a readable pack tag, for example Velvet Room - Sessions; adding a pack does not rename the core titles.
- Original128 one-shots: descriptive family/role names, such as Dry Kick01, Round Snare03, Wood Rim02 and Shaker04. Each core family's four variants retain their identity. These labels follow the known original synthesis families.
- Private imported WAV: neutral Crate Kick001, Crate Closed Hat001, Crate Chop001 and Crate Loop001 labels. These identify roles and entries; they do not claim a measured sonic character. Counters are local to each pack and role/type, assigned from sorted original paths so merging-directory or pack-install order does not change them for an unchanged pack.
- Init / Pocket becomes Init; Custom / Legacy becomes Custom Kit. Legacy CORE1 display is replaced by the actual core sample alias. Pack captions omit the old numeric prefix and arbitrary10-character truncation. Numbered library folders use Collection labels; CORE8 is labelled CORE SOUNDS.
- Existing readable MIDI names and user-authored host preset filenames are preserved. This does not rename a user's saved .xpl files.

Implementation: src/DisplayNames.h constructs immutable display tables during plugin construction. Runtime raw catalogues remain intact for provenance, pool matching and state restoration. Display tables are not rebuilt or allocated in process().

Verification before final device installation: public97 programs/128 samples and installed personal-profile201 programs/11199 samples pass uniqueness, VST name-length, raw ID/path preservation, byte-identical state and directory-reordering checks. Final build/host regression and hardware status are recorded in VALIDATION.md. Public release remains1.0.0 until separately published.

## Original Workshop kit titles

| Variant | Velvet | Amber | Midnight | Sidewalk | Honey | Dusty |
|---|---|---|---|---|---|---|
| 1 | Velvet Basement | Amber Basement | Midnight Basement | Sidewalk Basement | Honey Basement | Dusty Basement |
| 2 | Velvet Sunday | Amber Sunday | Midnight Sunday | Sidewalk Sunday | Honey Sunday | Dusty Sunday |
| 3 | Velvet Room | Amber Room | Midnight Room | Sidewalk Room | Honey Room | Dusty Room |
| 4 | Velvet Window | Amber Window | Midnight Window | Sidewalk Window | Honey Window | Dusty Window |
| 5 | Velvet Porch | Amber Porch | Midnight Porch | Sidewalk Porch | Honey Porch | Dusty Porch |
| 6 | Velvet Corner | Amber Corner | Midnight Corner | Sidewalk Corner | Honey Corner | Dusty Corner |
| 7 | Velvet Kitchen | Amber Kitchen | Midnight Kitchen | Sidewalk Kitchen | Honey Kitchen | Dusty Kitchen |
| 8 | Velvet Attic | Amber Attic | Midnight Attic | Sidewalk Attic | Honey Attic | Dusty Attic |
| 9 | Velvet Lantern | Amber Lantern | Midnight Lantern | Sidewalk Lantern | Honey Lantern | Dusty Lantern |
| 10 | Velvet Stoop | Amber Stoop | Midnight Stoop | Sidewalk Stoop | Honey Stoop | Dusty Stoop |
| 11 | Velvet Session | Amber Session | Midnight Session | Sidewalk Session | Honey Session | Dusty Session |
| 12 | Velvet Letters | Amber Letters | Midnight Letters | Sidewalk Letters | Honey Letters | Dusty Letters |
| 13 | Velvet Sunrise | Amber Sunrise | Midnight Sunrise | Sidewalk Sunrise | Honey Sunrise | Dusty Sunrise |
| 14 | Velvet Evening | Amber Evening | Midnight Evening | Sidewalk Evening | Honey Evening | Dusty Evening |
| 15 | Velvet Market | Amber Market | Midnight Market | Sidewalk Market | Honey Market | Dusty Market |
| 16 | Velvet Walk | Amber Walk | Midnight Walk | Sidewalk Walk | Honey Walk | Dusty Walk |
