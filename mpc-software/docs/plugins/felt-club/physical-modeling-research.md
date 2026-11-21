# Electric-piano physical-modeling research

Latest Wurli status2026-11-21:0.2.7was rejected; see [Wurli source/geometry research restart](wurli-research-restart.md). Version/status statements below describe the earlier Tine research stage and are historical.

2026-11-21. User asked to investigate electric-piano physical modeling to improve Tine. This is research and a proposed implementation sequence; the installed/runtime engine remains0.2.4. No new sound-quality certification, public release or UI change. Keyscape direct rendering remains forbidden; official public YouTube performance audio remains a reference, not an isolated physical measurement.

## Primary sources

1. Florian Pfeifle, *Real-time Physical Model of a Wurlitzer and Rhodes Electric Piano*, DAFx2017. [Paper](https://www.dafx17.eca.ed.ac.uk/papers/DAFx17_paper_79.pdf). Full paper consulted, particularly sections4–5. Measurements motivate two-plane Rhodes tine motion and magnetic-field geometry; Wurlitzer uses reed/capacitance geometry. The hammer includes compression-rate-dependent dissipation, and the pickup models are physically distinct. Its FPGA implementation does not establish ARM/MPC cost.

2. Antoine Falaize and Thomas Hélie, *Guaranteed-passive simulation of an electro-mechanical piano: a port-Hamiltonian approach*, DAFx2015. [Paper](https://www.ntnu.edu/documents/1001201110/1266017954/DAFx-15_submission_33.pdf/a87ee400-f47c-4938-92e7-4030b1338cb7). A modal beam, nonlinear hammer and pickup are interconnected while accounting for energy. Useful for a stable reduced-order design; passivity requires the actual discrete coupling/integrator, not merely a safety limiter.

3. Falaize/Hélie,2016journal work; [author’s model, measurement comparisons and demonstrations](https://afalaize.github.io/posts/rhodes/). The author discusses nonlinear hysteretic contact and adjustable pickup geometry/circuit loading. Distance and transverse displacement are independent physical controls. The comparisons are instructive but are not a ready-made full-keyboard calibration of Felt Club.

4. Leonardo Gabrielli, Michela Cantarini, Paolo Castellini, Stefano Squartini, *The Rhodes electric piano: Analysis and simulation of the inharmonic overtones*, JASA148(5),2020, DOI10.1121/10.0002002. [Authors’ abstract and citation](https://pubmed.ncbi.nlm.nih.gov/33261386/). Abstract reviewed; publisher full text was unavailable. It addresses attack inharmonics through pickup spectra and laser-vibrometry measurements of the tuning-fork components, and describes nonlinear modal intermodulation. Do not invent numeric modal distributions from the abstract.

5. Tobias Sønderbo, *Real-time physical model of the Rhodes Electric Piano*, Aalborg University,2024. [Thesis](https://vbn.aau.dk/ws/files/719175589/Master_Thesis_Real_time_physical_model_of_the_Rhodes_Electric_Piano_Tobias_Sonderbo.pdf). Abstract and model/structure consulted. The author explicitly reports tuning and pickup-model limitations; a physically modeled resonator alone does not establish realistic output.

6. [jstolberg/rhodes](https://github.com/jstolberg/rhodes), author’s research implementation/README. It separates identification into fundamental/decay, pickup, hammer and inharmonic modes. This staged approach helps avoid several coefficients compensating for each other. Code and example recordings were not imported. An applicable code license was not verified, so it is a research reference only.

## Findings in our current source

These are direct observations of0.2.4source, distinct from external research findings.

| Part | Current implementation | Consequence / next check |
|---|---|---|
| Evaluation | fit_demo_voicing.py measures harmonics1–16 only; scanner prefers >=90%integer-harmonic energy | Bell-rich windows may be filtered out and inharmonic attack energy is not directly scored |
| Tine resonator | PhysicalVoice.h uses fixed relative modal ratios incl6.267,17.55,34.39; register variation mostly scales damping/mass/gap | Actual modal frequency and damping dependence across the keyboard is not calibrated |
| Motion/readout | All modal displacement sums feed one scalar pickup function; near-unison auxiliary modes exist | There is no explicit two-coordinate tine-tip trajectory / two-dimensional magnetic-field evaluation |
| Tone-bar coupling | Reciprocal spring/damping between modes0/1, with a hard force clamp | Some energy exchange exists; not a complete tuning-fork support model or a passivity proof |
| Hammer | Four contact substeps, nonlinear compression force, force/time limits | No explicit compression-rate-dependent hysteretic material loss; numerical safety limits are not material physics |
| Pickup | One-dimensional field-slope approximation, normalized velocity and displacement compression | Pickup distance, lateral offset and trajectory geometry are only partially represented |
| Register gain | Velocity readout divided by angular fundamental, then key gain correction | Physical induction and musical level balancing should be identifiable separately |

### Confirmed evaluation blind spot

For middle C,f0≈261.63Hz, the engine’s6.267mode lies about69.85Hz from the nearest integer harmonic; the old feature mask is only±10.47Hz. Its17.55mode is outside the1–16harmonic set entirely. The same omission was confirmed at C2/C5. Therefore0.2.4Tine rejection means failure to improve that limited proxy, not that the bell transient was adequately evaluated or that the model cannot improve. No numerical proxy result alone establishes perceived realism.

## Proposed order

1. Correct Tine diagnostics before further fitting: separate harmonic and inharmonic peaks, track attack-to-body decay, and revise selection so bell-rich excerpts are not rejected by an integer-harmonic purity gate. Label mono/dominant/polyphonic and unknown FX/velocity. Detect reference peaks rather than forcing real recordings to ideal cantilever ratios. Keep independent held-out notes/dynamics.
2. Isolate the pickup from the resonator in offline experiments. Retain the efficient modal core initially, but represent vertical/horizontal tip coordinates and calculate voltage from magnetic-flux change. Geometry-derived field lookup tables are physical-model coefficients, not recorded audio. Account for interpolation/aliasing and separate circuit loading/gain from geometry.
3. Refine contact with compression-rate-dependent, dissipative behavior. Verify material-force work and no artificial energy growth; do not merely raise stiffness or add noise.
4. Improve modal/register description and tone-bar energy exchange. Avoid adding parallel free oscillators to imitate a mechanical interaction. Test tuning/energy/ringing at multiple sample rates and all keys.
5. Calibrate in stages: fundamental/modal loss → pickup → hammer → attack modes. Use Keyscape demos for musical timbre direction; their unknown MIDI/age/FX prevents identifying every physical parameter. Preserve previous code when new variants do not generalize.
6. Keep approved UI/UID/238parameter order/57program names. Existing Bell/Bark/Tone controls can map to improved internals; no automatic new controls/pages. Benchmark actual MPC CPU after desktop correctness and authorized restart; desktop timing is not hardware certification.

No engine change or release was performed as part of this research. The recommended first milestone is a diagnostic + isolated pickup prototype, then a reviewed native Felt Club before/after render; never direct-render installed Keyscape.
