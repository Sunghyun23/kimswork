# Crate Press

Read root instructions andmpc-software/docs/plugins/crate-press/{spec,presets,validation}.md. Public-source stereo effect, kim/K36Z, current public preview 0.2.5. Do not claim device completion beyond the recorded verification scope.

- Preserve current380parameter IDs/order.0–346legacy;347–354momentary gates;355–370independent performance setup;371–378activity. Factory programs deliberately stop atLegacyCount347. Full state retains setup; reset disarms and clears gates at first display tick.
- PreserveMAIN16Q-Links and groupedI/O. PERFORM has its own16Screen controls; Program/Track staysMAIN.
- Eight MAIN buttons use rawMouseDownToggle+top transparent continuousSliderResetOnGestureEnd. Never useButtonResetOnGestureEnd, or placehit surfaces/captions above the slider. Inputs/markers must be tested on device before claiming hold works here.
- Real signal monitor uses measured signed stereo extrema; no fake animated wave. Curves use DSP reduction function. Concepts and offline runtime previews are not hardware screenshots.
- Noallocation/lock/log/file/hostcallbacks inprocess. History allocates inprepare; reset invalidates filled history. Test8effects,release/echo tail,capture freeze,tempo,setting persistence andstate gate disarming. External sidechain is notimplemented.
- Latest approved art isdesign/main-concept-v6-performance-eight.png;kim badge removed while vendor metadata stayskim. Keep concepts outofruntime package.

- LATCH parameter379 appends; schema380. Touch-only header toggles conditional momentary vsnormalButton surfaces. Never addResetOnGestureEndtoLatchedbuttons. TurningLatchOff clears8gates; loadingstate disarmsactualeffectswhilepreservingmode. Main/Program andPERFORM16assignments unchanged.

- Native MPC skins cache values. Preserve the plugin-local deferred host notification bridge: engine publish queues flags; non-render value reads/idle drain them; render-thread tokens prevent callbacks from process/read calls there. Program/state selection queues a full refresh. Keep tests/bridge.cpp.
- Choice captions must be baked into each actual Button on/off image. Neither Label nor Image siblings may cover their touch surfaces. Do not restore the SDK radio parent MouseDown Q-Link handler over those buttons.

- Local0.2.4approved footer reflow: LATCH and Curve Band controls at y488; unchanged eight performance buttons at y538. Original lettering/knobs/faders retained;380IDs/16maps/detail pages stable. Groove Dice stays untouched. See spec and validation for actual QA.

- User explicitly authorized official0.2.4 publication and website updates after device verification. Earlier local-only statements are history. Preserve original artwork and the recorded physical/audio limits.

2026-11-19 user approved Crate Press Open MPC registration.0.2.5is a preview docs/license/distribution patch only: metadata205, explicit MIT project/SDK notices, engine/UI/380parameters/32programs/Q-Link maps unchanged. Retain preview:true/prerelease:true; do not promote to stable or claim new hardware verification. Original0.2.4screens remain labeled. Only official SDK remote deployment is allowed; no Open MPC legacy installers/benchmarks on device. Pack Builder companion registration is a separate future task.

2026-11-19 Crate Press0.2.5public preview release/site complete: sourcee75e395, Pages44528905173success;91responsive and11public page/image/download checks passed. Native/ARM/SDK, full offline UBSan/engine/performance/28pairs/bridge/skin and3host configurations20PASS/0WARN/0FAIL. Explicit MIT notices included; SDK and per-CPU Open MPC artifacts preserve engine/UI/380parameters/32programs. PR1036submitted, Linux CI44529150797success, review/merge pending. Preview/prerelease:true; expected catalogExperimental, no Verified/tested.json. Current MPC installation remains0.2.4; no device update/restart or new hardware certification in this docs/distribution task. Other plugin/app versions unchanged; Pack Builder companion registration remains future work. Records: mpc-software/catalog/open-mpc/ and CratePress/design/release-0.2.5/.


2026-11-20 approved common footer / Crate Dice. Current candidate0.2.6: Dice randomizes13MAIN character values with13shared locks and exact Undo; Input/Output/Mix/safety/LATCH/performance gates and setup remain unchanged. First380declarations retained;396total. Same32factory programs and Q-Link targets. Policy in mpc-software/shared/ui/. NVMe only; preserve all19Groove owner files, SD/SSD untouched. No public release/site update in this task.


Remote FX browser tip (MPC XL3.10.1.12): if the TYPE tree puts External Plugins below the viewport and remote wheel/drag does not scroll, turn TYPE off and MANUFACTURER on. The short manufacturer list exposes kim directly; expand its plus icon, select Crate Press and SELECT. This is UI navigation only, not a firmware/settings edit.

2026-11-20 lock visual refinement: the previous44px full lock tile overwhelmed MAIN knobs. Render a24px badge within the unchanged44×44touch surface on every page; transparent surrounding pixels must not introduce a visible full-size box. Parameters, lock state, footer and mappings are unchanged. This refinement is local until explicitly deployed/verified.

2026-11-20 small24px lock badge deployed to internal NVMe; all165files independently SHA256verified. Actual MPC remote MAIN/DUCK/COLOR/MORE artwork, original44px transparent hit, Drive lock throughDice, Undo andUnlockAll checked; final stateInit/unlocked. Prior local-only refinement note is superseded. No SD/SSD/public release/site changes. Evidence:design/device-0.2.6/lock-refinement/.

2026-11-20 user rejected the24px lock badge as too small and requested consistency with other plugins. Supersede the badge design with Donut Theory exact lock silhouettes/padding in the existing44px image/hit: outlined open lock, filled closed lock, no square badge. Geometry alpha masks match both Donut states exactly; Crate uses theme amber ink. New native/bothARM build and20page footer regression pass;165NVMe files independently SHA256verified. Earlier small-badge captures are historical.

2026-11-20 user authorized Crate Press0.2.6 public preview release and website update, including final Donut-matched locks. Prior local-only statements are historical for Crate; other candidates remain unpublished. Keep preview status and recorded hardware limits. Publish normal SDK plus per-CPU portable archives, final actual captures/manual. No new device restart or storage changes required.

2026-11-20 Crate Press0.2.6preview publication complete: source2b02d59/tag crate-press/v0.2.6, sixSDK/perCPU ZIP/checksum assets anonymously SHA256verified. Pages44673082720success;35responsive/295local reference/11public page-image checks pass. Final actual MPC captures and Dice manual are live. NVMe165files verified, final Donut-matched lock silhouettes with44px touch; stopped-track Dice/locks/Undo and two latched states checked. SD/SSD untouched. Donut1.15.0/Groove1.8.0public versions unchanged; local footer candidates remain pending. Preview/hardware limits remain explicit. Evidence:CratePress/design/device-0.2.6/publication.json.

2026-11-20 owner explicitly requested stable Crate Press promotion. Promote existing0.2.6GitHub Release to prerelease:false; keep all six public assets/digests and existing tag unchanged. Stable status is a distribution choice, not additional hardware certification. Update site/current manual/README; historical preview artifacts retain initial labels. No device reinstall or other product release is needed.

2026-11-20 Crate Press0.2.6stable promotion complete: GitHub prerelease:false/draft:false, sixassets/digests unchanged, existingtag/runtime preserved. Pages44674703497success from665939b;35responsive/295local reference/11publicpage-imagechecks pass, actual site badgeAvailable now and manualStable release confirmed. NVMe0.2.6needs no reinstall; hardware verification scope unchanged. Donut/Groove public and local candidate statuses unchanged. Evidence:CratePress/design/device-0.2.6/stable-promotion.json.
