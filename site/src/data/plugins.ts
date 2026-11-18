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
 developmentVersion:'0.2.3',release:{version:'0.2.3',tag:'crate-press/v0.2.3',asset:'crate-press-0.2.3-mpc.zip',preview:true},image:'crate-press-main-0.2.2.webp',imageAlt:'Actual Crate Press 0.2.2 master insert on MPC Live III with compact section titles and eight performance buttons',manual:'plugins/crate-press/manual/',
 intro:'Start with one of 32 programs, then use four Q-Link groups to shape the sound. Hold an effect for a brief change, or enable LATCH and combine independent tap toggles.',
 verifiedDevice:'MPC Live III / OS 3.10.1.12; partial hardware verification',
 mapping:[['A · Comp / Drive','Drive','Compress','Punch','Release'],['B · Band / Duck','Low Comp','Mid Comp','High Comp','Duck'],['C · Color','Tone','High Cut','Lo-Fi','Drift'],['D · I/O / Mix','Input','Low Cut','Mix','Output']],
 features:[{title:'Compression in context.',text:'Full-band character compression and a three-band crossover let you control low, mid and high energy separately. Internal bass sensing or host-tempo pulses can duck selected bands.'},{title:'Eight effects, close at hand.',text:'Tape Stop, Reverse, Repeat, Dub Echo, Filter Dip, Crush, Wobble and Dropout can overlap. LATCH keeps independent effects engaged; turn it off to release them all.'},{title:'See the signal.',text:'MAIN permanently shows measured input/output stereo extrema, a static compressor transfer curve and gain-reduction meters. No decorative waveform animation.'},{title:'A practical starting point.',text:'32 programs cover Init, Drums, Keys, Samples, Master and Performance. Factory programs preserve your performance timing and latch preference.'}],
 changes:[{title:'0.2.3 maintenance update.',text:'Refreshed package metadata, internal-storage installation guidance and regression checks. DSP, UI, 32 programs and parameter identities are unchanged from 0.2.2. This remains a public preview.'},{title:'Quieter typography.',text:'Smaller 18 px section lettering and lighter reference-derived captions keep the controls prominent. Numeric readouts and detail-page choice captions were corrected against the running MPC interface.'},{title:'Combination testing.',text:'All 28 effect pairs were checked at three sample rates against both solo effects, with independent release and echo-tail checks. Hardware multi-touch and worst-case CPU are not certified.'}],
 limitations:['Public preview: hardware verification is partial; read the manual before using it in a critical live set.','External sidechain input is not implemented. Ducking uses internal bass detection or host tempo.','Real-device checks cover master-insert loading, project recall, visible controls, tab navigation and independent latched button states. Physical Q-Link rotation, simultaneous finger touches and track-insert audio remain unverified.','Automated audio tests pass; subjective listening and full hardware audio/latency compensation and worst-case CPU certification remain outstanding.','The transfer curve is static and pre-makeup, not the combined nonlinear effect transfer. The waveform is a 0.77-second extrema overview.','Peak Guard is a soft peak bound, not a true-peak limiter. Captured performance history is limited to eight seconds.'],
 screenshots:[{image:'crate-press-main-0.2.2.webp',alt:'Crate Press MAIN running on MPC Live III',caption:'MAIN · Actual MPC Live III capture, 0.2.2. Compact section headings, live numeric values and eight performance buttons.'},{image:'crate-press-perform-0.2.2.webp',alt:'Crate Press PERFORM settings running on MPC Live III',caption:'PERFORM · Actual MPC capture. Independent timing, feedback, texture and release controls.'}]
},
{
  "slug": "donut-theory",
  "name": "Donut Theory",
  "category": "POLYPHONIC SYNTHESIZER",
  "tagline": "A little worn. Endlessly warm.",
  "summary": "Warm synthesis. Rich chords. Movement in every direction.",
  "description": "A Moog-inspired synthesizer for MPC bass, keys, leads and pads. Shape its nonlinear ladder engine with Body and Sub, two independent LFOs and synchronized Tremolo or Auto Pan, then build your own chord changes.",
  "developmentVersion": "1.9.0",
  "release": {
    "version": "1.9.0",
    "tag": "donut-theory/v1.9.0",
    "asset": "donut-theory-1.9.0-mpc.zip"
  },
  "image": "donut-theory-main-device-1.9.0.webp",
  "imageAlt": "Donut Theory1.9.0 running on MPC XL with Dusty Sunday, two LFOs, Body, Sub and synchronized Auto Pan",
  "manual": "plugins/donut-theory/manual/",
  "intro": "66 synthesized factory sounds, 32 chord types and 96 progression recipes. The new Dusty Sunday and Velvet Melt sounds bring wide Auto Pan character into the factory bank. No external samples are required.",
  "verifiedDevice": "MPC XL / OS 3.10.1.12, internal NVMe; actual1.9.0 remote UI checks and runtime read-back",
  "mapping": [
    [
      "A · OSC / Sound",
      "OSC Pair",
      "OSC Mix",
      "Detune",
      "Drift"
    ],
    [
      "B · Filter / Tone",
      "Filter Cutoff",
      "Resonance",
      "Body",
      "Sub"
    ],
    [
      "C · LFO 1 /2",
      "LFO 1 Rate",
      "LFO 1 Depth",
      "LFO 2 Rate",
      "LFO 2 Depth"
    ],
    [
      "D · FX / Motion",
      "Drive",
      "Glide",
      "Mod Rate",
      "Mod Depth"
    ]
  ],
  "features": [
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
      "title": "1.9.0: two new voices, built in.",
      "text": "Keys / Dusty Sunday and Synth / Velvet Melt join the factory bank at programs 65/66. Both use eighth-note Auto Pan at 70%/80% depth. Set MPC to 88 BPM for the audition feel; sound selection preserves your chords, Strum, CHORD GLIDE and master Volume."
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
    "The engine changed in 1.2.0 and explicit Sub in 1.2.3; retained parameter identities do not guarantee identical older project sounds."
  ],
  "screenshots": [
    {
      "image": "donut-theory-main-device-1.9.0.webp",
      "alt": "Actual MPC XL 1.9.0 MAIN with Dusty Sunday and independent modulation",
      "caption": "MAIN · Actual 1.9.0 MPC XL capture. Four control groups and the Dusty Sunday factory sound."
    },
    {
      "image": "donut-theory-chord-device-1.9.0.webp",
      "alt": "Actual MPC XL Donut Theory chord editor with key reference and circle of fifths",
      "caption": "CHORD · Four editable slots, Key/circle reference, transpose, Smooth Voicing, chord glide and Strum."
    },
    {
      "image": "donut-theory-browser-device-1.9.0.webp",
      "alt": "Actual Donut Theory category-first sound browser running on MPC XL",
      "caption": "SOUND PRESET · Choose a category, then a sound. The host PRESET list also contains all 66 program names."
    },
    {
      "image": "donut-theory-mod-device-1.9.0.webp",
      "alt": "Donut Theory MOD detail with two independent LFOs on MPC XL",
      "caption": "MOD · Independent LFOs, routing and tempo settings. MAIN keeps both rates and depths close."
    },
    {
      "image": "donut-theory-amp-device-1.9.0.webp",
      "alt": "Donut Theory AMP detail running on MPC XL",
      "caption": "AMP · Amp ADSR and playing controls live on their dedicated page."
    },
    {
      "image": "donut-theory-fx-device-1.9.0.webp",
      "alt": "Actual Donut Theory FX detail with Auto Pan and stereo balance",
      "caption": "FX · Drive, chorus, delay, cleanup Low-cut and shared Tremolo/Auto Pan with static Pan."
    }
  ],
  "auditions": [
    {
      "name": "Keys / Dusty Sunday",
      "file": "donut-theory-dusty-sunday-1.9.0.wav",
      "description": "Soft sine/triangle keys ·8 ms Strum ·70% Auto Pan."
    },
    {
      "name": "Synth / Velvet Melt",
      "file": "donut-theory-velvet-melt-1.9.0.wav",
      "description": "Soft saw/triangle ·6 ms Strum ·65 ms chord glide ·80% Auto Pan."
    }
  ]
},
{
 slug:'groove-dice', name:'Groove Dice',manual:'plugins/groove-dice/manual/', category:'DRUM & GROOVE INSTRUMENT',
 tagline:'A new groove, one roll away.',
 summary:'Roll a musical groove. Lock what you love. Shape every drum.',
 description:'A one-bar neo-soul and hip-hop drum instrument for MPC. Roll new timing, rules, mix and cymbal accents, protect individual controls with a padlock, and build your sound from layered original drums.',
 developmentVersion:'1.3.4', release:{version:'1.3.4',tag:'groove-dice/v1.3.4',asset:'groove-dice-1.3.4-mpc.zip'}, image:'groove-dice-main-device-1.3.4.webp',
 imageAlt:'Groove Dice 1.3.4 MAIN captured directly from MPC XL with sixteen individual Dice locks',
 intro:'Includes 128 original synthesized one-shots, 96 kit combinations, two authored beats and 93 timing templates. Optional .gdexp packs stay separate. No commercial or personal sample library is bundled.',
 verifiedDevice:'MPC XL / OS 3.10.1.12, internal NVMe;1.3.4 remote page/REV/polarity checks and earlier owner Dice/RULE confirmation',
 mapping:[['A · Timing','Swing','Snare Late','Hat Shift','Rnd Nudge'],['B · Rules','Kick','Snare','Hat','Perc'],['C · Drum Mix','Kick Level','Snare Level','Hat Level','Perc Level'],['D · Hats / Cymbals','Hat Rhythm','Open Hat','Ride','Crash']],
 features:[
  {title:'Roll the values you can see.',text:'Sixteen MAIN padlocks decide which controls Dice may change. Open a lock to explore; close it to preserve that value. Timing and mix stay within musical bounds, with strong/weak/medium/weak velocity accents. Undo restores the preceding roll.'},
  {title:'Seven styles. A separate lock.',text:'Each part has seven playable rules, with no LOCK value in the knob range. Snare backbeats stay on 2 and 4, kicks work around them, and live edits reshape their part without another roll.'},
  {title:'Give the hats a phrase.',text:'Choose 15 straight, dotted, fill and roll rhythms. Add restrained open hats, replace automatic hats with Ride/Bell, or mark starts and fill endings with Crash. Character processing remains on FX.'},
  {title:'Find a sample. Hear it. Shape it.',text:'Tap a sound name for its browser, step with arrows or turn a Q-Link. Two in-place pages offer Sample/Level/Tune/Shift, then Start/Decay/Low Cut/High Cut. Every source has independent REV, with polarity on page2 and a single-layer preview. Four kick/snare layers and independent Ride/Crash/Bell sources keep sound design close.'},
  {title:'Choose where the next kit comes from.',text:'Use one pack, every installed pack or up to 64 selected kits. Roll a whole kit or mix parts, keeping linked hats and a four-role percussion ensemble. Kit Fixed and layer locks protect your selections.'},
  {title:'Record a fill. Keep it yours.',text:'Play physical pads into four independent fill slots, up to four bars each. Two grid views expose all eight drum roles. Main-pattern rolls preserve recorded fills unless you explicitly include Fill.'}
 ],
 changes:[
  {title:'1.3.4: two sound pages. Every source in reach.',text:'Large previous/next arrows and a visible SOUND1/2 counter switch all rows and their SCREEN Q-Links together. Tune is on page1, Decay on page2. REV and polarity are independent for all17sources; no separate Detail screen is needed.'},
  {title:'Visible Dice, individual locks.',text:'Roll Dice updates eligible MAIN values. Separate 44px padlocks replace LOCK in the seven-style RULE selectors. Older saved LOCK states retain their frozen rhythm and migrate to BASIC with a closed padlock.'},
  {title:'A complete sound workflow.',text:'Tap-to-browse lists, synchronized arrows/Q-Link, isolated previews, Decay on every sound row and editable Ride/Crash/Bell sources. Dice pools can be limited to selected kits. The blank-skin and overlapping kit-caption defects are fixed.'},
  {title:'One repeating bar, with room to perform.',text:'Live rules, metrical velocities, hats/cymbals and independent recorded fills build on the one-bar engine. ROOM is a stereo diffused reverb. FOLLOW retains automatic record-count-in detection.'}
 ],
 limitations:['Signed Shift reports a fixed 30ms baseline latency, including direct pads. Hardware compensation and playing feel are not certified.','Remote UI checks cover page/REV/polarity operation; full physical Q-Link coverage, worst-case CPU and every output destination remain unverified.','A stopped recording intentionally started inside a bar may wait for a host bar anchor/seek; use aligned starts or punch in during running playback.','External sidechain input and generated MIDI output to the MPC sequencer are not implemented.','Install optional packs using Pack Builder or the Python CLI, then reopen the plugin. This is not native MPC XPN registration.'],
 screenshots:[
  {image:'groove-dice-main-device-1.3.4.webp',alt:'Groove Dice1.3.4 MAIN running on MPC XL with timing, rules, drum mix and hats/cymbals',caption:'MAIN · Direct MPC XL1.3.4 capture, with Original Workshop selected. This owner device also has optional packs; the public ZIP contains only128original sounds.'},
  {image:'groove-dice-sound-device-1.3.4.webp',alt:'Actual MPC Groove Dice SOUND1/2 page with Sample Level Tune Shift and per-row reverse buttons',caption:'SOUND1/2 · Sample, Level, Tune and Shift. Each row keeps one SCREEN Q-Link group; previous/next arrows and the page counter stay visible.'},
  {image:'groove-dice-sound-page-2-device-1.3.4.webp',alt:'Actual MPC SOUND2/2 with Start Decay Low Cut High Cut and polarity switches',caption:'SOUND2/2 · Start, Decay, Low Cut and High Cut for each source. REV stays available; Ø controls polarity independently.'},
  {image:'groove-dice-browser-device-1.3.4.webp',alt:'Actual Groove Dice sample browser on MPC, filtered to Original Workshop',caption:'SAMPLE BROWSER · Tap the name, choose a source and preview it. Pack filters, arrows and SCREEN selection share the same catalogue.'},
  {image:'groove-dice-cymbals-device-1.3.4.webp',alt:'Actual MPC Ride Crash and Bell source editor in Groove Dice',caption:'CYMBALS · Ride, Crash and Bell use the same two-page layout, with separate sources, reverse and polarity.'},
  {image:'groove-dice-fill-device-1.3.4.webp',alt:'Actual MPC Groove Dice fill editor showing snare tom percussion and clap rows',caption:'FILL EDITOR · Snare, Tom, Perc and Clap. Switch views for Kick, Closed Hat, Open Hat and Cymbal.'}
 ]
}];
export const repo = 'https://github.com/Sunghyun23/kimswork';
export const navigationVersion = 'software-2026-11-18-home-deduplicated';
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
