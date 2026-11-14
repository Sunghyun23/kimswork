export interface Plugin {
 slug: string; name: string; category: string; tagline: string; description: string;
 developmentVersion: string; image: string; imageAlt: string; intro: string; mapping: string[][]; limitations: string[]; verifiedDevice: string; features: {title: string; text: string}[];
 release: null | { version: string; tag: string; asset: string };
}
export const plugins: Plugin[] = [{
 slug: 'donut-theory', name: 'Donut Theory', category: 'POLYPHONIC SYNTHESIZER',
 tagline: 'A little worn. Endlessly warm.',
 description: 'Rounded bass, vintage analog leads, and soft, enveloping pads. Soulful sounds, made inside your MPC.',
 developmentVersion: '0.1.5', image: 'donut-theory.webp', release: null,
 imageAlt: 'Donut Theory design concept with donut knobs and ADSR faders',
 intro: 'Oscillator pairs, filters, envelopes, and modulation, all within easy reach.',
 verifiedDevice: 'MPC Live III / OS 3.10.1.12',
 mapping: [ ['A · Sound / Play','OSC Pair','Body','Detune','Glide'], ['B · Filter / FX','Filter Cutoff','Resonance','Drive','Space'], ['C · Amp','Attack','Decay','Sustain','Release'], ['D · Modulation','LFO Rate','LFO Depth','Tremolo Rate','Tremolo Depth'] ],
 limitations: ['Screen-mode Q-Link mapping mismatch remains unresolved', 'Touch chord hold-to-play and release behavior remain unresolved', 'Hardware MIDI, preset listening tests, and performance validation are in progress'],
 features: [
  { title: 'Two oscillators. Many shades of warmth.', text: 'Explore 16 oscillator pairs, then shape their weight and texture with Body and Detune.' },
  { title: 'Let the sound drift.', text: 'An LFO, independent tremolo, and amp ADSR shape everything from a moving lead to a slow, lingering pad.' },
  { title: 'Find your next chord.', text: 'Includes 64 sound presets and 32 chord progressions. Preset tuning and hardware playability testing are still in progress.' },
  { title: 'One more layer of character.', text: 'Combine Tape, Tube, and Fuzz drive with chorus and delay. These are character effects, not exact circuit emulations.' }
 ]
}];
export const repo = 'https://github.com/Sunghyun23/mpc-plugins';
export const url = (path = '') => `${import.meta.env.BASE_URL.replace(/\/$/, '')}/${path.replace(/^\//, '')}`;
export function releaseLinks(p: Plugin) {
 if (!p.release) return null;
 const tag = `${p.slug}/v${p.release.version}`;
 if (p.release.tag !== tag || !/^[a-z0-9.-]+\.zip$/.test(p.release.asset)) throw new Error(`Invalid release: ${p.slug}`);
 return { page: `${repo}/releases/tag/${encodeURIComponent(tag)}`, asset: `${repo}/releases/download/${encodeURIComponent(tag)}/${p.release.asset}` };
}
