# Factory preset guide

32native SDK programs. Slash prefixes describe use; host category-first browsing is not verified. Input/Output start0dB. Master programs use noise/drift0, width100%, low-cutoff and low-band duckoff. Init is effects-neutral apart from crossover phase and fixed latency; Bypass is delayed raw audio.

| Category | Presets |
|---|---|
|Init|Clean|
|Drums|Pocket Press / Back Room / Soft Concrete / Loose Breaks / Snare Room / Dusty Knock / Ghost Lift / Bus Pump|
|Keys|Sunday Dust / Slow Velvet / Worn Chords / Electric Honey / Soft Felt / Midnight Tape|
|Samples|Crate Air / Loose Wire / Amber Room / Low Resolution / Chopped Soul / Frayed Edges|
|Master|Gentle Glue / Kick Space / Soft Pulse / Warm Finish / Sub Weight / Mid Pocket / Silk Top / Dry Glue|
|Performance|Half Light / Dotted Sway / Brake Room|

Drums: preserve transients withPunch, use restrained parallelMix for harder programs. Keys: gentle color and optionalDrift; Samples: progressively coarserLo-Fi. Master: conservative glue/band focus; Kick Space reacts to internal low frequencies, Soft Pulse follows host tempo. Performance programs use tempo ducking; Dotted Sway uses1/8DOT. Actual final balance depends on source level.

The bank is deterministic and generated fromfactory-presets.json. Offline synthetic-input renders are produced for every program; subjective listening is not claimed. Actual MPC host selection of Pocket Press was checked against its parameter values and transfer curve; exhaustive hardware audition of all32programs is not certified.
