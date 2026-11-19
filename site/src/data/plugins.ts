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
 developmentVersion:'0.2.4',release:{version:'0.2.4',tag:'crate-press/v0.2.4',asset:'crate-press-0.2.4-mpc.zip',preview:true},image:'crate-press-main-0.2.4.webp',imageAlt:'Actual Crate Press 0.2.4 track insert on MPC XL with footer LATCH and curve choices above eight performance buttons',manual:'plugins/crate-press/manual/',
 intro:'Start with one of 32 programs, then use four Q-Link groups to shape the sound. Hold an effect for a brief change, or enable LATCH and combine independent tap toggles.',
 verifiedDevice:'MPC XL / OS 3.10.1.12, internal NVMe; remote footer, curve, hold and latched-state verification; hardware coverage remains partial',
 mapping:[['A · Comp / Drive','Drive','Compress','Punch','Release'],['B · Band / Duck','Low Comp','Mid Comp','High Comp','Duck'],['C · Color','Tone','High Cut','Lo-Fi','Drift'],['D · I/O / Mix','Input','Low Cut','Mix','Output']],
 features:[{title:'Compression in context.',text:'Full-band character compression and a three-band crossover let you control low, mid and high energy separately. Internal bass sensing or host-tempo pulses can duck selected bands.'},{title:'Eight effects, close at hand.',text:'Tape Stop, Reverse, Repeat, Dub Echo, Filter Dip, Crush, Wobble and Dropout can overlap. LATCH keeps independent effects engaged; turn it off to release them all.'},{title:'See the signal.',text:'MAIN permanently shows measured input/output stereo extrema, a static compressor transfer curve and gain-reduction meters. No decorative waveform animation.'},{title:'A practical starting point.',text:'32 programs cover Init, Drums, Keys, Samples, Master and Performance. Factory programs preserve your performance timing and latch preference.'}],
 changes:[{title:'0.2.4: performance controls, together.',text:'LATCH and FULL/LOW/MID/HIGH curve selection now sit just above the eight performance buttons. The original artwork, lettering, knobs and I/O faders remain. DSP, all 380 parameters, 32 programs, detail pages and Q-Link assignments are unchanged. This remains a public preview.'},{title:'Quieter typography.',text:'Smaller 18 px section lettering and lighter reference-derived captions keep the controls prominent. Numeric readouts and detail-page choice captions were corrected against the running MPC interface.'},{title:'Combination testing.',text:'All 28 effect pairs were checked at three sample rates against both solo effects, with independent release and echo-tail checks. Hardware multi-touch and worst-case CPU are not certified.'}],
 limitations:['Public preview: hardware verification is partial; read the manual before using it in a critical live set.','External sidechain input is not implemented. Ducking uses internal bass detection or host tempo.','Remote MPC XL checks cover track-insert loading, footer LATCH, curve selection, momentary release, two and all eight latched states, and PERFORM navigation. Earlier Live III checks cover master loading and project recall. Physical Q-Link rotation, finger multi-touch, hardware audio and worst-case CPU remain unverified.','Automated audio tests pass; subjective listening and full hardware audio/latency compensation and worst-case CPU certification remain outstanding.','The transfer curve is static and pre-makeup, not the combined nonlinear effect transfer. The waveform is a 0.77-second extrema overview.','Peak Guard is a soft peak bound, not a true-peak limiter. Captured performance history is limited to eight seconds.'],
 screenshots:[{image:'crate-press-main-0.2.4.webp',alt:'Crate Press 0.2.4 MAIN track insert running on MPC XL',caption:'MAIN · Actual MPC XL 0.2.4 capture. Footer LATCH and curve selection above eight original performance buttons. Input is silent in this capture.'},{image:'crate-press-latched-0.2.4.webp',alt:'Actual MPC Crate Press with all eight performance effects visibly latched',caption:'LATCH · All eight effects show independent active states; turning LATCH off clears them. This visual check is separate from hardware audio certification.'},{image:'crate-press-perform-0.2.4.webp',alt:'Crate Press 0.2.4 PERFORM settings on MPC XL',caption:'PERFORM · Actual MPC XL capture. The independent timing, feedback, texture and release controls are unchanged.'}]
},
{
  "slug": "donut-theory",
  "name": "Donut Theory",
  "category": "POLYPHONIC SYNTHESIZER",
  "tagline": "A little worn. Endlessly warm.",
  "summary": "Warm synthesis. Rich chords. Movement in every direction.",
  "description": "A Moog-inspired synthesizer for MPC bass, keys, leads and pads. Shape its nonlinear ladder engine with Body and Sub, two independent LFOs and synchronized Tremolo or Auto Pan, then build your own chord changes. Shape both envelopes on ENV and add a downward pitch release from PLAY.",
  "developmentVersion": "1.13.14",
  "release": {
    "version": "1.13.14",
    "tag": "donut-theory/v1.13.14",
    "asset": "donut-theory-1.13.14-mpc.zip"
  },
  "image": "donut-theory-main-device-1.13.14.webp",
  "imageAlt": "Actual MPC XL Donut Theory 1.13.14 with individual shared Dice locks",
  "manual": "plugins/donut-theory/manual/",
  "intro": "66 synthesized factory sounds, 32 chord types and 96 progression recipes. Tap ROLL DICE to generate a complete oscillator-to-FX sound with synchronized motion, while keeping your chords and master level. No external samples are required.",
  "verifiedDevice": "MPC XL / OS3.10.1.12; native presets, Dice and MAIN/CHORD verified;177installed files matched",
  "mapping": [
    [
      "A \u00b7 OSC / Sound",
      "OSC Pair",
      "OSC Mix",
      "Detune",
      "Drift"
    ],
    [
      "B \u00b7 Filter / Tone",
      "Filter Cutoff",
      "Resonance",
      "Body",
      "Sub"
    ],
    [
      "C \u00b7 LFO 1 /2",
      "LFO 1 Rate",
      "LFO 1 Depth",
      "LFO 2 Rate",
      "LFO 2 Depth"
    ],
    [
      "D \u00b7 FX / Motion",
      "Drive",
      "Glide",
      "Mod Rate",
      "Mod Depth"
    ]
  ],
  "features": [
    {
      "title": "Shape the whole release.",
      "text": "PLAY places Amp Release, Drop Amount and Drop Time together. Release a key to slide down by up to24semitones; sustain and legato fallback follow your playing. Set Amount to0 to turn it off."
    },
    {
      "title": "Two envelopes. One clear view.",
      "text": "ENV brings Amp ADSR and Filter ADSR side by side, with editable diagrams and vertical faders. Filter response stays together below. PLAY Release shares the Amp Release value, while Filter Release stays independent."
    },
    {
      "title": "66 sounds. A warm place to start.",
      "text": "Bass, Keys, Synth, Lead and Pad, plus dry Init. Category-first browsing and per-sound low-cut keep selection practical. Dusty Sunday and Velvet Melt append to the original 64 programs."
    },
    {
      "title": "Weight and texture, side by side.",
      "text": "Sub adds a sine one octave below the main oscillator. Body adds broad low-mid density through gentle saturation. Filter Cutoff and Resonance share the same group; amp/filter ADSRs stay on detail pages."
    },
    {
      "title": "Two LFOs. One shared motion effect.",
      "text": "Each LFO has independent Shape, Target, Rate, Depth and Sync. Choose Tremolo or Auto Pan in a separate module. Every synchronized Rate uses its existing Q-Link to select a division and remembers its free Hz value."
    },
    {
      "title": "Find a home for your chords.",
      "text": "Set a Major or Minor reference Key beside the circle of fifths. Explicit transpose moves active roots together. Smooth Voicing proposes smaller top-note movement, with preview before you apply it."
    },
    {
      "title": "Play the space between notes.",
      "text": "Overlapping chord triggers can glide between nearby voices while holding common tones. One-shot Strum spreads the chord from bass to top. Ordinary Mono/Legato melody remains separate."
    },
    {
      "title": "Sixteen slots, four within reach.",
      "text": "Expand from 4 to 16 slots. Four large MAIN pads play while held; physical pads connect through individual MIDI Note/LEARN. CHORD uses sixteen SCREEN Q-Links for Root, Type, Voicing and Octave."
    }
  ],
  "changes": [
    {
      "title": "1.13.14: lock your favorites. Keep exploring.",
      "text": "Individual shared Dice locks and UNLOCK ALL protect selected sound values across pages. Native preset browsing is restored by eliminating repeated false refreshes; snapshots preserve live gestures. Factory sounds and Q-Link maps are unchanged."
    },
    {
      "title": "1.12.0: shape what happens after the note.",
      "text": "ENV replaces FILTER and PLAY replaces AMP. Both ADSRs share one screen; PLAY D adds Release / Drop Amount / Drop Time. Old projects default to Drop Off; the first291parameters remain unchanged, with only two appended for293total. MAIN and its filter controls are unchanged."
    },
    {
      "title": "1.10.1: browse, create, play.",
      "text": "Sound Preset, Voice and Input move into a bottom browse row. ROLL DICE joins chord-preset and slot navigation beneath it, followed by four 70 px chord pads. Original artwork and typography stay; compact main controls and guides make room. Sound engine, 291 parameters, 66 programs, detail pages and Q-Link assignments are unchanged."
    },
    {
      "title": "1.10.0: a whole new sound, one tap away.",
      "text": "ROLL DICE generates oscillators, filter and envelopes, modulation and FX within playable bounds. LFO 1, LFO 2 and shared motion Sync turn on automatically. Chords, Key, MIDI mapping, Strum, CHORD GLIDE preference and master Volume stay in place. Select any factory sound to return to a known starting point."
    },
    {
      "title": "A clearer performance surface.",
      "text": "OSC Pair/Mix/Detune/Drift; Filter/Resonance/Body/Sub; both LFOs; Drive/Glide/Motion. Four groups keep sixteen controls in a consistent order. Touch selectors and independent SYNC toggles sit beside the relevant knobs."
    },
    {
      "title": "More harmony, less top-note travel.",
      "text": "32 chord types and 96 four-chord recipes, Key/circle reference, explicit transpose and Smooth Voicing extend the editor. Inversions, octave, overlapping chord glide and one-shot Strum remain directly editable."
    },
    {
      "title": "Bring your sessions forward.",
      "text": "The original 64 sound programs keep their order, with two new additions. Sound changes preserve your chord settings and master Volume. Saved tracks may retain older Q-Link maps: reset them or compare a fresh instance."
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
      "image": "donut-theory-main-device-1.13.14.webp",
      "alt": "Actual MPC XL Donut Theory 1.13.14 with individual shared Dice locks",
      "caption": "MAIN \u00b7 Actual MPC XL1.13.14. Shared per-control Dice locks; original artwork and sixteen Q-Link assignments."
    },
    {
      "image": "donut-theory-presets-device-1.13.14.webp",
      "alt": "Actual native MPC preset list showing Dusty Sunday and Velvet Melt",
      "caption": "NATIVE PRESETS \u00b7 Actual1.13.14 list and selected factory sound. Both final factory entries work."
    },
    {
      "image": "donut-theory-env-device-1.13.14.webp",
      "alt": "Actual MPC XL Donut Theory1.13.14 ENV controls with shared Dice locks",
      "caption": "ENV \u00b7 Direct MPC XL1.13.14 capture. Shared protection for controls that Dice changes; existing Q-Link assignments retained."
    },
    {
      "image": "donut-theory-play-device-1.13.14.webp",
      "alt": "Actual MPC XL Donut Theory1.13.14 PLAY controls with shared Dice locks",
      "caption": "PLAY \u00b7 Direct MPC XL1.13.14 capture. Shared protection for controls that Dice changes; existing Q-Link assignments retained."
    },
    {
      "image": "donut-theory-chord-device-1.13.14.webp",
      "alt": "Actual Donut Theory1.13.14 chord editor",
      "caption": "CHORD \u00b7 Actual MPC XL1.13.14: key/circle reference,32types,96recipes, voicing, octave, Glide and Strum."
    },
    {
      "image": "donut-theory-osc-device-1.13.14.webp",
      "alt": "Actual MPC XL Donut Theory1.13.14 OSC controls with shared Dice locks",
      "caption": "OSC \u00b7 Direct MPC XL1.13.14 capture. Shared protection for controls that Dice changes; existing Q-Link assignments retained."
    },
    {
      "image": "donut-theory-mod-device-1.13.14.webp",
      "alt": "Actual MPC XL Donut Theory1.13.14 MOD controls with shared Dice locks",
      "caption": "MOD \u00b7 Direct MPC XL1.13.14 capture. Shared protection for controls that Dice changes; existing Q-Link assignments retained."
    },
    {
      "image": "donut-theory-fx-device-1.13.14.webp",
      "alt": "Actual MPC XL Donut Theory1.13.14 FX controls with shared Dice locks",
      "caption": "FX \u00b7 Direct MPC XL1.13.14 capture. Shared protection for controls that Dice changes; existing Q-Link assignments retained."
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
 tagline:'A new groove, one roll away.',
 summary:'Roll a musical groove. Lock what you love. Shape every drum.',
 description:'A one-bar neo-soul and hip-hop drum instrument for MPC. Roll new timing, rules, mix and cymbal accents, protect individual controls with a padlock, and build your sound from layered original drums.',
 developmentVersion:'1.6.0', release:{version:'1.6.0',tag:'groove-dice/v1.6.0',asset:'groove-dice-1.6.0-mpc.zip'}, image:'groove-dice-main-device-1.6.0.webp',
 imageAlt:'Actual MPC XL Groove Dice1.6.0 with Triplet hats, MAIN locks and sound pool controls',
 intro:'Includes 128 original synthesized one-shots, 96 kit combinations, two authored beats and 93 timing templates. Optional .gdexp packs stay separate. No commercial or personal sample library is bundled.',
 verifiedDevice:'MPC XL / OS 3.10.1.12, internal NVMe; 1.6.0 three-page Triplet browser and live remote UI checks; automated timing, VST/Q-Link bridge, state/Undo and count-in regressions; earlier SOUND/REV/polarity checks',
 mapping:[['A · Timing','Swing','Snare Late','Hat Shift','Rnd Nudge'],['B · Rules','Kick','Snare','Hat','Perc'],['C · Drum Mix','Kick Level','Snare Level','Hat Level','Perc Level'],['D · Hats / Cymbals','Hat Rhythm','Open Hat','Ride','Crash']],
 features:[
  {title:'Roll the values you can see.',text:'Sixteen MAIN padlocks decide which controls Dice may change. Open a lock to explore; close it to preserve that value. Timing and mix stay within musical bounds, with strong/weak/medium/weak velocity accents. Undo restores the preceding roll.'},
  {title:'Sixteen rules for every part.',text:'Kick, Snare, Hat and Perc each offer 16 playable rules. Choose through Q-Link or an eight-at-a-time left browser. Explore pickups, ghost notes, dynamic hats and percussion replies while keeping snare backbeats on 2 and 4 and kicks around them.'},
  {title:'Give the hats a phrase.',text:'Choose 24 straight, dotted and triplet rhythms, each with basic, Fill and Roll variants. Quarter-, eighth- and sixteenth-note triplets share the same live Hat Rhythm control. Add restrained open hats, replace automatic hats with Ride/Bell, or mark starts and fill endings with Crash. Character processing remains on FX.'},
  {title:'Find a sample. Hear it. Shape it.',text:'Tap a sound name for its browser, step with arrows or turn a Q-Link. Two in-place pages offer Sample/Level/Tune/Shift, then Start/Decay/Low Cut/High Cut. Every source has independent REV, with polarity on page2 and a single-layer preview. Four kick/snare layers and independent Ride/Crash/Bell sources keep sound design close.'},
  {title:'Choose where the next kit comes from.',text:'Use one pack, every installed pack or up to 64 selected kits. Roll a whole kit or mix parts, keeping linked hats and a four-role percussion ensemble. Kit Fixed and layer locks protect your selections.'},
  {title:'Record a fill. Keep it yours.',text:'Play physical pads into four independent fill slots, up to four bars each. Two grid views expose all eight drum roles. Main-pattern rolls preserve all recorded fills. Use ROLL FILL separately.'}
 ],
 changes:[
  {title:'1.6.0: hats in threes.',text:'Nine new Triplet choices: 1/4T, 1/8T and 1/16T, each basic, +FILL or +ROLL. The three-page browser, MAIN D1, GROOVE, Dice and Undo use the same 24-choice range. Old projects keep their notes and legacy parameter ranges. Momentary Fill/FX rolls clear after execution.'},
  {title:'1.5.1: three rolls, clear boundaries.',text:'ROLL DICE reshapes MAIN and its pattern while preserving every fill and FX value. Dedicated ROLL FILL and ROLL FX actions have Undo. Individual locks stay on MAIN; kit, role and layer protection remain. UNLOCK ALL changes protection only.'},
  {title:'1.4.0: 36 new ways to move.',text:'Nine new rules per part extend the original seven to sixteen. New choices include END DOUBLE kicks, DRAG IN snares, CRESCENDO hats and LATIN THREE percussion. Live edits and Roll Dice use the same full range, honor locks and preserve recorded fills.'},
  {title:'1.3.4: two sound pages. Every source in reach.',text:'Large previous/next arrows and a visible SOUND1/2 counter switch all rows and their SCREEN Q-Links together. Tune is on page1, Decay on page2. REV and polarity are independent for all17sources; no separate Detail screen is needed.'},
  {title:'Visible Dice, individual locks.',text:'Roll Dice updates eligible MAIN values. Separate 44px padlocks replace LOCK in the RULE selectors. Older saved LOCK states retain their frozen rhythm and migrate to BASIC with a closed padlock.'},
  {title:'A complete sound workflow.',text:'Tap-to-browse lists, synchronized arrows/Q-Link, isolated previews, Decay on every sound row and editable Ride/Crash/Bell sources. Dice pools can be limited to selected kits. The blank-skin and overlapping kit-caption defects are fixed.'},
  {title:'One repeating bar, with room to perform.',text:'Live rules, metrical velocities, hats/cymbals and independent recorded fills build on the one-bar engine. ROOM is a stereo diffused reverb. FOLLOW retains automatic record-count-in detection.'}
 ],
 limitations:['Signed Shift reports a fixed 30ms baseline latency, including direct pads. Hardware compensation and playing feel are not certified.','Remote UI checks cover page/REV/polarity operation; full physical Q-Link coverage, worst-case CPU and every output destination remain unverified.','A stopped recording intentionally started inside a bar may wait for a host bar anchor/seek; use aligned starts or punch in during running playback.','External sidechain input and generated MIDI output to the MPC sequencer are not implemented.','Install optional packs using Pack Builder or the Python CLI, then reopen the plugin. This is not native MPC XPN registration.'],
 screenshots:[
  {image:'groove-dice-main-device-1.6.0.webp',alt:'Groove Dice1.6.0 MAIN running on MPC XL with Triplet Hat Rhythm',caption:'MAIN · Direct MPC XL1.6.0 capture. This device includes optional owner packs; the public ZIP contains only the 128 original sounds.'},
  {image:'groove-dice-triplets-device-1.6.0.webp',alt:'Actual MPC XL Hat Rhythm page3/3 showing Triplet Fill and Roll choices',caption:'TRIPLET HATS · Three pages, 24 choices. Quarter, eighth and sixteenth triplets include Fill and Roll endings. Actual MPC XL1.6.0.'},
  {image:'groove-dice-fill-device-1.5.1.webp',alt:'Actual MPC fill editor with separate ROLL FILL',caption:'ROLL FILL · Vary only the selected fill; Undo restores it. Actual MPC XL1.5.1.'},
  {image:'groove-dice-fx-device-1.5.1.webp',alt:'Actual MPC FX page with separate ROLL FX',caption:'ROLL FX · Independent texture exploration and Undo. Main Dice preserves these values. Actual MPC XL1.5.1.'},
  {image:'groove-dice-rules-device-1.4.0.webp',alt:'Actual MPC Groove Dice Kick Rules second page with eight additional choices',caption:'RULES · Eight choices per page, with PAGE 1/2 and PAGE 2/2 controls. All sixteen choices are also available through Q-Link.'},
  {image:'groove-dice-sound-device-1.5.1.webp',alt:'Actual MPC Groove Dice SOUND1/2 page with Sample Level Tune Shift and per-row reverse buttons',caption:'SOUND1/2 · Sample, Level, Tune and Shift. Each row keeps one SCREEN Q-Link group; previous/next arrows and the page counter stay visible.'},
  {image:'groove-dice-sound-page-2-device-1.3.4.webp',alt:'Actual MPC SOUND2/2 with Start Decay Low Cut High Cut and polarity switches',caption:'SOUND2/2 · Start, Decay, Low Cut and High Cut for each source. REV stays available; Ø controls polarity independently.'},
  {image:'groove-dice-browser-device-1.3.4.webp',alt:'Actual Groove Dice sample browser on MPC, filtered to Original Workshop',caption:'SAMPLE BROWSER · Tap the name, choose a source and preview it. Pack filters, arrows and SCREEN selection share the same catalogue.'},
  {image:'groove-dice-cymbals-device-1.3.4.webp',alt:'Actual MPC Ride Crash and Bell source editor in Groove Dice',caption:'CYMBALS · Ride, Crash and Bell use the same two-page layout, with separate sources, reverse and polarity.'},
  {image:'groove-dice-fill-device-1.3.4.webp',alt:'Actual MPC Groove Dice fill editor showing snare tom percussion and clap rows',caption:'FILL EDITOR · Snare, Tom, Perc and Clap. Switch views for Kick, Closed Hat, Open Hat and Cymbal.'}
 ]
}];
export const repo = 'https://github.com/Sunghyun23/kimswork';
export const navigationVersion = 'software-2026-11-19-triplet';
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
