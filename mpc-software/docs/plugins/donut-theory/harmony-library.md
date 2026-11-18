# Donut Theory harmony library — 1.5.0 development candidate

32 chord types and96four-slot progression presets. First11types and first32recipes retain their legacy meanings. Recipes change only the selected bank’s Root/Type; supported Voicing/Octave and MIDI settings remain. Reference Key does not automatically transpose a recipe.

## Chord types

| Type | Display | Voicing formula |
|---|---|---|
| Minor 7 | Cm7 | 1 · b3 · 5 · b7 |
| Major 7 | Cmaj7 | 1 · 3 · 5 · 7 |
| Minor 9 | Cm9 | 1 · b3 · 5 · b7 · 9 |
| Major 9 | Cmaj9 | 1 · 3 · 5 · 7 · 9 |
| Dominant 7 | C7 | 1 · 3 · 5 · b7 |
| Dominant 9 | C9 | 1 · 3 · 5 · b7 · 9 |
| Dominant 13 | C13 | 1 · 3 · b7 · 9 · 13 |
| Sus 2 | Csus2 | 1 · 2 · 5 |
| Minor | Cm | 1 · b3 · 5 |
| Major | C | 1 · 3 · 5 |
| 7 flat 9 | C7b9 | 1 · 3 · 5 · b7 · b9 |
| Minor 6 | Cm6 | 1 · b3 · 5 · 6 |
| Major 6 | C6 | 1 · 3 · 5 · 6 |
| Minor 6/9 | Cm6/9 | 1 · b3 · 5 · 6 · 9 |
| Major 6/9 | C6/9 | 1 · 3 · 5 · 6 · 9 |
| Minor 11 | Cm11 | 1 · b3 · b7 · 9 · 11 |
| Minor 13 | Cm13 | 1 · b3 · b7 · 9 · 13 |
| Major 9 #11 | Cmaj9#11 | 1 · 3 · 7 · 9 · #11 |
| Major 13 | Cmaj13 | 1 · 3 · 7 · 9 · 13 |
| 7 sus4 | C7sus4 | 1 · 4 · 5 · b7 |
| 9 sus4 | C9sus4 | 1 · 4 · 5 · b7 · 9 |
| 13 sus4 | C13sus4 | 1 · 4 · b7 · 9 · 13 |
| 7 sharp 9 | C7#9 | 1 · 3 · 5 · b7 · #9 |
| 7 flat 13 | C7b13 | 1 · 3 · 5 · b7 · b13 |
| 7 altered | C7alt | 1 · 3 · b7 · b9 · #11 |
| Minor 7 flat 5 | Cm7b5 | 1 · b3 · b5 · b7 |
| Diminished 7 | Cdim7 | 1 · b3 · b5 · bb7 |
| Augmented 7 | C7#5 | 1 · 3 · #5 · b7 |
| Minor major 7 | CmMaj7 | 1 · b3 · 5 · 7 |
| Add 9 | Cadd9 | 1 · 3 · 5 · 9 |
| Minor add9 | Cmadd9 | 1 · b3 · 5 · 9 |
| Sus 4 | Csus4 | 1 · 4 · 5 |

Extended voicings use at most five tones; optional fifths/elevenths are omitted as shown. Existing inversions/Open voicing apply. Minor11, Minor13, Major9#11 and Major13 are compact synth voicings, not six- or seven-note stacks.

## Four-chord progression presets

New recipes are original harmonic designs for neo-soul, hip-hop, R&B, vamps, modal colors, gospel movement, borrowed chords and turnarounds; no recorded artist performance was copied. The64new harmonic fingerprints are distinct even after root transposition. They are starting points, not a claim of a specific artist’s exact chords.

| # | Family | Name | Chords |
|---|---|---|---|
| 1 | Classic | Sunday Soul | Dm9 → G13 → Cmaj9 → Am9 |
| 2 | Classic | Basement Loop | Fm9 → Bbm9 → Eb9 → Abmaj7 |
| 3 | Classic | After Hours | Dm9 → Gm9 → A7b9 → Dm9 |
| 4 | Classic | Velvet Morning | Ebmaj9 → Cm9 → Fm9 → Bb13 |
| 5 | Classic | Dusty Sunset | Am9 → Fmaj9 → Dm9 → E7b9 |
| 6 | Classic | Warm Windows | Gmaj9 → Em9 → Am9 → D13 |
| 7 | Classic | Open Sky | Csus2 → Fmaj9 → Asus2 → Gsus2 |
| 8 | Classic | Soul Turnaround | Cmaj9 → A7 → Dm9 → G9 |
| 9 | Classic | Sunday Soul +2 | Em9 → A13 → Dmaj9 → Bm9 |
| 10 | Classic | Basement Loop +2 | Gm9 → Cm9 → F9 → Bbmaj7 |
| 11 | Classic | After Hours +2 | Em9 → Am9 → B7b9 → Em9 |
| 12 | Classic | Velvet Morning +2 | Fmaj9 → Dm9 → Gm9 → C13 |
| 13 | Classic | Dusty Sunset +2 | Bm9 → Gmaj9 → Em9 → F#7b9 |
| 14 | Classic | Warm Windows +2 | Amaj9 → F#m9 → Bm9 → E13 |
| 15 | Classic | Open Sky +2 | Dsus2 → Gmaj9 → Bsus2 → Asus2 |
| 16 | Classic | Soul Turnaround +2 | Dmaj9 → B7 → Em9 → A9 |
| 17 | Classic | Sunday Soul -2 | Cm9 → F13 → Bbmaj9 → Gm9 |
| 18 | Classic | Basement Loop -2 | Ebm9 → Abm9 → C#9 → F#maj7 |
| 19 | Classic | After Hours -2 | Cm9 → Fm9 → G7b9 → Cm9 |
| 20 | Classic | Velvet Morning -2 | C#maj9 → Bbm9 → Ebm9 → Ab13 |
| 21 | Classic | Dusty Sunset -2 | Gm9 → Ebmaj9 → Cm9 → D7b9 |
| 22 | Classic | Warm Windows -2 | Fmaj9 → Dm9 → Gm9 → C13 |
| 23 | Classic | Open Sky -2 | Bbsus2 → Ebmaj9 → Gsus2 → Fsus2 |
| 24 | Classic | Soul Turnaround -2 | Bbmaj9 → G7 → Cm9 → F9 |
| 25 | Classic | Sunday Soul +5 | Gm9 → C13 → Fmaj9 → Dm9 |
| 26 | Classic | Basement Loop +5 | Bbm9 → Ebm9 → Ab9 → C#maj7 |
| 27 | Classic | After Hours +5 | Gm9 → Cm9 → D7b9 → Gm9 |
| 28 | Classic | Velvet Morning +5 | Abmaj9 → Fm9 → Bbm9 → Eb13 |
| 29 | Classic | Dusty Sunset +5 | Dm9 → Bbmaj9 → Gm9 → A7b9 |
| 30 | Classic | Warm Windows +5 | Cmaj9 → Am9 → Dm9 → G13 |
| 31 | Classic | Open Sky +5 | Fsus2 → Bbmaj9 → Dsus2 → Csus2 |
| 32 | Classic | Soul Turnaround +5 | Fmaj9 → D7 → Gm9 → C9 |
| 33 | Neo Soul | Neo / Blue Hour | Dm11 → G13sus4 → Cmaj13 → Am13 |
| 34 | Neo Soul | Neo / Honey Glass | Fmaj9#11 → E7alt → Am11 → D9sus4 |
| 35 | Neo Soul | Neo / Warm Linen | C6/9 → Ebmaj9#11 → Dm11 → G7alt |
| 36 | Neo Soul | Neo / Soft Landing | Bbmaj13 → A7alt → Dm11 → G13sus4 |
| 37 | Neo Soul | Neo / Velvet Steps | Fmaj13 → F#dim7 → Gm11 → C13sus4 |
| 38 | Neo Soul | Neo / Amber Room | Ebmaj9#11 → D7alt → Gm13 → C9sus4 |
| 39 | Neo Soul | Neo / Open Curtains | Cmaj13 → Bm7b5 → E7alt → Am11 |
| 40 | Neo Soul | Neo / Quiet Light | F6/9 → Fm6/9 → Cmaj13 → G13sus4 |
| 41 | Hip-hop | Pocket / Back Porch | Am11 → Am13 → Fmaj13 → E7#9 |
| 42 | Hip-hop | Pocket / Sidewalk | Dm11 → G9sus4 → Dm13 → Eb7alt |
| 43 | Hip-hop | Pocket / Head Nod | Cm11 → F13sus4 → Cm6/9 → G7#9 |
| 44 | Hip-hop | Pocket / Night Bus | Gm11 → C13sus4 → Bbmaj9#11 → A7alt |
| 45 | Hip-hop | Pocket / Dust Loop | Fm11 → F#maj9#11 → Fm6/9 → C7alt |
| 46 | Hip-hop | Pocket / Loose Ends | Am6/9 → Fmaj9#11 → Dm11 → E7alt |
| 47 | Hip-hop | Pocket / Low Ceiling | Em11 → A9sus4 → Fmaj13 → F#m7b5 |
| 48 | Hip-hop | Pocket / Tape Corner | Dm6/9 → Fmaj13 → Em7b5 → A7#9 |
| 49 | Vamps | Vamp / Two Chairs | Cm11 → Cm13 → F9sus4 → F13sus4 |
| 50 | Vamps | Vamp / Same Window | Am11 → Am6/9 → AmMaj7 → Am7 |
| 51 | Vamps | Vamp / Moon Carpet | Dm11 → Ebmaj9#11 → Dm11 → Ebmaj13 |
| 52 | Vamps | Vamp / One Block | Gm13 → C13sus4 → Gm11 → C9sus4 |
| 53 | Vamps | Vamp / Copper Haze | Fm6/9 → Bb9sus4 → Fm11 → Bb13sus4 |
| 54 | Vamps | Vamp / Slow Orbit | Cmaj9#11 → D9sus4 → Cmaj13 → D13sus4 |
| 55 | Vamps | Vamp / Last Train | Em11 → Fmaj9#11 → Em13 → F6/9 |
| 56 | Vamps | Vamp / Still Warm | Amadd9 → Am11 → F6/9 → E7sus4 |
| 57 | R&B | RnB / Late Call | Fmaj13 → G13sus4 → Em11 → A7alt |
| 58 | R&B | RnB / Silk Morning | Cmaj13 → Bm7b5 → E7#9 → Am13 |
| 59 | R&B | RnB / Half Awake | Am11 → Dm6/9 → G13sus4 → C6/9 |
| 60 | R&B | RnB / Deep Blue | Ebmaj13 → Cm11 → F9sus4 → Bb7alt |
| 61 | R&B | RnB / Soft Focus | Dm11 → G7alt → Cmaj9#11 → A7#9 |
| 62 | R&B | RnB / Slow Reply | Fmaj9#11 → Em11 → Eb7alt → Dm13 |
| 63 | R&B | RnB / Felt Letter | Bbmaj13 → Am6/9 → Ab7alt → Gm11 |
| 64 | R&B | RnB / Afterglow | C6/9 → E7alt → Am6/9 → G13sus4 |
| 65 | Modal | Modal / Mist Walk | Dm11 → Fmaj9#11 → G9sus4 → Bbmaj13 |
| 66 | Modal | Modal / Wide Street | Cm11 → Ebmaj13 → F13sus4 → Abmaj9#11 |
| 67 | Modal | Modal / Pale Gold | Fmaj9#11 → Gmaj9#11 → Em11 → A9sus4 |
| 68 | Modal | Modal / Waterline | Am11 → Cmaj13 → G9sus4 → Dm13 |
| 69 | Modal | Modal / Paper Sky | D9sus4 → Fmaj13 → Ebmaj9#11 → Dm11 |
| 70 | Modal | Modal / Far Window | Gm13 → Bbmaj9#11 → Fmaj13 → C9sus4 |
| 71 | Modal | Modal / Dusk Garden | Em11 → Gmaj13 → D9sus4 → Fmaj9#11 |
| 72 | Modal | Modal / Soft Horizon | Cmaj9#11 → Dmaj13 → Fmaj9#11 → G13sus4 |
| 73 | Gospel | Gospel / Front Door | Cmaj13 → Fmaj13 → Fm6/9 → C6/9 |
| 74 | Gospel | Gospel / Sunday Air | Fmaj13 → F#dim7 → Gm11 → C7alt |
| 75 | Gospel | Gospel / Home Keys | C6/9 → C13sus4 → Fmaj9#11 → Fm6 |
| 76 | Gospel | Gospel / Warm Choir | Dm7b5 → G7alt → Cm11 → Cm6/9 |
| 77 | Gospel | Gospel / Open Hands | F6/9 → F#dim7 → Cmaj13 → A7#9 |
| 78 | Gospel | Gospel / Golden Step | Cmaj13 → Ebdim7 → Dm11 → G7alt |
| 79 | Gospel | Gospel / Back Home | Fmaj13 → Fm11 → Em11 → A7alt |
| 80 | Gospel | Gospel / Amen Room | Cmaj13 → Fmaj9#11 → Abmaj13 → G13sus4 |
| 81 | Borrowed | Color / Side Door | Fmaj9#11 → Abmaj13 → Ebmaj9#11 → Cmaj13 |
| 82 | Borrowed | Color / Half Light | Cmaj13 → Ebmaj13 → Abmaj9#11 → G7alt |
| 83 | Borrowed | Color / Paper Moon | Dm11 → C#maj9#11 → Cmaj13 → Eb7alt |
| 84 | Borrowed | Color / Blue Carpet | Am6/9 → Abmaj9#11 → Fmaj13 → E7alt |
| 85 | Borrowed | Color / Bent Frame | C6/9 → C#dim7 → Dm11 → C#7alt |
| 86 | Borrowed | Color / Hidden Stair | Fmaj9#11 → Edim7 → Ebmaj13 → D7alt |
| 87 | Borrowed | Color / Warm Shadow | Cmaj13 → Cm11 → Fm6/9 → Bb13sus4 |
| 88 | Borrowed | Color / Velvet Turn | Ebmaj9#11 → Dm7b5 → C#7alt → C6/9 |
| 89 | Turnarounds | Turn / Night Return | Cmaj13 → A7alt → Dm11 → G13sus4 |
| 90 | Turnarounds | Turn / Soft Resolve | Bm7b5 → E7b9 → Am11 → Fmaj13 |
| 91 | Turnarounds | Turn / Back Alley | Dm11 → C#7alt → Cmaj13 → A7#9 |
| 92 | Turnarounds | Turn / Slow Corner | Fmaj13 → E7alt → Am11 → G13sus4 |
| 93 | Turnarounds | Turn / Dim Lantern | Cmaj13 → C#dim7 → Dm11 → G7#9 |
| 94 | Turnarounds | Turn / Late Checkout | Em7b5 → A7alt → Dm11 → G7alt |
| 95 | Turnarounds | Turn / Round Trip | C6/9 → F#7alt → Fmaj13 → G13sus4 |
| 96 | Turnarounds | Turn / Last Cup | Fmaj9#11 → Em11 → A7alt → D9sus4 |
