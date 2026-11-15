export interface Plugin {
 slug: string; name: string; category: string; tagline: string; description: string; summary: string;
 developmentVersion: string; image: string; imageAlt: string; intro: string; mapping: string[][]; limitations: string[]; verifiedDevice: string; features: {title: string; text: string}[];
 changes: {title:string; text:string}[]; manual?: string; screenshots: {image:string; alt:string; caption:string}[];
 release: null | { version: string; tag: string; asset: string };
}
export const plugins: Plugin[] = [{
 slug: 'donut-theory', name: 'Donut Theory', category: 'POLYPHONIC SYNTHESIZER',
 tagline: 'A little worn. Endlessly warm.',
 summary: 'Moog-inspired warmth across bass, keys, synths, leads and pads—with up to sixteen playable chord slots.',
 description: 'Start with a factory bank revoiced for rounded Moog-inspired bass, singing leads, warm keys and soft pads. Shape the sound with hands-on controls, tape-like drive and independent tremolo, then play your own chord changes inside your MPC.',
 developmentVersion: '1.1.2', image: 'donut-theory-main-1.1.2.webp', release: {version:'1.1.2',tag:'donut-theory/v1.1.2',asset:'donut-theory-1.1.2-mpc.zip'},
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
  'Pad-location labels are manually assigned hints. Use LEARN to register MIDI; the plugin cannot automatically detect physical pad numbers or layout changes.',
  'The refreshed 63-sound bank was measured offline; a complete listening review of every revised sound on hardware is not certified.',
  'Testing on other MPC models, simultaneous finger multi-touch and a complete physical Q-Link rotation matrix are not certified.'
 ],
 changes: [
  {title:'Moog-inspired character, built into the factory sounds.',text:'All 63 playable factory presets have been revoiced across Bass, Keys, Synth, Lead and Pad. Fuller oscillator blends, smoother low-pass shaping, filter envelopes and restrained Tape saturation create the new starting sound. There is no extra mode to enable.'},
  {title:'A cleaner, more balanced starting point.',text:'Per-sound low-cut settings are retained and output trims have been recalibrated from stereo renders. Init remains dry with low-cut and effects off. The 64 names and program order stay the same.'},
  {title:'Easier chord navigation.',text:'Chord-preset arrows and slot-page arrows now have 44 × 44 touch targets. The four large chord pads remain, chord names have room beside the position hints, and the repeated HOLD TO PLAY captions are gone.'},
  {title:'See the pad position you assigned.',text:'Choose A–H and a cell in the 4 × 4 PAD LOCATION drawer inside the shared CHORD editor, then connect the physical pad with LEARN. MAIN shows the saved label and highlighted cell. Unassigned hints stay hidden; CLEAR LOCATION leaves MIDI routing intact.'},
  {title:'Keep your saved projects.',text:'Existing parameter identities and Q-Link assignments are preserved. Saved projects retain their stored sound settings; select a factory preset to load its refreshed voicing. Older projects start with pad-location hints unassigned. Screen pads audition sound and do not record MIDI into the MPC sequencer.'}
 ],
 screenshots: [
  {image:'donut-theory-main-1.1.2.webp',alt:'Donut Theory 1.1.2 running on MPC Live III with A1 to A4 manual position hints inside four large chord pads',caption:'MAIN · Actual v1.1.2 device screen. A1–A4 are manually assigned QA hints, not automatically detected pad positions.'},
  {image:'donut-theory-chords-1.1.2.webp',alt:'Actual Donut Theory 1.1.2 CHORD editor with root, type, voicing, octave, manual pad location and LEARN',caption:'CHORD · One shared editor for chords, voicing, octave, manual pad locations and MIDI LEARN.'}
 ],
 features: [
  { title: '64 starting points. Six familiar categories.', text: 'The refreshed Moog-inspired bank is the standard factory voicing. Browse Init, Bass, Keys, Synth, Lead and Pad. Per-sound low-cut and output trim help place the sound in a mix, while Init starts dry.' },
  { title: 'A little movement goes a long way.', text: 'Drift adds slow wow and faster flutter. Filter LFO and independent tremolo have separate rate and depth controls, alongside a complete amp ADSR.' },
  { title: 'Your changes, across sixteen slots.', text: 'Start with four chords and expand to eight, twelve or sixteen. Keep four large touch pads on MAIN; assign physical pads with LEARN and add a manual location hint. Each bank keeps its own chord-preset selection.' },
  { title: 'Shape the voicing. Keep the groove.', text: 'Choose the available inversions for each chord, Open voicing and an independent ±2-octave offset. The 32 four-chord presets are starting points, and affect only the bank you are viewing.' },
  { title: 'Weight, warmth and space.', text: 'Combine Tape, Tube or Fuzz character with chorus and tempo delay. Filter cutoff and resonance sit beside the LFO that moves them.' },
  { title: 'An MPC instrument, from the first screen.', text: 'Play keyboard notes, chord triggers or both in Split mode. Mono and Legato apply to melody notes; chord pads keep their full voicings.' }
 ]
}];
export const repo = 'https://github.com/Sunghyun23/mpc-plugins';
export const navigationVersion = 'donut-theory-1-1-2';
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
