# Crate Press — confirmed development specification

Identity: Crate Press / kim / K36Z / stereo2in2out effect /0.2.2 public preview. A separate public-source project; existing products untouched. User approved proceeding with simple track/master character processing and real multiband compression/ducking. No external sidechain promise.

Current MAIN and16Q-Links: ADrive/Compress/Punch/Release; BLow Comp/Mid Comp/High Comp/Duck; CTone/High Cut/Lo-Fi/Drift; DInput/Low Cut/Mix/Output. This supersedes the original I/O layout; Input/Mix/Output stay together. Internal IDs/order are independent.

User approved permanent MAIN waveform plus simultaneous compression curve and band reduction display. Production target1280×628native tab area. Waveform retains actual signed stereo extrema over64×12ms,30Hz parameter snapshots. Input/output use the same scale and input compensates fixed algorithm latency. No baked waveform or animated fake signal. Static pre-makeup curve uses DSP reduction function; selectable FULL/LOW/MID/HIGH, current detector cursor. Meters display full compressor and per-band/duck reduction.

2026-11-16 latest correction: remove visible`kim`logo/badge and divider, shiftCRATE PRESS title left. Retain vendor metadata`kim`. `design/main-concept-v4.png` is the latest no-brand concept; runtime header uses it. Concept is not a device screenshot.

32factory programs, category guide inpresets.md. Implementation and automated checks pass. Hardware verification is partial; see validation.md for actual results and outstanding physical/audio/CPU coverage. The user authorized public Release and website publication. Classify as Effect, show public-preview status and use actual MPC screenshots in product cards.


## Approved8-effect performance strip — implemented prototype

User approved replacing the6-button proposal with8buttons in one main row: TAPE STOP / REVERSE / REPEAT / DUB ECHO | FILTER DIP / CRUSH / WOBBLE / DROPOUT. Latest approvedconceptv6. MAIN groups reposition to206/372,158px height; buttons150×82at y538, with a wider gap separating4+4. Existing16MAIN/Program assignments preserved. A separatePERFORM tab has16Screen controls; no extra MAIN Q-Link groups.

All are momentary, with lit button and activity/progress feedback. Echo send alone retains a tail. Performance setup stays independent of factory sound programs. Gates347–354/setup355–370/activity371–378 append to existing347; full schema380. Loading state disarms gates and clears them by the first30Hz display tick. Actualdevice touch/CPU/track+master verification isstill pending.


## LATCH — user-approved addition

Headerbutton624/4/140/44. Offdefault usesmomentaryhold; On usesindependenttap toggles for8effects. TurningOff releasesallgates,withnaturalecho tail. NoextraMainQ-Link. Parameter379appends;schema380. Programselectiondoesnot overwriteLatchpreference;state restorekeepsmode butdisarmsgates. Nativephysicalmultitouchremainsunverified;Latchallowsremoteone-pointertests ofmultipleactiveeffects.


## Latest visual corrections

User rejected the mixedSDK/Times typography, then excessivebold weight and dominant section headings. MAINstatic lettering is compiled fromapprovedv6art, with shadow/weight reduction. Section title ink18px; A–Dbadges38×20; control labels remainlegible22px height; numeric text isregularweight. LATCH remains inheader;8performancebuttons retained. This changes artwork only;380parameter IDs andallQ-Link assignments preserved. Currentdevelopment0.2.2.

Latest verification and release record: validation.md and releases/0.2.2.md. The deferred native-skin bridge preserves real-time constraints; detail choice captions are baked into button images to preserve touch input.
