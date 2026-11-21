# Felt Club0.3.0 factory bank

Source of truth: `factory-sounds.json`.57stable names/indices, eight individually voiced recipes per engine plus Init. The complete bank and playing roles are documented in [the manual](manual.md#the-complete-bank).

Dice uses `src/SoundProfiles.h`, preserving engine/Sync/velocity curve/playing/chord state and every sound lock. Body/Release are paired. At most one new unlocked color effect; active locked color suppresses new effects. Noise/KeyOff generate zero. Muted uses18–38%Mute and100–220msRelease rather than the obsolete50–76%/45–125msprofile. Other engines have independent contact, character, decay and motion bounds. Undo restores all38sound controls and RNG while retaining current locks.

Approved front Noise revision: MAIN A4=Hiss(index238), LockHiss239 appended; legacy Character4 remains MODEL only. Rare bounded5–18%Noise draws, with individual lock/Undo. Mechanical/KeyOff stillgenerate0; Init and seven dry references haveHiss0. Selected dustyfactoryrecipes receive subtle5–9%hiss.
