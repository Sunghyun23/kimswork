import {repo} from './plugins';
export const categories = [
 {slug:'instruments',name:'Instruments',description:'Hands-on instruments that run inside MPC standalone.'},
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
 name:'Workshop Sessions',version:'1.0.0',target:'Groove Dice',path:'expansions/groove-dice/workshop-sessions/',
 summary:'Eight curated kits from 32 original one-shots, plus a beat and a timing template. Made with Pack Builder.',
 release:`${repo}/releases/tag/groove-dice-workshop-sessions%2Fv1.0.0`,
 asset:`${repo}/releases/download/groove-dice-workshop-sessions%2Fv1.0.0/groove-dice-workshop-sessions-1.0.0.gdexp`
};
