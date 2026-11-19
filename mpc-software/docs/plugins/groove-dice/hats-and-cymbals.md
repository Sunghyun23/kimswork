# Groove Dice hats and cymbals

Current1.6.0 adds nine triplet choices (quarter/eighth/sixteenth × plain/Fill/Roll), three list pages and full-range MAIN D1. The current manual is authoritative. Historical 1.3.0 design below. Current1.3.3 adds independent source editing and unlocked visible Dice changes; see [current manual](manual.md) and [sound/Dice workflow](sound-browser-and-kit-pool.md). Statements below about retaining all rule choices apply to the earlier revision.

MAIN D is **HATS / CYMBALS**: Hat Rhythm / Open Hat / Ride / Crash. Drive, Dust and Room remain available on FX (with Duck, Low Cut, Mod Rate, Mod Depth and Master); they no longer occupy MAIN. The other three MAIN groups are unchanged. Use Q-Link Screen mode for the current page; older host-saved Program/Track maps may retain earlier assignments.

## Choices

| Control | Choices | Behavior |
|---|---|---|
| Hat Rhythm | Existing 15 straight/dotted/Fill/Roll choices | Live division for the closed-hat pattern. |
| Open Hat | OFF / OFFBEAT / BAR END / FILL END | At most one generated open hat per bar, or at the selected fill's last eighth. OFFBEAT chooses one of the four offbeats, fixed until Dice changes its seed. BAR END uses the final eighth. Matching closed hits are replaced rather than doubled. The normal closed/open hat choke applies. |
| Ride | OFF / 1/4 / 1/8 / 1/4 + BELL / 1/8 + BELL | Replaces automatic MAIN closed/open hats, with a stronger downbeat and softer intervening eighth. BELL substitutes the third-beat ride hit. Pauses during recorded fills; physical pads and recorded fill hats remain available. A header hint explains that hats rest when Ride is on. |
| Crash | OFF / START / AFTER FILL / EVERY 4 / EVERY 8 / EVERY 16 | START once at actual playback entry. AFTER FILL at the end of the complete selected fill. Periodic choices at song bars 1,5,9… / 1,9,17… / 1,17,33…; they do not repeat every bar. No crash is added during count-in. |

Rules apply at the continuing transport position when changed. Elapsed notes are not replayed. Ride takes priority over the MAIN open/closed hat rules. FILL END is a fill-ending articulation: it can replace a scheduled closing hat without changing the stored recorded note. Imported/recorded note data are not rewritten. Legacy generated OPEN HAT style aliases BAR END; new projects start with the three new selectors OFF. Generic cymbal ornaments are no longer synthesized by the generated Perc pattern; use Crash for structural accents.

## What Dice does

Dice retains these selector choices, the chosen hat division and the metrical strong/weak/medium/weak hierarchy. It changes permitted motifs, ghost/percussion placement, seeds and small timing/velocity differences according to the existing scope and locks. KIT FIXED protects sounds. With the kit unlocked, a kit-scope roll can also change the kit and its automatically derived cymbal sources. Main-pattern rolls retain recorded Fill A–D unless a Fill/All scope is explicitly selected.

## Sound sources and mixing

Ride, Crash and Bell are independent articulations with separate sample-cache lanes and pinned voices. The engine identifies named families inside the current cymbal source's pack and makes a deterministic choice from that same pack. Kit-fixed rhythm rolls therefore do not silently change those sounds. It does not use one arbitrary cymbal sample for every articulation. Packs without a matching family use separately authored procedural Ride/Crash/Bell fallback sounds, generated off the audio thread. Missing declared base samples remain muted instead of borrowing unrelated sounds.

Ride/Bell follow MAIN **Hat Mix**. Crash follows **Perc Mix**. Their final part level/tune/decay/pan and separate-output destination share the existing Cymbal role on SOUND/ROUTING; this update does not add a ninth output bus or dedicated physical pad. New source choices are automatic, not additional manual sound selectors. Closed/Open Hat physical pads and their assignments remain unchanged.

## Compatibility and verification

The first 533 parameter declarations are unchanged, including legacy FX IDs; the new selectors append at 533–535 (536 total). Existing layer/state trailers and sample-pack format remain unchanged. Old state initializes Ride/Crash OFF; the legacy OPEN HAT style migrates to BAR END. Parameter compatibility does not promise identical old generated cymbal ornaments. Save important projects before upgrading.

Tests cover MAIN and native Q-Link maps; all new choices under fine VST input during playback; Open Hat count/replacement; all Ride divisions/Bell; Crash start/4/8/16 intervals; full-length recorded-fill endings; no stored-note mutation; Dice retaining rules; state roundtrip and old-state loading; Density0; count-in fixtures with the new rules enabled; per-family source/cache/voice separation; routed output and real-time allocation guards. See project VALIDATION.md for recorded results and hardware limits. The local build is not a new public release or website deployment.
