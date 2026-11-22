import {repo} from './plugins';
import {loopClub} from './loop-club';
export const categories = [
 {slug:'sample-mods',name:'MPC Sample',description:'SD-based tools for MPC Sample hardware. Separate from MPC OS VST plugins.'},
 {slug:'instruments',name:'Instruments',description:'Hands-on instruments that run inside MPC standalone.'},
 {slug:'effects',name:'Effects',description:'Character and performance effects for MPC tracks and master buses.'},
 {slug:'apps',name:'Apps',description:'Desktop tools that help you prepare, organize and make music.'},
 {slug:'expansions',name:'Expansions',description:'Independent sound packs, with a clearly named target instrument.'}
];
export const companion = {
 name:'Groove Dice Pack Builder',version:'1.0.0',path:'apps/groove-dice-pack-builder/',
 image:'groove-dice-pack-builder-1.0.0.webp',
 summary:'Turn your WAV and MIDI folders into playable Groove Dice expansion packs.',
 release:`${repo}/releases/tag/groove-dice-pack-builder%2Fv1.0.0`,
 mac:`${repo}/releases/download/groove-dice-pack-builder%2Fv1.0.0/groove-dice-pack-builder-1.0.0-darwin-arm64.zip`,
 windows:`${repo}/releases/download/groove-dice-pack-builder%2Fv1.0.0/groove-dice-pack-builder-1.0.0-windows-amd64.zip`
};
export const workshop = {
 image:'groove-dice-workshop-sessions-device-1.3.4.webp',
 name:'Workshop Sessions',version:'1.0.0',target:'Groove Dice',path:'expansions/groove-dice/workshop-sessions/',
 summary:'Eight curated kits from 32 original one-shots, plus a beat and a timing template. Made with Pack Builder.',
 release:`${repo}/releases/tag/groove-dice-workshop-sessions%2Fv1.0.0`,
 asset:`${repo}/releases/download/groove-dice-workshop-sessions%2Fv1.0.0/groove-dice-workshop-sessions-1.0.0.gdexp`
};

// One product source for the overview and category pages; thumbnails always show real software.
import {plugins} from './plugins';
export const softwareItems = [
 loopClub,
 ...plugins.map(p=>({name:p.name,path:`plugins/${p.slug}/`,summary:p.summary,image:p.image,imageAlt:p.imageAlt,category:p.kind==='effect'?'effects':'instruments',status:p.release?`${p.release.preview?'Public preview':'Available'} · ${p.release.version}`:'In development'})),
 {...companion,imageAlt:'Groove Dice Pack Builder running on macOS',category:'apps',status:'macOS Apple Silicon · Windows x64'},
 {...workshop,imageAlt:'Workshop Sessions selected in Groove Dice1.3.4 on MPC XL',category:'expansions',status:'For Groove Dice 1.0.0 or later'}
];
