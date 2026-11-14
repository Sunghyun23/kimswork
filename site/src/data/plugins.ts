export interface Plugin {
 slug: string; name: string; category: string; tagline: string; description: string; summary: string;
 developmentVersion: string; image: string; imageAlt: string; intro: string; mapping: string[][]; limitations: string[]; verifiedDevice: string; features: {title: string; text: string}[];
 manual?: string; screenshots: {image:string; alt:string; caption:string}[];
 release: null | { version: string; tag: string; asset: string };
}
export const plugins: Plugin[] = [{
 slug: 'donut-theory', name: 'Donut Theory', category: 'POLYPHONIC SYNTHESIZER',
 tagline: 'A little worn. Endlessly warm.',
 summary: 'Vintage-inspired bass, keys, leads and pads—with up to sixteen playable chord slots.',
 description: 'Shape rounded bass, soulful keys, analog-style leads and soft pads inside your MPC. Add a little pitch drift, tape-like drive and independent tremolo, then play your own chord changes.',
 developmentVersion: '1.0.0', image: 'donut-theory-main-1.0.0.webp', release: {version:'1.0.0',tag:'donut-theory/v1.0.0',asset:'donut-theory-1.0.0-mpc.zip'},
 imageAlt: 'Donut Theory running on MPC Live III, showing its four performance control groups and large chord pads',
 manual: 'plugins/donut-theory/manual/',
 intro: 'Sixteen oscillator pairs, hands-on envelopes and four Q-Link banks keep the sound close. Start with one of 64 sound presets, or build a chord arrangement across four banks of four slots.',
 verifiedDevice: 'MPC Live III / OS 3.10.1.12',
 mapping: [ ['A · Sound / Play','OSC Pair','Body','Drift','Glide'], ['B · Amp Envelope','Attack','Decay','Sustain','Release'], ['C · Filter / LFO','Filter Cutoff','Resonance','LFO Rate','LFO Depth'], ['D · FX / Tremolo','Drive','Space','Tremolo Rate','Tremolo Depth'] ],
 limitations: [
  'Sixteen chord slots share a sixteen-voice engine; overlapping chords can steal older voices.',
  'Screen chord pads audition the instrument; they do not send MIDI notes to the MPC sequencer.',
  'Use Q-Link Screen mode for chord editing. Program/Track mode retains the synth mapping.',
  'The MPC’s gray host PRESET list is flat. Use the instrument’s SOUND PRESET browser for category-first selection.',
  'Testing on other MPC models, simultaneous finger multi-touch and a complete physical Q-Link rotation matrix are not certified.'
 ],
 screenshots: [
  {image:'donut-theory-main-1.0.0.webp',alt:'Donut Theory main performance screen on MPC',caption:'MAIN · Sound, amp envelope, filter/LFO and FX/tremolo.'},
  {image:'donut-theory-chords-1.0.0.webp',alt:'Donut Theory chord editor showing bank selection and sixteen available slots',caption:'CHORD · Select a bank inside one editor. Edit root, type, voicing and octave.'},
  {image:'donut-theory-presets-1.0.0.webp',alt:'Donut Theory sound preset browser with Init, Bass, Keys, Synth, Lead and Pad categories',caption:'SOUND PRESET · Choose a category before a sound.'},
  {image:'donut-theory-fx-1.0.0.webp',alt:'Donut Theory FX detail page with drive, chorus, delay, tremolo and low-cut controls',caption:'FX · Character effects, independent tremolo and low-end cleanup.'}
 ],
 features: [
  { title: '64 starting points. Six familiar categories.', text: 'Browse Init, Bass, Keys, Synth, Lead and Pad. Per-sound low-cut and output trim help place the sound in a mix, while Init starts dry.' },
  { title: 'A little movement goes a long way.', text: 'Drift adds slow wow and faster flutter. Filter LFO and independent tremolo have separate rate and depth controls, alongside a complete amp ADSR.' },
  { title: 'Your changes, across sixteen slots.', text: 'Start with four chords and expand to eight, twelve or sixteen. Keep four large touch pads on MAIN; assign physical pads with LEARN. Each bank keeps its own chord-preset selection.' },
  { title: 'Shape the voicing. Keep the groove.', text: 'Choose the available inversions for each chord, Open voicing and an independent ±2-octave offset. The 32 four-chord presets are starting points, and affect only the bank you are viewing.' },
  { title: 'Weight, warmth and space.', text: 'Combine Tape, Tube or Fuzz character with chorus and tempo delay. Filter cutoff and resonance sit beside the LFO that moves them.' },
  { title: 'An MPC instrument, from the first screen.', text: 'Play keyboard notes, chord triggers or both in Split mode. Mono and Legato apply to melody notes; chord pads keep their full voicings.' }
 ]
}];
export const repo = 'https://github.com/Sunghyun23/mpc-plugins';
export const navigationVersion = 'donut-theory-1-0-0';
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
