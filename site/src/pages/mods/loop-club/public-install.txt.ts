import install from '../../../../../mpc-software/mpc-sample/mods/public/LoopClub/docs/public-install.txt?raw';
export function GET(){return new Response(install,{headers:{'Content-Type':'text/plain; charset=utf-8'}});}
