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
  "category": "POLYPHONIC & MONOPHONIC SYNTHESIZER",
  "tagline": "A little worn. Endlessly warm.",
  "summary": "Warm synthesis. Rich chords. Movement in every direction.",
  "description": "A Moog-inspired synthesizer for MPC bass, keys, leads and pads. Edit one shared sound from POLY or MONO without changing its mode, shape each oscillator and delay either LFO, then build your own chord changes.",
  "developmentVersion": "1.14.2",
  "release": {
    "version": "1.14.2",
    "tag": "donut-theory/v1.14.2",
    "asset": "donut-theory-1.14.2-mpc.zip"
  },
  "image": "donut-theory-poly-device-1.14.2.webp",
  "imageAlt": "Actual MPC XL Donut Theory 1.14.2 POLY performance editor",
  "manual": "plugins/donut-theory/manual/",
  "intro": "66 synthesized factory sounds, 32 chord types and 96 progression recipes. Tap ROLL DICE to generate a complete oscillator-to-FX sound with synchronized motion, while keeping your chords and master level. No external samples are required.",
  "verifiedDevice": "MPC XL / OS 3.10.1.12; remote POLY/MONO, shared values, native presets and Sync drawers checked; native VST/state/audio tests pass. Physical encoder/hardware-audio/CPU coverage remains separate.",
  "mapping": [
    [
      "A \u00b7 Selected OSC",
      "Wave",
      "Pitch \u00b7 Tune/Fine",
      "Level",
      "Drift"
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
      "text": "Select OSC1 or OSC2 for Wave, Pitch and Level. Tune and Fine add together; Drift remains shared. OSC exposes both sources, pulse widths, Sub, noise, FM and Tone."
    },
    {
      "title": "Motion that arrives in time.",
      "text": "Both LFOs have independent Rate, Depth, Delay and Target. Per-note Delay waits up to five seconds before modulation fades in. Each LFO and Tremolo/Auto Pan offers 15 straight, dotted and triplet Sync periods; small knob turns accumulate normally."
    },
    {
      "title": "Explore. Protect. Return.",
      "text": "61 shared value locks protect actual sound parameters across views. Roll a whole sound within a chosen family, Undo the generation or Unlock All without changing a note. Actual mode, harmony, MIDI, master and both LFO Delays remain intact."
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
      "title": "1.14.2: selectors that line up.",
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
      "image": "donut-theory-poly-device-1.14.2.webp",
      "alt": "Donut Theory POLY editor on MPC XL",
      "caption": "POLY \u00b7 Selected OSC and LFO controls with Amp/Motion; one shared sound. Actual MPC XL / OS 3.10.1.12, version 1.14.2."
    },
    {
      "image": "donut-theory-mono-device-1.14.2.webp",
      "alt": "Donut Theory MONO editor on MPC XL",
      "caption": "MONO \u00b7 Glide, Decay, Release and Pitch Drop. The editor tab does not change the actual voice mode. Actual MPC XL / OS 3.10.1.12, version 1.14.2."
    },
    {
      "image": "donut-theory-osc-device-1.14.2.webp",
      "alt": "Donut Theory independent OSC controls on MPC XL",
      "caption": "OSC \u00b7 Individual Wave, Tune, Fine, Level and Pulse Width, plus texture controls. Actual MPC XL / OS 3.10.1.12, version 1.14.2."
    },
    {
      "image": "donut-theory-env-device-1.14.2.webp",
      "alt": "Donut Theory combined ADSR screen on MPC XL",
      "caption": "ENV \u00b7 Both editable ADSRs and filter response; Amp Release is shared with the fronts. Actual MPC XL / OS 3.10.1.12, version 1.14.2."
    },
    {
      "image": "donut-theory-mod-device-1.14.2.webp",
      "alt": "Donut Theory delayed LFOs and Motion on MPC XL",
      "caption": "MOD \u00b7 Both LFOs, independent Delay and shared Tremolo/Auto Pan. Actual MPC XL / OS 3.10.1.12, version 1.14.2."
    },
    {
      "image": "donut-theory-fx-device-1.14.2.webp",
      "alt": "Donut Theory FX and output on MPC XL",
      "caption": "FX \u00b7 Drive/color, space/echo and output with shared sound locks. Actual MPC XL / OS 3.10.1.12, version 1.14.2."
    },
    {
      "image": "donut-theory-chord-device-1.14.2.webp",
      "alt": "Donut Theory CHORD editor and circle of fifths on MPC XL",
      "caption": "CHORD \u00b7 Root, type, voicing and octave for each visible slot, with key reference and harmony tools. Actual MPC XL / OS 3.10.1.12, version 1.14.2."
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
 developmentVersion:'1.6.1', release:{version:'1.6.1',tag:'groove-dice/v1.6.1',asset:'groove-dice-1.6.1-mpc.zip'}, image:'groove-dice-main-device-1.6.0.webp',
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
  {title:'1.6.1: your own packs, documented.',text:'Includes a complete Pack Builder workflow and clear MIT notices for the plugin and original core content. Pack Builder is a separate optional desktop app for macOS Apple Silicon and Windows x64. Engine, UI, 782 parameters, Q-Link maps and the core bank stay unchanged.'},
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
export const navigationVersion = 'software-2026-11-19-groove-161';
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
