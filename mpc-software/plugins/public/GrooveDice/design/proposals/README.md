# SOUND layout review — 2026-11-18

These are offline layout proposals made from the project's UI artwork. They are not MPC screenshots and must not be published as hardware captures.

- sound-rev-tune-qlink-proposal-2026-11-18.png: superseded initial layout. Decay on page1 and a separate touch-only Shift were not the final choice.
- sound-page-1-proposal-2026-11-18.png / sound-page-2-proposal-2026-11-18.png: approved control order. Page1 Sample/Level/Tune/Shift; page2 Start/Decay/LowCut/HighCut. The pictured single page-number button was rejected as unclear navigation.
- The subsequently approved implementation uses separate48×48 previous/next arrows around the visible SOUND1/2 counter, retained REV on both pages, and approved per-row Ø polarity on page2. Fixed16 Screen Q-Links follow the visible row/page; inactive rows do not write another source.

Implementation: ../../tools/sound_skin.py and ../../src/Layers.inc. Verification: ../../tests/sound_pages_test.cpp and ../../tests/skin_test.py. Actual-device screenshots belong in a versioned device capture folder with provenance.
