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
 developmentVersion:'0.2.2',release:{version:'0.2.2',tag:'crate-press/v0.2.2',asset:'crate-press-0.2.2-mpc.zip',preview:true},image:'crate-press-main-0.2.2.webp',imageAlt:'Actual Crate Press 0.2.2 master insert on MPC Live III with compact section titles and eight performance buttons',manual:'plugins/crate-press/manual/',
 intro:'Start with one of 32 programs, then use four Q-Link groups to shape the sound. Hold an effect for a brief change, or enable LATCH and combine independent tap toggles.',
 verifiedDevice:'MPC Live III / OS 3.10.1.12; partial hardware verification',
 mapping:[['A · Comp / Drive','Drive','Compress','Punch','Release'],['B · Band / Duck','Low Comp','Mid Comp','High Comp','Duck'],['C · Color','Tone','High Cut','Lo-Fi','Drift'],['D · I/O / Mix','Input','Low Cut','Mix','Output']],
 features:[{title:'Compression in context.',text:'Full-band character compression and a three-band crossover let you control low, mid and high energy separately. Internal bass sensing or host-tempo pulses can duck selected bands.'},{title:'Eight effects, close at hand.',text:'Tape Stop, Reverse, Repeat, Dub Echo, Filter Dip, Crush, Wobble and Dropout can overlap. LATCH keeps independent effects engaged; turn it off to release them all.'},{title:'See the signal.',text:'MAIN permanently shows measured input/output stereo extrema, a static compressor transfer curve and gain-reduction meters. No decorative waveform animation.'},{title:'A practical starting point.',text:'32 programs cover Init, Drums, Keys, Samples, Master and Performance. Factory programs preserve your performance timing and latch preference.'}],
 changes:[{title:'First public preview.',text:'0.2.2 introduces the stereo effect, eight performance gates and global LATCH. Manufacturer metadata remains kim; the visible design has no manufacturer badge.'},{title:'Quieter typography.',text:'Smaller 18 px section lettering and lighter reference-derived captions keep the controls prominent. Numeric readouts and detail-page choice captions were corrected against the running MPC interface.'},{title:'Combination testing.',text:'All 28 effect pairs were checked at three sample rates against both solo effects, with independent release and echo-tail checks. Hardware multi-touch and worst-case CPU are not certified.'}],
 limitations:['Public preview: hardware verification is partial; read the manual before using it in a critical live set.','External sidechain input is not implemented. Ducking uses internal bass detection or host tempo.','Real-device checks cover master-insert loading, project recall, visible controls, tab navigation and independent latched button states. Physical Q-Link rotation, simultaneous finger touches and track-insert audio remain unverified.','Automated audio tests pass; subjective listening and full hardware audio/latency compensation and worst-case CPU certification remain outstanding.','The transfer curve is static and pre-makeup, not the combined nonlinear effect transfer. The waveform is a 0.77-second extrema overview.','Peak Guard is a soft peak bound, not a true-peak limiter. Captured performance history is limited to eight seconds.'],
 screenshots:[{image:'crate-press-main-0.2.2.webp',alt:'Crate Press MAIN running on MPC Live III',caption:'MAIN · Actual MPC Live III capture, 0.2.2. Compact section headings, live numeric values and eight performance buttons.'},{image:'crate-press-perform-0.2.2.webp',alt:'Crate Press PERFORM settings running on MPC Live III',caption:'PERFORM · Actual MPC capture. Independent timing, feedback, texture and release controls.'}]
},
{
 slug: 'donut-theory', name: 'Donut Theory', category: 'POLYPHONIC SYNTHESIZER',
 tagline: 'A little worn. Endlessly warm.',
 summary: 'A Moog-inspired synthesis engine, warm factory sounds and sixteen playable chord slots for MPC.',
 description: 'A nonlinear oscillator mixer, four-stage ladder low-pass and rounded VCA now shape the sound at its source. Explore 63 factory voices rebuilt for this engine, plus a dry Init, then play bass, leads and your own chord changes inside your MPC.',
 developmentVersion: '1.2.2', image: 'donut-theory-main-1.2.2.webp', release: {version:'1.2.2',tag:'donut-theory/v1.2.2',asset:'donut-theory-1.2.2-mpc.zip'},
 imageAlt: 'Donut Theory running on MPC Live III, showing its four performance control groups and large chord pads',
 manual: 'plugins/donut-theory/manual/',
 intro: 'Sixteen oscillator pairs, hands-on envelopes and four Q-Link banks keep the sound close. Start with one of 64 sound presets, or build a chord arrangement across four banks of four slots.',
 verifiedDevice: 'MPC Live III / OS 3.10.1.12',
 mapping: [ ['A · Sound / Play','OSC Pair','OSC Mix','Body','Drift'], ['B · Amp Envelope','Attack','Decay','Sustain','Release'], ['C · Filter / LFO','Filter Cutoff','Resonance','LFO Rate','LFO Depth'], ['D · FX / Tremolo','Drive','Glide','Tremolo Rate','Tremolo Depth'] ],
 limitations: [
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
  {title:'1.2.2: a lighter chord editor.',text:'Removed manual pad-location hint controls after reports of stalled MAIN-to-CHORD navigation. The expanded CHORD widget tree shrinks from 10,889 to 1,913. Repeated remote tab switching, inversion selection and sixteen-slot navigation were checked on MPC Live III. MIDI Note, LEARN and saved assignments remain.'},
  {title:'A new engine, from mixer to VCA.',text:'Version 1.2.0 replaces the low-pass voice path with a four-stage nonlinear ladder, a saturating oscillator summer and level-dependent VCA rounding. This is the default engine, with no extra mode to enable. The existing band-limited oscillators, envelopes and effects remain part of the instrument.'},
  {title:'Resonance that responds to level.',text:'Nonlinear stages and a simultaneous feedback solve make resonance react to the signal. The ladder runs internally at twice the sample rate with output filtering. Higher resonance naturally reduces low-end gain; its top is limited below autonomous self-oscillation.'},
  {title:'Every playable factory sound refitted.',text:'All 63 Bass, Keys, Synth, Lead and Pad recipes have revised cutoff, resonance and filter-envelope settings, gentler external drive and remeasured output trims. Names, categories, program order and individual low-cut settings remain. Init keeps low-cut and effects off, while using the new engine.'},
  {title:'The familiar performance surface.',text:'The UI, 208 parameter identities, synth and CHORD Q-Link mappings, enlarged chord arrows are retained. Band-pass, High-pass and Notch remain 12 dB SVF extensions; they are not ladder variants.'},
  {title:'Saved settings load. Their sound changes.',text:'Existing projects pass through the new mixer, filter and VCA, so parameter compatibility does not mean identical sound. Keep the 1.1.2 package and render tracks before upgrading if you need their old sound. The new engine costs more CPU; full hardware load certification is listed separately.'}
 ],
 screenshots: [
  {image:'donut-theory-main-1.2.2.webp',alt:'Donut Theory 1.2.2 running on MPC Live III with four synthesis groups and large screen chord pads',caption:'MAIN · Actual v1.2.2 MPC Live III capture. The engine is active; no new mode is required.'},
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
 summary:'Original drums, rule-guided grooves and fill recording for MPC.',
 description:'Shape neo-soul and hip-hop drums with timing controls, role-specific rhythm rules and independent kick, snare, hat and percussion-set choices.',
 developmentVersion:'1.0.0', release:{version:'1.0.0',tag:'groove-dice/v1.0.0',asset:'groove-dice-1.0.0-mpc.zip'}, image:'groove-dice-main-1.0.0.webp',
 imageAlt:'Actual Groove Dice 0.9.0 MAIN screen on MPC Live III with its original core bank',
 intro:'The original core includes 128 synthesized one-shots, 96 curated kit combinations, two beats and 93 timing templates. Independent expansion files add sounds without rebuilding the engine.',
 verifiedDevice:'MPC Live III / OS 3.10.1.12; partial device verification',
 mapping:[['A · Timing','Swing','Snare Late','Hat Shift','Rnd Nudge'],['B · Rules','Kick','Snare','Hat','Perc'],['C · Drum Sounds','Kick Select','Snare Select','Hat Select','Perc Select'],['D · Character','Drive','Dust','Room','Hat Rhythm']],
 features:[{title:'Shape a musical phrase.',text:'Rules protect the backbeat and guide kick placement. Straight and dotted hat divisions support short fills and rolls.'},{title:'Record your own fills.',text:'Capture physical-pad MIDI into four fill slots and edit timing, velocity, gate and probability.'},{title:'Keep packs independent.',text:'The original public core and optional .gdexp expansions use separate payloads. Reopen the plugin to refresh installed packs.'}],
 changes:[{title:'Original Workshop core.',text:'128 original one-shots and 96 kit combinations replace the personal library in the base distribution.'},{title:'Modular expansion catalogues.',text:'Browse and randomize installed kits and role choices through a shared pack filter. Missing references are preserved.'},{title:'First stable release.',text:'Stable public instrument with original core sounds. Pack Builder can create separate expansions from your WAV and MIDI folders.'}],
 limitations:['First stable release; see the manual for the exact verification scope.','Full physical Q-Link coverage, worst-case hardware CPU and all separate-output destinations remain unverified.','Expansion installation uses Pack Builder or the Python CLI and requires reopening the plugin; it is not native MPC XPN registration.','Optional packs are installed separately; the base package contains Original Workshop only.'],
 screenshots:[{image:'groove-dice-main-1.0.0.webp',alt:'Groove Dice running original sounds and optional expansions on MPC Live III',caption:'MAIN · Actual 1.0.0 on MPC Live III / OS 3.10.1.12, playing the Pack Builder-authored Workshop Sessions pack. Optional packs are installed in this capture; the base release contains 128 WAV / 95 MIDI.'}]
}];
export const repo = 'https://github.com/Sunghyun23/kimswork';
export const navigationVersion = 'software-2026-11-16-crate-press';
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
