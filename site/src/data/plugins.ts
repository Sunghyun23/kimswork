export interface Plugin {
 slug: string; name: string; kind?: 'instrument' | 'effect'; category: string; tagline: string; description: string; summary: string;
 developmentVersion: string; image: string; imageAlt: string; intro: string; mapping: string[][]; limitations: string[]; verifiedDevice: string; features: {title: string; text: string}[];
 changes: {title:string; text:string}[]; manual?: string; screenshots: {image:string; alt:string; caption:string}[];
 auditions?: {name:string; file:string; description:string}[];
 release: null | { version: string; tag: string; asset: string; preview?: boolean };
}
export const plugins: Plugin[] = [{
 slug:'crate-press',name:'Crate Press',kind:'effect',category:'STEREO CHARACTER & PERFORMANCE EFFECT',
 tagline:'Press your sound into a new shape.',summary:'Character compression, three-band ducking and eight playable lo-fi effects for MPC tracks and masters.',
 description:'Bring weight and movement to drums, keys and samples with independent full-band and three-band compression, drive, sampler color and wow/flutter. Eight performance effects share the main screen with a live stereo waveform and compression curve.',
 developmentVersion:'0.2.6',release:{version:'0.2.6',tag:'crate-press/v0.2.6',asset:'crate-press-0.2.6-mpc.zip',preview:false},image:'crate-press-main-0.2.6.webp',imageAlt:'Actual Crate Press 0.2.6 on MPC XL with shared Dice footer and individual locks',manual:'plugins/crate-press/manual/',
 intro:'Start with one of 32 programs, then use four Q-Link groups to shape the sound. Hold an effect for a brief change, or enable LATCH and combine independent tap toggles.',
 verifiedDevice:'MPC XL / OS 3.10.1.12, internal NVMe; remote footer, curve, hold and latched-state verification; hardware coverage remains partial',
 mapping:[['A · Comp / Drive','Drive','Compress','Punch','Release'],['B · Band / Duck','Low Comp','Mid Comp','High Comp','Duck'],['C · Color','Tone','High Cut','Lo-Fi','Drift'],['D · I/O / Mix','Input','Low Cut','Mix','Output']],
 features:[{title:'Compression in context.',text:'Full-band character compression and a three-band crossover let you control low, mid and high energy separately. Internal bass sensing or host-tempo pulses can duck selected bands.'},{title:'Eight effects, close at hand.',text:'Tape Stop, Reverse, Repeat, Dub Echo, Filter Dip, Crush, Wobble and Dropout can overlap. LATCH keeps independent effects engaged; turn it off to release them all.'},{title:'See the signal.',text:'MAIN permanently shows measured input/output stereo extrema, a static compressor transfer curve and gain-reduction meters. No decorative waveform animation.'},{title:'A practical starting point.',text:'32 programs cover Init, Drums, Keys, Samples, Master and Performance. Factory programs preserve your performance timing and latch preference.'}],
 changes:[{title:'0.2.6: stable release.',text:'Promoted to a regular release after the owner approved distribution. The verified plugin binaries and download checksums are unchanged. Actual hardware verification scope remains documented separately.'},{title:'0.2.6: roll a new character.',text:'Roll Dice shapes thirteen character controls. Protect individual values and Undo the result; Input, Output, Mix, safety and live performance states stay in place. All five pages share the same action row, with smaller locks matching Donut Theory. Actual MPC XL screenshots show the final interface.'},{title:'0.2.5: clearer source and distribution.',text:'Explicit project and SDK MIT notices, with Open MPC ARM32/ARM64 packages alongside the normal SDK ZIP. Engine, 380 parameters, 32 programs, performance controls and UI remain unchanged. That version was a preview; its screenshots showed the unchanged 0.2.4 interface.'},{title:'0.2.4: performance controls, together.',text:'LATCH and FULL/LOW/MID/HIGH curve selection now sit just above the eight performance buttons. The original artwork, lettering, knobs and I/O faders remain. DSP, all 380 parameters, 32 programs, detail pages and Q-Link assignments are unchanged. That version was released as a preview.'},{title:'Quieter typography.',text:'Smaller 18 px section lettering and lighter reference-derived captions keep the controls prominent. Numeric readouts and detail-page choice captions were corrected against the running MPC interface.'},{title:'Combination testing.',text:'All 28 effect pairs were checked at three sample rates against both solo effects, with independent release and echo-tail checks. Hardware multi-touch and worst-case CPU are not certified.'}],
 limitations:['Hardware verification scope and remaining checks are documented in the manual.','External sidechain input is not implemented. Ducking uses internal bass detection or host tempo.','Remote MPC XL checks cover track-insert loading, footer LATCH, curve selection, momentary release, two and all eight latched states, and PERFORM navigation. Earlier Live III checks cover master loading and project recall. Physical Q-Link rotation, finger multi-touch, hardware audio and worst-case CPU remain unverified.','Automated audio tests pass; subjective listening and full hardware audio/latency compensation and worst-case CPU certification remain outstanding.','The transfer curve is static and pre-makeup, not the combined nonlinear effect transfer. The waveform is a 0.77-second extrema overview.','Peak Guard is a soft peak bound, not a true-peak limiter. Captured performance history is limited to eight seconds.'],
 screenshots:[{image:'crate-press-main-0.2.6.webp',alt:'Actual Crate Press 0.2.6 MAIN on MPC XL',caption:'MAIN · Final locks and shared UNLOCK ALL / ROLL DICE / UNDO footer. Actual MPC XL capture; input is silent.'},{image:'crate-press-dice-0.2.6.webp',alt:'Actual MPC Dice result with Drive locked',caption:'DICE · Drive stays locked at zero while the other character values change. Input, Output and Mix are preserved.'},{image:'crate-press-color-0.2.6.webp',alt:'Actual Crate Press COLOR page with shared locks',caption:'COLOR · The same locks and footer are available in detail views.'},{image:'crate-press-perform-0.2.6.webp',alt:'Actual Crate Press 0.2.6 PERFORM settings',caption:'PERFORM · Independent performance settings and the shared footer on MPC XL.'}]
},
{
  "slug": "donut-theory",
  "name": "Donut Theory",
  "category": "POLYPHONIC & MONOPHONIC SYNTHESIZER",
  "tagline": "A little worn. Endlessly warm.",
  "summary": "Warm synthesis. Rich chords. Movement in every direction.",
  "description": "A Moog-inspired synthesizer for MPC bass, keys, leads and pads. Edit one shared sound from POLY or MONO without changing its mode, shape each oscillator and delay either LFO, then build your own chord changes.",
  "developmentVersion": "1.15.0",
  "release": {
    "version": "1.15.0",
    "tag": "donut-theory/v1.15.0",
    "asset": "donut-theory-1.15.0-mpc.zip"
  },
  "image": "donut-theory-poly-device-1.15.0.webp",
  "imageAlt": "Actual MPC XL Donut Theory 1.15.0 POLY performance editor",
  "manual": "plugins/donut-theory/manual/",
  "intro": "66 synthesized factory sounds, 32 chord types and 96 progression recipes. Tap ROLL DICE to generate a complete oscillator-to-FX sound with synchronized motion, while keeping your chords and master level. No external samples are required.",
  "verifiedDevice": "MPC XL / OS 3.10.1.12, NVMe; actual remote Phase/FREE-RESET, Drift isolation, wave browsing, locks, Undo, native presets and rendering checks. Native regression/UBSan and 19 host checks pass. Physical encoders, hardware audio and worst-case CPU remain separate.",
  "mapping": [
    [
      "A \u00b7 Selected OSC",
      "Wave",
      "Pitch \u00b7 Tune/Fine",
      "Level",
      "Drift / Phase"
    ],
    [
      "B \u00b7 Filter / Tone",
      "Filter Cutoff",
      "Resonance",
      "Body",
      "Drive"
    ],
    [
      "C \u00b7 Selected LFO",
      "Rate",
      "Depth",
      "Delay",
      "Target"
    ],
    [
      "D \u00b7 POLY Amp / Motion",
      "Amp Attack",
      "Amp Release",
      "Motion Rate",
      "Motion Depth"
    ]
  ],
  "features": [
    {
      "title": "Two views. One shared sound.",
      "text": "POLY and MONO are editing views. Tab changes preserve the actual voice mode and every sound value; use VOICE to switch modes. POLY brings Amp/Motion forward, while MONO offers Glide, Decay, Release and Pitch Drop."
    },
    {
      "title": "Shape each oscillator.",
      "text": "Choose eight waves independently for each oscillator. Main A4 switches between Drift and OSC2 Phase without changing either value. FREE preserves phase history; RESET aligns new articulations. OSC keeps pulse widths, Sub, noise, FM and Tone."
    },
    {
      "title": "Motion that arrives in time.",
      "text": "Both LFOs have independent Rate, Depth, Delay and Target. Per-note Delay waits up to five seconds before modulation fades in. Each LFO and Tremolo/Auto Pan offers 15 straight, dotted and triplet Sync periods; small knob turns accumulate normally."
    },
    {
      "title": "Explore. Protect. Return.",
      "text": "62 shared value locks protect actual sound parameters across views. Roll a whole sound within a chosen family, Undo the generation or Unlock All without changing a note. Actual mode, harmony, MIDI, master and both LFO Delays remain intact."
    },
    {
      "title": "66 sounds. A warm place to start.",
      "text": "Bass, Keys, Synth, Lead and Pad, plus dry Init. Category-first plugin browsing and the native MPC preset list share all 66 sounds. Dusty Sunday and Velvet Melt remain included."
    },
    {
      "title": "Make room for rich harmony.",
      "text": "32 chord types, 96 progression recipes and up to 16 slots. POLY pads sound while held; CHORD exposes key/circle reference, inversions, octaves, Smooth Voicing, Strum, chord glide and individual MIDI learn."
    },
    {
      "title": "Two envelopes. Clear release control.",
      "text": "ENV combines editable Amp and Filter ADSRs with filter response. Amp Release is shared across POLY, MONO and ENV. MONO places downward release Pitch Drop beside it; the remaining tail determines how much bend you hear."
    }
  ],
  "changes": [
    {
      "title": "1.15.0: new phase. More colors.",
      "text": "Main-screen Phase with FREE/RESET, eight waves for each oscillator, a dedicated phase Dice lock and exact Undo. Existing 66 factory programs retain their original default sound. Actual MPC screenshots and a detailed manual cover the new controls."
    },
    {
      "title": "1.15.0: selectors that line up.",
      "text": "Consistent one- or two-column widths, clear text and arrow spacing, and FREE/SYNC immediately above Rate. Footer selectors match in size. The sound engine, parameters and Q-Link assignments are unchanged."
    },
    {
      "title": "Retained from 1.14.1: shared performance editors.",
      "text": "New POLY/MONO views never switch your sound or actual voice mode. Individual oscillator focus, Tune/Fine, LFO Delay and independent detail pages share one saved sound. The explicit VOICE button controls Poly/Mono."
    },
    {
      "title": "Small turns. Fifteen Sync choices.",
      "text": "Fixed the per-block cursor snapback that discarded small Q-Link turns. LFO1, LFO2 and Motion offer straight, dotted and triplet periods, clockwise from slow to fast. Free Hz and original straight periods are retained."
    },
    {
      "title": "Real MPC value-display correction.",
      "text": "Device QA found recursive Data-to-Data label bindings that blanked the numeric displays. Value labels now inherit their actual parent parameter. The new regression prevents the same skin defect."
    },
    {
      "title": "Bring your projects forward.",
      "text": "Same kim/Krql identity and 66 factory programs. The first 353 parameter declarations remain unchanged; 391 total. Old oscillator and five-step Sync values migrate, while locks and one-step Dice Undo share the new controls."
    },
    {
      "title": "Native presets stay available.",
      "text": "The earlier notification-storm fix remains in place. The MPC header lists factory programs, while the plugin browser opens categories first. Explicit factory selection preserves the actual voice mode and your harmony/MIDI/master settings."
    }
  ],
  "limitations": [
    "The gray MPC host PRESET list is flat. Category-first navigation is provided inside SOUND PRESET.",
    "The sixteen-voice pool is shared by melody and chords. Long overlapping chord releases can steal older voices.",
    "Screen chord pads audition sound; they do not send MIDI notes to the MPC sequencer. Record physical trigger notes instead.",
    "SCREEN mode follows CHORD; Program/Track retains the synth mapping. Old tracks may cache previous defaults.",
    "Actual XL checks cover remote screen operation and installed-file read-back. Complete physical Q-Link/multi-touch, worst-case CPU, hardware audio and full project round-trip certification remain outstanding.",
    "Native audio demos are engine renders, not MPC output recordings. Testing on every MPC model/OS is not certified.",
    "The engine changed in 1.2.0 and explicit Sub in 1.2.3; retained parameter identities do not guarantee identical older project sounds.",
    "Pitch Drop needs an audible Amp Release tail. Editable envelope diagrams are parameter views, not calibrated time plots or hardware waveform measurements."
  ],
  "screenshots": [
    {
      "image": "donut-theory-poly-device-1.15.0.webp",
      "alt": "Donut Theory POLY editor on MPC XL",
      "caption": "POLY \u00b7 Selected OSC and LFO controls with Amp/Motion; one shared sound. Actual MPC XL / OS 3.10.1.12, version 1.15.0."
    },
    {
      "image": "donut-theory-mono-device-1.15.0.webp",
      "alt": "Donut Theory MONO editor on MPC XL",
      "caption": "MONO \u00b7 Glide, Decay, Release and Pitch Drop. The editor tab does not change the actual voice mode. Actual MPC XL / OS 3.10.1.12, version 1.15.0."
    },
    {
      "image": "donut-theory-osc-device-1.15.0.webp",
      "alt": "Donut Theory independent OSC controls on MPC XL",
      "caption": "OSC \u00b7 Individual Wave, Tune, Fine, Level and Pulse Width, plus texture controls. Actual MPC XL / OS 3.10.1.12, version 1.15.0."
    },
    {
      "image": "donut-theory-env-device-1.15.0.webp",
      "alt": "Donut Theory combined ADSR screen on MPC XL",
      "caption": "ENV \u00b7 Both editable ADSRs and filter response; Amp Release is shared with the fronts. Actual MPC XL / OS 3.10.1.12, version 1.15.0."
    },
    {
      "image": "donut-theory-mod-device-1.15.0.webp",
      "alt": "Donut Theory delayed LFOs and Motion on MPC XL",
      "caption": "MOD \u00b7 Both LFOs, independent Delay and shared Tremolo/Auto Pan. Actual MPC XL / OS 3.10.1.12, version 1.15.0."
    },
    {
      "image": "donut-theory-fx-device-1.15.0.webp",
      "alt": "Donut Theory FX and output on MPC XL",
      "caption": "FX \u00b7 Drive/color, space/echo and output with shared sound locks. Actual MPC XL / OS 3.10.1.12, version 1.15.0."
    },
    {
      "image": "donut-theory-chord-device-1.15.0.webp",
      "alt": "Donut Theory CHORD editor and circle of fifths on MPC XL",
      "caption": "CHORD \u00b7 Root, type, voicing and octave for each visible slot, with key reference and harmony tools. Actual MPC XL / OS 3.10.1.12, version 1.15.0."
    },
    {
      "image": "donut-theory-waves-device-1.15.0.webp",
      "alt": "Actual MPC XL eight-wave selector sidebar",
      "caption": "WAVE \u00b7 Eight choices per oscillator in the existing sidebar. Actual MPC XL capture."
    },
    {
      "image": "donut-theory-phase-device-1.15.0.webp",
      "alt": "Actual MPC XL main-screen Phase control",
      "caption": "PHASE \u00b7 A4 edits OSC2 phase relative to OSC1. FREE/RESET sits beside the value. Actual MPC XL capture."
    }
  ],
  "auditions": [
    {
      "name": "Keys / Dusty Sunday",
      "file": "donut-theory-dusty-sunday-1.9.0.wav",
      "description": "Soft sine/triangle keys \u00b78 ms Strum \u00b770% Auto Pan."
    },
    {
      "name": "Synth / Velvet Melt",
      "file": "donut-theory-velvet-melt-1.9.0.wav",
      "description": "Soft saw/triangle \u00b76 ms Strum \u00b765 ms chord glide \u00b780% Auto Pan."
    }
  ]
},
{
 slug:'groove-dice', name:'Groove Dice',manual:'plugins/groove-dice/manual/', category:'DRUM & GROOVE INSTRUMENT',
 tagline:'Roll a groove. Play it your way.',
 summary:'Generate a groove, perform fills and snare rolls, then export your take as MIDI.',
 description:'A neo-soul and hip-hop drum instrument for MPC. Generate a musical one-bar groove, perform four fills, four snare rolls and four hat rolls from touch or assigned pads, and capture the result for MIDI export.',
 developmentVersion:'1.8.0', release:{version:'1.8.0',tag:'groove-dice/v1.8.0',asset:'groove-dice-1.8.0-mpc.zip'}, image:'groove-dice-perform-device-1.8.0.webp',
 imageAlt:'Actual MPC XL Groove Dice 1.8.0 PERFORM screen with four fills, four snare rolls and four hat rolls; public Original Workshop only',
 intro:'Includes 128 original synthesized one-shots, 96 kit combinations, two authored beats and 93 timing templates. Optional .gdexp packs stay separate. No commercial or personal sample library is bundled.',
 verifiedDevice:'MPC XL / OS 3.10.1.12, internal NVMe; actual remote touch checks for loop/fill/roll, generation/locks, native presets, editing and MIDI export. Public-only screenshots. Physical B1/B3 were owner-confirmed with correct bar-end completion. Full new-pad/encoder coverage and worst-case hardware CPU are not newly certified.',
 mapping:[['A · Timing','Swing','Snare Late','Hat Shift','Rnd Nudge'],['B · Rules','Kick','Snare','Hat','Perc'],['C · Drum Mix','Kick Level','Snare Level','Hat Level','Perc Level'],['D · Hats / Cymbals','Hat Rhythm','Open Hat','Ride','Crash']],
 features:[
  {
    "title": "Roll a musical starting point.",
    "text": "Sixteen rules per drum part, metrical velocity accents and 24 straight/dotted/triplet hat rhythms create a repeating one-bar groove. MAIN locks preserve the timing, rules and mix you want to keep."
  },
  {
    "title": "Play the changes.",
    "text": "PERFORM gives you four fills, four snare rolls, four hat rolls and loop/stop controls. Touch the screen or assign physical pads. Rolls and fills play their complete selected length at the end of a bar. Late presses wait for the next valid bar, then return to the groove. Releasing a pad early does not cut a roll short."
  },
  {
    "title": "Keep your favorite phrases.",
    "text": "Global Dice regenerates unlocked fills and rolls with the main groove. Lock a slot to keep it, or explicitly generate just that slot. Recorded and edited fills lock automatically; populated older-project fills load protected. Four Screen Q-Links edit the selected slot."
  },
  {
    "title": "Turn a performance into MIDI.",
    "text": "CAPTURE records the actual drum events, including fills and rolls. EXPORT MIDI saves a .mid file for import into an MPC track. Keep the take before closing the instance; live plugin MIDI output remains unavailable in the host."
  },
  {
    "title": "Shape every drum.",
    "text": "Four kick/snare layers, linked hats, a percussion ensemble and independent Ride/Crash/Bell sources. Browse and audition sounds, then use two SOUND pages for tuning, trim, Decay, Shift and filters. REV and polarity remain independent."
  },
  {
    "title": "Build a library that is yours.",
    "text": "Original Workshop includes 128 original sounds and 96 kit combinations. Add independent .gdexp packs with the optional macOS/Windows Pack Builder. Dice can choose sounds from one pack, every pack or selected kits."
  }
],
 changes:[
  {title:'1.8.0: musical phrases, now with hats.',text:'Four hat rolls on B9–B12 join snare rolls and fills. Motifs, rests, beat accents and shaped endings replace uniform streams; fills keep the main kick and 2/4 snare anchors. Twelve slot locks, selected-slot Q-Links, Dice and Undo. Queued phrases clear on count-in. First 880 declarations remain unchanged; 917 total.'},
  {title:'1.7.1: the right pad, the full phrase.',text:'Correct Bank B defaults start at MIDI48 in the tested C-major layout. Known incorrect 1.7.0 defaults migrate while custom learned assignments remain. Fills and rolls finish their full selected length at a bar boundary; late triggers move to the next valid bar. PERFORM uses only the MAIN/PERFORM subtab; MAIN retains EDIT FILL for note editing.'},
  {title:'1.7.0: generate, perform, capture.',text:'Four fills and four snare rolls with pad triggers, selected-slot Q-Links, generation and locks. Dice now regenerates open performance slots at the next bar. Capture and export actual MIDI notes; expansion packs and captured takes are preserved by portable upgrades. Existing 782 declarations remain exact; 880 parameters total.'},
  {title:'1.6.2: keep your expansion packs.',text:'Open MPC packages now preserve Content/Expansions during upgrades, including the pack registry. Actual Linux installer tests reproduce the old loss and verify upgrade, reinstall and uninstall preservation for both CPUs. Fresh MPC screenshots show only the public core: 128 sounds and 95 MIDI assets.'},
  {title:'1.6.1: your own packs, documented.',text:'Includes a complete Pack Builder workflow and clear MIT notices for the plugin and original core content. Pack Builder is a separate optional desktop app for macOS Apple Silicon and Windows x64. Engine, UI, 782 parameters, Q-Link maps and the core bank stay unchanged.'},
  {title:'1.6.0: hats in threes.',text:'Nine new Triplet choices: 1/4T, 1/8T and 1/16T, each basic, +FILL or +ROLL. The three-page browser, MAIN D1, GROOVE, Dice and Undo use the same 24-choice range. Old projects keep their notes and legacy parameter ranges. Momentary Fill/FX rolls clear after execution.'},
  {title:'1.5.1: three rolls, clear boundaries.',text:'ROLL DICE reshapes MAIN and its pattern while preserving every fill and FX value. Dedicated ROLL FILL and ROLL FX actions have Undo. Individual locks stay on MAIN; kit, role and layer protection remain. UNLOCK ALL changes protection only.'},
  {title:'1.4.0: 36 new ways to move.',text:'Nine new rules per part extend the original seven to sixteen. New choices include END DOUBLE kicks, DRAG IN snares, CRESCENDO hats and LATIN THREE percussion. Live edits and Roll Dice use the same full range, honor locks and preserve recorded fills.'},
  {title:'1.3.4: two sound pages. Every source in reach.',text:'Large previous/next arrows and a visible SOUND1/2 counter switch all rows and their SCREEN Q-Links together. Tune is on page1, Decay on page2. REV and polarity are independent for all17sources; no separate Detail screen is needed.'},
  {title:'Visible Dice, individual locks.',text:'Roll Dice updates eligible MAIN values. Separate 44px padlocks replace LOCK in the RULE selectors. Older saved LOCK states retain their frozen rhythm and migrate to BASIC with a closed padlock.'},
  {title:'A complete sound workflow.',text:'Tap-to-browse lists, synchronized arrows/Q-Link, isolated previews, Decay on every sound row and editable Ride/Crash/Bell sources. Dice pools can be limited to selected kits. The blank-skin and overlapping kit-caption defects are fixed.'},
  {title:'One repeating bar, with room to perform.',text:'Live rules, metrical velocities, hats/cymbals and independent recorded fills build on the one-bar engine. ROOM is a stereo diffused reverb. FOLLOW retains automatic record-count-in detection.'}
 ],
 limitations:['CAPTURE holds up to 32768 note-ons. Export before closing the instance; the MIDI file uses the starting tempo and has no tempo-change map. Full MPC file import is not newly hardware-tested.','Signed Shift reports a fixed 30ms baseline latency, including direct pads. Hardware compensation and playing feel are not certified.','Remote UI checks cover page/REV/polarity operation; full physical Q-Link coverage, worst-case CPU and every output destination remain unverified.','A stopped recording intentionally started inside a bar may wait for a host bar anchor/seek; use aligned starts or punch in during running playback.','External sidechain input and generated MIDI output to the MPC sequencer are not implemented.','Install optional packs using Pack Builder or the Python CLI, then reopen the plugin. This is not native MPC XPN registration.'],
 screenshots:[
  {
    "image": "groove-dice-perform-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 perform screen, public core only",
    "caption": "PERFORM \u00b7 Four fills, four snare rolls and four hat rolls and selected-slot controls. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-main-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 main screen, public core only",
    "caption": "MAIN \u00b7 The original four groups remain in place. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-roll-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 roll screen, public core only",
    "caption": "ROLL \u00b7 Finite hat rolls use rhythmic motifs while kick and snare continue. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-capture-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 capture screen, public core only",
    "caption": "CAPTURE \u00b7 Capture holds the performed drum notes; export saves a .mid file. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-groove-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 groove screen, public core only",
    "caption": "GROOVE \u00b7 Timing, density and variation details. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-sound-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 sound screen, public core only",
    "caption": "SOUND 1/2 \u00b7 Sample, Level, Tune and Shift. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-sound2-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 sound2 screen, public core only",
    "caption": "SOUND 2/2 \u00b7 Start, Decay and filters. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-cymbals-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 cymbals screen, public core only",
    "caption": "CYMBALS \u00b7 Independent Ride, Crash and Bell sources. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-fill-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 fill screen, public core only",
    "caption": "FILL EDIT \u00b7 Recorded notes remain editable. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-kits-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 kits screen, public core only",
    "caption": "KITS \u00b7 Original presets and separate expansion packs. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-fx-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 fx screen, public core only",
    "caption": "FX \u00b7 Drive, Dust and stereo Room. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  },
  {
    "image": "groove-dice-routing-device-1.8.0.webp",
    "alt": "Actual MPC XL Groove Dice1.8.0 routing screen, public core only",
    "caption": "ROUTING \u00b7 Drum outputs and performance pad learning. Actual MPC XL / OS3.10.1.12; Original Workshop128WAV/95MIDI, optional owner packs not loaded."
  }
]
}];
export const repo = 'https://github.com/Sunghyun23/kimswork';
export const navigationVersion = 'software-2026-11-20-phase-hats';
export const url = (path = '') => {
 const target = new URL(`${import.meta.env.BASE_URL.replace(/\/$/, '')}/${path.replace(/^\//, '')}`, 'https://site.invalid');
 if (target.pathname.endsWith('/') || target.pathname.endsWith('.html')) target.searchParams.set('v', navigationVersion);
 return target.pathname + target.search + target.hash;
};
export function releaseLinks(p: Plugin) {
 if (!p.release) return null;
 const tag = `${p.slug}/v${p.release.version}`;
 if (p.release.tag !== tag || !/^[a-z0-9.-]+\.zip$/.test(p.release.asset)) throw new Error(`Invalid release: ${p.slug}`);
 return { page: `${repo}/releases/tag/${encodeURIComponent(tag)}`, asset: `${repo}/releases/download/${encodeURIComponent(tag)}/${p.release.asset}` };
}
