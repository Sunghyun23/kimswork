export interface Plugin {
 slug: string; name: string; category: string; tagline: string; description: string;
 developmentVersion: string; image: string; imageAlt: string; intro: string; mapping: string[][]; limitations: string[]; verifiedDevice: string; features: {title: string; text: string}[];
 release: null | { version: string; tag: string; asset: string };
}
export const plugins: Plugin[] = [{
 slug: 'donut-theory', name: 'Donut Theory', category: 'POLYPHONIC SYNTHESIZER',
 tagline: '조금 낡고, 한없이 따뜻하게.',
 description: '둥근 베이스부터 빈티지 아날로그 리드, 포근한 패드까지. MPC 안에서 만드는 소울풀한 사운드.',
 developmentVersion: '0.1.5', image: 'donut-theory.webp', release: null,
 imageAlt: 'Donut Theory 승인 디자인 시안 — 도넛 노브와 ADSR 페이더',
 intro: '두 오실레이터의 조합, 필터, 엔벌로프와 모듈레이션을 손끝 가까이에 모았습니다.',
 verifiedDevice: 'MPC Live III / OS 3.10.1.12',
 mapping: [ ['A · Sound / Play','OSC Pair','Body','Detune','Glide'], ['B · Filter / FX','Filter Cutoff','Resonance','Drive','Space'], ['C · Amp','Attack','Decay','Sustain','Release'], ['D · Modulation','LFO Rate','LFO Depth','Tremolo Rate','Tremolo Depth'] ],
 limitations: ['Screen Q-Link 매핑 불일치 해결 중', '화면 코드 패드의 hold 발음·해제 미해결', '물리 MIDI 연주, 프리셋 청음·성능 검증 진행 중'],
 features: [
  { title: '두 개의 오실레이터, 여러 가지 온도.', text: '16가지 OSC Pair 조합에 Body와 Detune을 더해 음색의 두께와 결을 조절합니다.' },
  { title: '느리게 흔들리는 사운드.', text: 'LFO와 독립 Tremolo, 앰프 ADSR로 리드의 움직임부터 패드의 긴 여운까지 설계합니다.' },
  { title: '다음 코드로 이어지는 아이디어.', text: '음색 64개와 코드 진행 32개를 담았습니다. 프리셋별 청음 튜닝과 실기기 연주 검증을 진행 중입니다.' },
  { title: '마지막 한 겹의 질감.', text: 'Tape·Tube·Fuzz 드라이브와 Chorus·Delay를 조합합니다. 회로의 정밀 복각이 아닌 합성 효과입니다.' }
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
