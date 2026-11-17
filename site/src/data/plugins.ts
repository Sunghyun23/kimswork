export interface Plugin {
 slug: string; name: string; kind?: 'instrument' | 'effect'; category: string; tagline: string; description: string; summary: string;
 developmentVersion: string; image: string; imageAlt: string; intro: string; mapping: string[][]; limitations: string[]; verifiedDevice: string; features: {title: string; text: string}[];
 changes: {title:string; text:string}[]; manual?: string; screenshots: {image:string; alt:string; caption:string}[];
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
 slug: 'donut-theory', name: 'Donut Theory', category: 'POLYPHONIC SYNTHESIZER',
 tagline: 'A little worn. Endlessly warm.',
 summary: 'A Moog-inspired synthesis engine, warm factory sounds and sixteen playable chord slots for MPC.',
 description: 'A nonlinear oscillator mixer, four-stage ladder low-pass and rounded VCA now shape the sound at its source. Explore 63 factory voices rebuilt for this engine, plus a dry Init, then play bass, leads and your own chord changes inside your MPC.',
 developmentVersion: '1.2.3', image: 'donut-theory-main-1.2.2.webp', release: {version:'1.2.3',tag:'donut-theory/v1.2.3',asset:'donut-theory-1.2.3-mpc.zip'},
 imageAlt: 'Donut Theory running on MPC Live III, showing its four performance control groups and large chord pads',
 manual: 'plugins/donut-theory/manual/',
 intro: 'Sixteen oscillator pairs, hands-on envelopes and four Q-Link banks keep the sound close. Start with one of 64 sound presets, or build a chord arrangement across four banks of four slots.',
 verifiedDevice: 'MPC XL internal installation; earlier MPC Live III audio/UI checks, OS 3.10.1.12',
 mapping: [ ['A · Sound / Play','OSC Pair','OSC Mix','Sub','Drift'], ['B · Amp Envelope','Attack','Decay','Sustain','Release'], ['C · Filter / LFO','Filter Cutoff','Resonance','LFO Rate','LFO Depth'], ['D · FX / Tremolo','Drive','Glide','Tremolo Rate','Tremolo Depth'] ],
 limitations: [
  'SUB spectral and parameter/skin tests pass; a dedicated MPC XL physical SUB knob/listening matrix has not been confirmed.',
  'Sixteen chord slots share a sixteen-voice engine; overlapping chords can steal older voices.',
  'Screen chord pads audition the instrument; they do not send MIDI notes to the MPC sequencer.',
  'Use Q-Link Screen mode for chord editing. Program/Track mode retains the synth mapping.',
  'The MPC’s gray host PRESET list is flat. Use the instrument’s SOUND PRESET browser for category-first selection.',
  'Manual pad-location hints have been removed in 1.2.2. Use each slot’s MIDI Note / LEARN to connect a physical pad; saved MIDI assignments are unchanged.',
  'MPC Live III hardware checks cover all 64 program changes and audible screen-chord gates, filter sweeps and native preset reload after restart. The overlapping-release CPU test observed 41%; this is not a worst-case guarantee.',
  'Remote Viewer reported audio-output dropouts during testing; their source was not isolated. Artifact-free hardware output and a full MPC project round-trip are not certified.',
  'The new engine changes existing project sounds. Keep 1.1.2 for sound-identical recall. Testing on other MPC models, simultaneous finger multi-touch and every physical Q-Link rotation is not certified.'
 ],
 changes: [
  {title:'1.2.3: bass weight within reach.',text:'Sub replaces Body beside OSC Mix. Its independent sine plays exactly one octave below OSC1 through the shared filter and amp envelope. Factory programs migrate the former Body mix/sub contributions into explicit controls; old user Body automation may sound different.'},
  {title:'1.2.2: a lighter chord editor.',text:'Removed manual pad-location hint controls after reports of stalled MAIN-to-CHORD navigation. The expanded CHORD widget tree shrinks from 10,889 to 1,913. Repeated remote tab switching, inversion selection and sixteen-slot navigation were checked on MPC Live III. MIDI Note, LEARN and saved assignments remain.'},
  {title:'A new engine, from mixer to VCA.',text:'Version 1.2.0 replaces the low-pass voice path with a four-stage nonlinear ladder, a saturating oscillator summer and level-dependent VCA rounding. This is the default engine, with no extra mode to enable. The existing band-limited oscillators, envelopes and effects remain part of the instrument.'},
  {title:'Resonance that responds to level.',text:'Nonlinear stages and a simultaneous feedback solve make resonance react to the signal. The ladder runs internally at twice the sample rate with output filtering. Higher resonance naturally reduces low-end gain; its top is limited below autonomous self-oscillation.'},
  {title:'Every playable factory sound refitted.',text:'All 63 Bass, Keys, Synth, Lead and Pad recipes have revised cutoff, resonance and filter-envelope settings, gentler external drive and remeasured output trims. Names, categories, program order and individual low-cut settings remain. Init keeps low-cut and effects off, while using the new engine.'},
  {title:'The familiar performance surface.',text:'The UI, 208 parameter identities, synth and CHORD Q-Link mappings, enlarged chord arrows are retained. Band-pass, High-pass and Notch remain 12 dB SVF extensions; they are not ladder variants.'},
  {title:'Saved settings load. Their sound changes.',text:'Existing projects pass through the new mixer, filter and VCA, so parameter compatibility does not mean identical sound. Keep the 1.1.2 package and render tracks before upgrading if you need their old sound. The new engine costs more CPU; full hardware load certification is listed separately.'}
 ],
 screenshots: [
  {image:'donut-theory-main-1.2.2.webp',alt:'Donut Theory 1.2.2 running on MPC Live III with four synthesis groups and large screen chord pads',caption:'MAIN · Actual v1.2.2 MPC Live III capture. In 1.2.3, Sub replaces the pictured Body control; this is not a fresh 1.2.3 capture.'},
  {image:'donut-theory-chords-1.2.2.webp',alt:'Actual Donut Theory 1.2.2 editor with all sixteen slots enabled and bank 13 to 16 selected',caption:'CHORD · Sixteen slots, four at a time. Root, type, inversions, octave and individual MIDI assignments share one editor.'},
  {image:'donut-theory-browser-1.2.2.webp',alt:'Actual Donut Theory 1.2.2 category-first sound browser over the MAIN screen',caption:'SOUND PRESET · Choose a category first. The left sidebar covers underlying chord arrows and hides Q-Link guides.'}
 ],
 features: [
  { title: '64 starting points. Six familiar categories.', text: 'The ladder engine has 63 individually refitted factory sounds and a dry Init. Browse Bass, Keys, Synth, Lead and Pad. Per-sound low-cut and output trim provide practical starting levels.' },
  { title: 'A little movement goes a long way.', text: 'Drift adds slow wow and faster flutter. Filter LFO and independent tremolo have separate rate and depth controls, alongside a complete amp ADSR.' },
  { title: 'Your changes, across sixteen slots.', text: 'Start with four chords and expand to eight, twelve or sixteen. Keep four large touch pads on MAIN; assign physical pads with LEARN. Each bank keeps its own chord-preset selection.' },
  { title: 'Shape the voicing. Keep the groove.', text: 'Choose the available inversions for each chord, Open voicing and an independent ±2-octave offset. The 32 four-chord presets are starting points, and affect only the bank you are viewing.' },
  { title: 'Weight, warmth and space.', text: 'Combine Tape, Tube or Fuzz character with chorus and tempo delay. Filter cutoff and resonance sit beside the LFO that moves them.' },
  { title: 'An MPC instrument, from the first screen.', text: 'Play keyboard notes, chord triggers or both in Split mode. Mono and Legato apply to melody notes; chord pads keep their full voicings.' }
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
export const navigationVersion = 'software-2026-11-18-groove-dice-134';
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
