import { readdir, readFile, stat } from 'node:fs/promises';
import { join } from 'node:path';
const root = new URL('../dist/', import.meta.url).pathname;
async function walk(dir) { const entries = await readdir(dir, {withFileTypes:true}); return (await Promise.all(entries.map(e=>e.isDirectory()?walk(join(dir,e.name)):[join(dir,e.name)]))).flat(); }
const files=await walk(root); let checked=0;
for (const file of files.filter(f=>f.endsWith('.html'))) {
 const html=await readFile(file,'utf8');
 for (const [,ref] of html.matchAll(/(?:href|src)="([^"]+)"/g)) {
  if (!ref.startsWith('/')) continue;
  if (ref.startsWith('/mpc-plugins/')) throw Error(`Obsolete base: ${ref}`);
  const u=new URL(ref,'https://example.invalid');let target=join(root,decodeURIComponent(u.pathname.slice(1)));
  if ((await stat(target)).isDirectory()) target=join(target,'index.html');
  await stat(target);
  if(u.hash && !(await readFile(target,'utf8')).includes(`id="${decodeURIComponent(u.hash.slice(1))}"`)) throw Error(`Missing anchor: ${ref}`);
  checked++;
 }
 if(html.includes('/releases/latest')) throw Error('Repository-wide latest release link is forbidden');
}
console.log(`PASS: ${checked} local page, image, CSS and anchor references`);
