# Loop Club 0.14.1 — DATA-driven PROJECT menu

Owner corrected0.14.0: New/Open/Save belong in a separate menu operated by DATA, not fixed upper action buttons buried in Setup.

SHIFT+SEQ directly opens PROJECT. Four list rows: NEW PROJECT, OPEN PROJECT, SAVE, SAVE A COPY. Turn DATA to select; push to enter/execute. B1 BACK, B2 SELECT and B3 LOOP are navigation equivalents, not New/Open/Save shortcuts. Ordinary SEQ still opens GRID. Recording prevents project-menu entry. The existing Setup2/2 entry remains optional and the old generic PROJECT2/2 heading is now SETUP2/2.

Confirmation is also a DATA list: SAVE AND CONTINUE, DISCARD CHANGES, CANCEL. Default is CANCEL; pressing DATA again cannot accidentally discard work. BACK returns to the previous screen and restores the filename selection. Menu entry never automatically saves. Existing numbered storage, atomic save/backups, legacy Session, validated startup pointer, worker-only I/O and hardware-volume/MIDI-selection preservation stay unchanged. Format12 and DSP are unchanged.

Actual queued controller + DRM checks cover direct entry, all4menu rows, confirmation default/cancel, filename selection, New/Save/SaveCopy/Open, Save+New and stopped restart recall. Physical fingers/power-cycle remain separate. Final normal runtime/SD identities are in validation-0.14.1.json. Public0.13.0/releases/videos remain unchanged.
