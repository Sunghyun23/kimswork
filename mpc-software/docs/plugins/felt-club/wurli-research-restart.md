# Wurli research restart — 2026-11-21

The user rejected the0.2.7 audition as unlike a Wurli. Previous numerical/build success is not sound acceptance.0.2.7 remains an unaccepted local candidate, never installed/released. Approved Tine0.2.6 is preserved. This turn is research/source audit only: no engine, presets, UI, binary or device changes.

## What went wrong in our evaluation

The earlier selection emphasized a mellow spectrum, gentler body loss and numerical safety. Those are not Wurlitzer identity criteria. Our own same-note files now show a major change: at MIDI60/velocity95, attack H2/H1 fell from-4.55dB to-14.96dB; at velocity35 it fell from-9.92dB to-19.75dB. These are power ratios in matching ±10Hz harmonic bands, Hann-windowed20–220ms after onset; this is NOT a measured real-Wurli target. Body windows250–650ms also lose roughly9–10dB of relative second harmonic. This supports the hypothesis that the selected surrogate removed an important part of the previous overtone character. It does not by itself prove why the user dislikes the sound or that the old model was correct.

The source also divides charge-difference output by note fundamental and applies generic gain/filter/ADSR stages. Those are musical approximations, not an identified pickup/load/preamp circuit. Removing weak mechanical modes while leaving an unvalidated transfer curve can make the result more sine-like rather than more Wurli-like.

## Primary-source findings

1. Pfeifle/Münster, DAGA2017, *Tone Production of the Wurlitzer and Rhodes E-Pianos*: [paper](https://pub.dega-akustik.de/DAGA_2017/data/articles/000210.pdf). Read all four pages and visually inspect PDFpage3/Figs9–10. Their measured reed motion is near sinusoidal but pickup output is more complex. Their slot-field calculation shows unequal capacitance excursions due to asymmetric reed geometry and a complex output decay. Mechanical upper modes are weak; this does not imply removing upper electrical harmonics or all beating. This paper is EP200-specific evidence, not a universal calibrated200A/140B field table.

2. Pfeifle, DAFx2017, *Real-time Physical Model…*: [paper](https://www.dafx17.eca.ed.ac.uk/papers/DAFx17_paper_79.pdf), sections5.2/5.5/5.6. Distinguishes viscoelastic hammer contact, a nonuniform reed and field-derived capacitive transfer. The simplified constant-bias argument gives current proportional to capacitance rate. The paper itself notes pickup simplifications. Merely differentiating any smooth curve is insufficient evidence of a faithful pickup. No FEM values or force-law constants have been identified for our instrument from it.

3. Wurlitzer, *Series200 Service Manual*, §18Voicing: [manufacturer scan](https://www.jupitervintagepianos.com/wp-content/uploads/2023/10/Wurlitzer-Series-200-Service-Manual.pdf). Download succeeded via curl though browser fetch returned403. Visually inspected PDFpp15–16/printed12–13, FigsN/O/P/Q. Shows a through-slot: lateral clearance, steel face alignment, lead placement and vertical voicing matter. A reed resting inside the pickup gives an unwanted strong harmonic; lateral imbalance affects level. Lead-down bass and lead-up treble cannot be represented by an invented uniform gap law alone. Another older [manufacturer scan](https://static1.squarespace.com/static/581b462f5016e14ae76bd275/t/5ebb550ed544905f3db1d03a/1589335321798/wurlitzer-200-200a-service-manual.pdf) has different pagination; PDFp56 is a Model200series schematic, NOT an identified200A circuit. Do not merge these editions' page numbers or models.

4. Miessner/Wurlitzer, [US2919616A](https://patents.google.com/patent/US2919616A/en), pickup discussion/Figs4–15. Straddling electrodes, vertical placement and spatial pickup location change harmonic translation; an ideal centered thin geometry can suppress fundamental and double frequency, whereas offset restores fundamental. The patent also discusses mechanical-partial rejection by spatial pickup position. This is historical design evidence, not exact dimensions for a later200A. Its ideal alignment statement and the service manual's steel-face alignment must be reconciled using finite conductor/lead thickness rather than assumed contradictory.

An openwurli research page helped locate primary documents and flags an incorrect floor-gap assumption; it was used only as a search lead. No third-party implementation/code/constants were copied. Primary documents were checked independently. Source PDFs remain temporary research inputs, outside plugin/install/release assets.

## Audit of our current implementation

| Element |0.2.7source fact | Consequence |
|---|---|---|
| Pickup shape | C=1/sqrt(1+(displacement/gap-offset)^2) | One shifted scalar curve lacks actual electrode/reed/lead overlap geometry; smoothness and a zero-mean derivative do not validate its harmonic evolution. |
| Geometric controls | offset=1.20−0.25Bark; register gap exponent36 | Empirical constants have no dimensional/field-calibration evidence; not a real200A adjustment law. |
| Electrical load | Current normalized per fundamental, followed by common filters | No identified bias resistance, input capacitance/impedance or load response. Correct current alone does not establish output voltage. |
| Mechanics | Generic modal ratios/loss, same hammer mass/stiffness law family | No measured per-register reed mass, contact time, modal loss or lead geometry. Changing high modes cannot compensate for missing pickup identity. |
| Amplifier/cabinet | Shared Colour drive/filter with optional stereo FX | No dedicated200/200A preamp/output/cabinet identification. FX can obscure the diagnostic dry signal. |
| Validation | Nine isolated notes, safety tests, spectral centroid | Does not constrain the full velocity-to-harmonics evolution or compare against a verified dry real instrument. |

## Replacement design and validation order (proposed, not implemented)

1. Establish a single target model/output point.200A is the proposed baseline; distinguish raw pickup, direct output and speaker sound. Keep140B Keyscape excerpts separate, with unknown MIDI/velocity/FX limitations. No direct rendering of installed Keyscape.
2. Identify reed displacement and decay first; use a fundamental-dominated mechanical model with weak upper modes only where supported. Separate excitation and pickup spatial weights. Preserve actual mechanical damper/CC64 behavior. Do not demand an arbitrary velocity-dependent decay solely to make curves differ.
3. Build a geometry-based electrostatic pickup prototype offline. Represent finite reed and solder mass cross-section, two slot fingers, lateral clearances, plate thickness and rest alignment separately. Prefer a2Delectrostatic field solve/charge-derived lookup over another hand-shaped bell curve. Mesh/boundary convergence and unknown dimensions must be documented. A field lookup is coefficient data, not sampled instrument audio, and is not ground truth without measured geometry.
4. Connect that capacitance to an explicit load approximation; identify the correct target circuit before implementing its response. Test constant-bias assumptions instead of treating all capacitive pickups as the same reciprocal-gap model. Keep gain normalization separate from the circuit equations.
5. Evaluate pickup alone with controlled sine displacement before full hammer/notes: fundamental survival, polarity, symmetry/doubling limit, harmonic response over amplitude, and late-time harmonic changes. Then inspect nine velocity levels across bass/mid/treble and extra held-out keys; retain Tine byte-identity and RT/state/safety tests. No invented universal H2/H1 threshold.
6. Only after dry timbre/attack/body matches the reference direction, audition a subtle native mono tremolo, then optional stereo/space FX. Real-instrument comparison, user listening acceptance and device CPU remain separate from correctness tests. Add amplifier/cabinet color after pickup behavior is understood; do not fake missing pickup bark with generic distortion.

No new implementation is accepted from this research.0.2.7 is retained as a rejected comparison artifact; no rollback, installation, restart, public release or site update performed.


## Isolation follow-up

0.2.8separates all seven backends without redesigning this rejected Wurli sound. Future Wurli work belongs exclusively in src/engines/wurli; see engine-isolation.md for boundaries and regression evidence. Approved Tine remains a protected reference.


## Field implementation candidate0.2.9

User authorized implementation. Wurli-only2Dboundary-element slot/reed/solder field coefficients, cubic readout, four substeps and explicit approximate load are now implemented as an audition candidate. Geometry ratios and load/voicing values are not measured200Aidentification. Prior analytic bell curve and old proxy-fit tilt are removed. Native/bothARM/full tests passed; all six other engines are sample-identical at3rates. Same-MIDI before/after and5HzmonoTremolo auditions are in FeltClub/design/audio-0.2.9; model/field limitations in validation-0.2.9. No listening acceptance, device install/restart or publication yet. Full verified amplifier/cabinet remains future work, not completed.
