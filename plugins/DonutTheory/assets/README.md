# Visual asset provenance

Generated with the built-in image generation tool for this project on 2026-11-13. No downloaded artist photography, logos or samples are included.

- `main-background.png`: edited from the approved Donut Theory concept stored in `docs/plugins/donut-theory/design/main-approved.png`. Prompt: preserve warm worn cream paper, vintage Donut Theory title/subtitle and realistic donut/vinyl in top header; remove all host bars, UI controls, panels and labels; leave the lower area blank for real controls; target 1280:628 composition.
- `knob-vintage.png`: new transparent square rotary-control asset. Prompt: front-facing charcoal-brown ribbed bakelite rotary knob, worn matte cap, thin ivory pointer at twelve o'clock, soft contact shadow, no text/ticks/panel, clear silhouette at 64px.

The generated source assets remain unmodified. The MPC Image control scales the background; RotatingImage scales/rotates the knob. Generated pad images and SDK UI-kit controls are produced by `tools/build_skin.py`.

The images are development assets based on an approved design direction, not proof of actual MPC rendering. Check performance/memory of the full-resolution knob on hardware; if needed, create an optimized render asset without changing its appearance.

## 0.1.5 도넛 노브 / 배경

사용자가 승인한 도넛 노브 시안을 바탕으로 built-in image_gen으로 독립 자산을 제작했다(2026-11-14). `knob-donut.png`: transparent top-down chocolate torus knob, small hole, golden edge, cream indicator at 12 o'clock; no text/ticks/sprinkles. `main-background-v2.png`: cream vintage paper, compact Donut Theory logo, upper-right chocolate donut/record sleeve, blank control surface. 프롬프트 핵심은 이 설명이며 외부 아티스트 로고·사진·음원은 사용하지 않았다. 런타임은 스킨 생성기에서 1280×628 배경 및 회전 컨트롤로 사용한다. `envelope-vertical.png`는 스킨 코드가 생성하는 금속 페이더 filmstrip이다.

## 승인 시안 복원 배경 (2026-11-14)

`main-background-v3.png`: built-in image_gen으로 `docs/plugins/donut-theory/design/main-donut-approved.png`를 참조해 생성한 헤더/양피지 배경. 사용자 요청에 따라 승인 시안의 작은 로고, 초콜릿 도넛과 레코드, 빈티지 크림색을 복원했다. 생성 결과는 프로젝트 자산으로 복사했으며 런타임 크기는 1280×628이다. 버튼·패널·눈금·LCD 표시창은 `tools/restore_design.py`의 코드 기반 자산이며 도넛 노브 원본은 그대로 유지한다.

최종 편집 프롬프트 (built-in, CLI/API 미사용):
> Edit this production UI texture only. Keep parchment background and overall vintage artwork exactly. Shrink logo plus subtitle as one group to fit entirely within upper-left 29% width and uppermost 14% height. Keep left margin 2%. Shrink donut+record+record sleeve as one group into the upper-right 15% width, top 14% height, flush right. All area between 31% and 84% width in the top 16% must be empty parchment, no text and no donut. Below top 16% must remain empty parchment. Preserve exact logo words, vintage lettering, chocolate donut look, no new controls or words. Overall aspect 2:1.

### Record-sleeve collage header — 2026-11-14

`main-background-v4.png` removes only the old corner donut/vinyl from v3, retaining parchment and logo. `header-donut.png` is the superseded photographic donut proposal. `header-donut-collage.png` is the selected replacement, generated with the built-in image_gen tool and genuine transparency; runtime size154×154, bounds1108/0, foreground layer below modal editors. No external photos or record artwork were used.

Final prompt: Create a transparent-background vintage soul record sleeve collage badge for the Donut Theory synthesizer interface. A large whole ring donut with a clearly visible hole is the central hero, rendered as coarse 1970s offset-printed photographic halftone, faded chocolate brown, burnt orange and ochre, visibly misregistered cream and rust ink edges, worn ink and photocopy grain. Behind the donut: an irregular torn kraft-paper record sleeve fragment angled slightly, a small visible arc of black vinyl with grooves. Tiny rubber-stamp lettering SIDE A and 33⅓ on the torn paper, imperfect hand stamped, legible but secondary. A gritty affectionate crate-digging soul LP aesthetic, handmade and dimensional through overlapping cut-paper layers. Compact nearly square composition, donut occupies75 percent of badge, full silhouette visible, no clipping,5 percent transparent border. Genuine transparent exterior and transparent donut hole where appropriate. No shiny product photography, no smooth3D render, no cute cartoon, no neon, no additional words, no UI controls. Make the donut silhouette readable even when the whole collage is reduced to170 pixels.

### Large background donut — 2026-11-14

User rejected the small corner badge and explicitly requested an enormous donut behind controls. `donut-background-print.png` was edited using built-in image_gen from `header-donut-collage.png`. Runtime is600×600 at662,12, directly above parchment and below all widgets, replacing the small foreground badge. Alpha is preserved.

Prompt: Edit this donut collage into a large BACKGROUND PRINT for a vintage synthesizer control panel. Keep the whole enormous ring donut and hole recognizable, worn offset halftone, slightly misregistered print, torn record-sleeve paper and small arc of vinyl. Remove all stamp lettering. Change the entire palette to faded pale ochre, muted terracotta and parchment beige, reduce dark ink density substantially so dark brown UI labels over it remain legible. It should resemble a huge weathered donut printed directly on an old soul LP sleeve, not a small sticker or a shiny pastry photograph. Retain recognizable circular form and textured warm shading but no black or deep chocolate areas. Transparent background outside collage. Full shape visible. No UI, knobs, letters, or other new elements.

### Wide header collage — 2026-11-14

`header-donut-wide.png`: built-in image_gen, transparent original, runtime300×126 at962,0. Supersedes the square collage. Prompt: refined wide horizontal vintage soul-record collage for cream synth UI;2.3:1 transparent composition; large whole chocolate donut left in naturally wide three-quarter perspective occupying60% width, golden dough edge; vinyl grooves and torn ochre sleeve extending right; rust/cream diagonal bands; stamped SIDE A,33⅓ and DT—01; restrained1970s warm halftone, slight ink misregistration, worn paper, contact shadows; readable at280×122, no clipping, modern gloss, cartoon, sprinkles or UI. Generated source is preserved unchanged; runtime sizing is performed by the skin generator.

### Chocolate header — 2026-11-14

`main-background-brown-header.png`: built-in image_gen edit of v4. Prompt: preserve aspect/composition; change only the top15% into full-width dark faded chocolate-brown printed paper (#503022 approximate) with subtle grain and straight lower boundary; recolor existing Donut Theory/SOUL SYNTHESIZER logo and accent lines warm cream without changing shape, wording, position or size; preserve lower parchment; add no objects, controls or text. Header labels are native cream widgets, LCD contents remain dark. The wide donut collage stays separate and unchanged. Original generated source preserved; runtime resized by the skin builder.

### Natural collage edges — 2026-11-14

`header-donut-wide-soft-edge.png`: built-in image_gen edit of wide header, preserves300×126 runtime layout. Prompt: preserve donut, angle, colors, vinyl, stamps and wide layout; remove rectangular cutout feel with irregular torn diagonal bottom/right paper contours, fibers and transparent notches; bottom sleeve sweeps upward toward right; remove hard corners; keep complete donut and soft contact shadow fading to transparency; transparent safety margin; no new text/UI, no donut blur or identity change. Source preserved, runtime reference switched to this version.

### Full-bleed correction

User clarified the right header artwork must fill its whole area, not end in torn transparent gaps. `header-donut-full-bleed.png` generated by built-in image_gen from wide original. Prompt: full-bleed rectangular vintage donut/record/sleeve scene, preserve donut left and vinyl/striped sleeve/stamps right, fill every outer canvas pixel with chocolate-brown textured paper or artwork, extend sleeve off right edge, no torn outer silhouette or isolated sticker shadow, entire donut visible, approximately2.38:1, no new text/UI. Runtime318×126 at962,0 reaches the1280px right edge. Previous soft-edge version superseded.


### 0.1.9 — refined wide soul collage (2026-11-14)

Current production source: `header-donut-soul-final-v2.png` (1991×789). Built-in `image_gen`, no CLI/API fallback. Runtime is318×126 at962,0; the aspect ratios differ by less than0.02%. The approved overlap of large donut, vinyl and diagonal sleeve is preserved. The swollen folded glaze was replaced with a thinner matte chocolate coating and a visibly open elliptical hole. The brown backdrop was revised against the actual interface twice. Source files are preserved; the skin builder only resizes the selected art. `header-donut-natural.png` was rejected and never activated on the device. `header-donut-soul-collage.png` and `header-donut-soul-final.png` are intermediate versions.

Art-direction prompt (first edit, from `header-donut-full-bleed.png`):
> Surgical art-direction edit of this approved wide soul-record collage. Preserve composition, donut position and large size, three-quarter camera angle, overlapping black vinyl, diagonal ochre/rust sleeve, stamp placement and words, full-bleed edges and vintage printed aesthetic. Change only donut anatomy and finish: thinner smoother chocolate coating, subtle irregularities, matte surface and soft broad low-contrast highlights; clean open elliptical hole, no concentric ridges or folded crater. Keep golden fried dough edge, dominant left58% donut and overlapping vinyl. No small top-down separate donut, glossy CGI, new words or objects.

Final background edit prompt (target `header-donut-soul-final.png`, context `build/main-preview.png`):
> Image1 is the edit target. Image2 is context only showing the asset installed at the top right of a brown synthesizer header. Return only the wide collage asset. Preserve donut, vinyl, diagonal sleeve, stamps, scale, framing and shape. Change only the empty paper backdrop to match the darker mottled cocoa header immediately to its left in Image2. At the outer left2% use approximately RGB(92,43,25)/#5C2B19 with subtle texture, blending into existing paper by10% across. Do not add a line, frame, border, highlight or vignette at the left boundary. Output318:126 aspect, opaque full-bleed rectangle.

Visual acceptance: inspect at native1280×800 MPC scale, keep the donut/record/sleeve readable, ensure INPUT and B title remain clear, and directly touch the DRIVE selector below the art. Offline preview is an approximation, not proof of device rendering.


### Screenprint redesign proposal — 2026-11-14

User rejected the0.1.9 upper-right donut again. `header-donut-screenprint-proposal.png` is a new built-in image_gen style-transfer proposal, not installed artwork. Composition reference: `header-donut-soul-final-v2.png`. Final prompt: Redesign as a confident hand-drawn1970s soul LP screenprint; preserve large left donut overlapping vinyl and diagonal sleeve, full-bleed2.52:1 banner. Wide elliptical ring with generous open oval hole, flatter natural dough, irregular ink contour, thin chocolate glaze,2–3 flat ochre highlights, golden edge. Bold chocolate/ochre/rust ink shapes, slight offset misregistration, restrained paper grain. Vinyl arc with clean ink grooves, ochre sleeve/rust stripe, SIDE A/33⅓ stamps. Cocoa backdrop#5c2b19. No photorealism/CGI/inflated rubber/gloss/airbrush/deep wrinkles/sprinkles/cartoon face/UI. Offline layout preview: build/screenprint-proposal-preview.png. Production skin and installed0.1.9 are unchanged pending design review.


### Header contour correction proposal
User clarified that vintage texture was acceptable; the rectangular artwork protruding below the header was the problem. Screenprint style change was a misunderstanding and is superseded. `header-donut-contour-proposal.png`: built-in image_gen precise extraction from0.1.9 photographic collage. Prompt: preserve original donut/vinyl/sleeve texture and composition; keep upper74.6% opaque; terminate brown backdrop/vinyl/sleeve at a straight header edge; make lower25.4% transparent except rounded donut and subtle contact shadow; no restyling, torn edges or rearrangement. Proposal preview only, not installed.


### Exact header clipping proposal
The previous contour extraction left an opaque horizontal strip. Use two UI layers: original collage cropped to native header bounds962,0,318,94, plus only the lower32px of a transparent donut cutout at962,94,318,32. This enforces the94px edge by layout rather than approximating it in generated artwork. `header-donut-cutout.png` generated using built-in image_gen: extract only original donut; preserve texture/shape/size/left placement and wide canvas; remove vinyl/sleeve/stamps/backdrop/shadow, all exterior and hole transparent; no restyling. Source assets preserved; native-size cropping is runtime UI texture preparation. Preview build/header-layers-preview.png; production skin not yet changed.


### Corrected diagnosis: header background mismatch
User clarified that the vintage texture and donut were not the issue: the collage's brown backdrop differed from the header. Earlier screenprint/contour proposals addressed the wrong problem. Current source `header-collage-transparent.png` uses built-in image_gen background extraction from `header-donut-soul-final-v2.png`. Prompt: remove only empty brown paper backdrop to true alpha; preserve donut/vinyl/striped sleeve/stamps, positions, scale and vintage texture; remove exterior brown backing and backdrop visible through hole; no restyling, rectangular ground, white matte or halo. Runtime318×126 at962,0 now reveals the actual header texture through empty regions, rather than approximating its brown. Source skin updated and12 skin groups passed; preview build/header-background-fixed-preview.png. Device remains installed0.1.9 previous art until this revision is deployed.
