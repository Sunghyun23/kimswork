export interface Plugin {
 slug: string; name: string; category: string; tagline: string; description: string; summary: string;
 developmentVersion: string; image: string; imageAlt: string; intro: string; mapping: string[][]; limitations: string[]; verifiedDevice: string; features: {title: string; text: string}[];
 changes: {title:string; text:string}[]; manual?: string; screenshots: {image:string; alt:string; caption:string}[];
 release: null | { version: string; tag: string; asset: string };
}
export const plugins: Plugin[] = [{
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
}];
export const repo = 'https://github.com/Sunghyun23/mpc-plugins';
export const navigationVersion = 'donut-theory-1-2-2';
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
