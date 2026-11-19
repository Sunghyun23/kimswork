# Crate Press

Read root instructions andmpc-software/docs/plugins/crate-press/{spec,presets,validation}.md. Public-source stereo effect, kim/K36Z, current development 0.2.2. Do not claim device completion beyond the recorded verification scope.

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
