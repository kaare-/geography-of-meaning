# Language and Signal Evolution

> **Guiding question:** How do communicative signals become shared predictive structure across populations and generations?

## Status

**Planned** — no signal production, mutation, dialect tracking, or language-emergence pipeline in code yet. Builds on communication skeleton ([10_communication.md](10_communication.md)), memory and concepts ([06_memory.md](06_memory.md), [07_concepts.md](07_concepts.md)), sleep rehearsal ([09_sleep.md](09_sleep.md)), culture ([16_culture.md](16_culture.md)), settlements ([22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md)), and narrative extraction ([27_narrative_extraction_and_interpretation.md](27_narrative_extraction_and_interpretation.md)).

## Summary

Language is not a starting assumption. It is a **late emergent compression** of repeated signal→prediction relationships that stabilize across individuals, lineages, and geography.

Creatures begin with sensor traces and memory graphs — not words. A researcher may later label a recurring sound pattern as referring to water, food, a river, or shelter. Those labels describe **emergent predictive structure** in analysis only. Creature code never receives `water`, `food`, `river`, or `shelter` as cognition types.

Signal evolution tracks how multidimensional sound patterns diversify, mutate, drift, compress into concepts, sequence into proto-syntax, and eventually support storytelling, trust, dialects, and cultural transmission — all without hardcoded language.

## Core Principle

> Signals are coordinates in prediction space — not labels in a dictionary.

A sound matters when it **changes expectations about future regulatory state**. What researchers call a "word" is a stable region in multidimensional signal space that reliably activates a shared predictive subgraph across listeners. Meaning remains predictive relevance ([00_project_overview.md](00_project_overview.md)) — never propositional content.

## Origins

Language grows from communication, not the reverse.

The developmental path:

1. **Social prediction** — passive coupling to conspecific behavior ([10_communication.md](10_communication.md) § Origins)
2. **Attention signals** — low-cost emissions biasing listener processing
3. **Predictive signals** — sounds that reliably precede outcomes the sender has learned
4. **Compressed call types** — recurring signal clusters linked to outcome regions
5. **Shared concepts** — population-stable activation patterns ([07_concepts.md](07_concepts.md))
6. **Sequences and proto-syntax** — ordered signals guiding multi-step prediction
7. **Narrative and cultural lineages** — temporal chains and geographic divergence ([16_culture.md](16_culture.md))

Each stage is a refinement of the same mechanism: one predictive system altering another through traces.

## Signal Space (Multidimensional, Not Labels)

Sounds are **points or trajectories** in a continuous space — not entries in a vocabulary table.

| Dimension | Role in discrimination |
|-----------|-------------------------|
| Frequency | Timbre, source class, signature encoding |
| Duration | Urgency, persistence, boundary marking |
| Rhythm | Memorability, turn-taking, sequence slots |
| Intensity | Range, salience, attention capture |
| Contour | Pitch movement over time; gesture-like shape |

No dimension carries a built-in semantic category. A high-intensity short burst is not "alarm" in creature cognition — it is a region of signal space that may, through experience, precede regulatory change.

**Planned:** `EmitSound` action writes physical events; listeners record them through `sound_ambient` and `sound_calls` sensor channels (`creatures/sensors.rs`).

## Signal Families

Over time, populations may cluster recurring productions into **signal families** — loose groupings of nearby points in signal space that tend to activate similar memory subgraphs.

Families emerge from:

- shared regulatory contexts (e.g. repeated proximity to moisture traces — researcher label: *water*)
- shared action contexts (e.g. repeated rest in enclosed low-wind regions — researcher label: *shelter*)
- shared social contexts (coordination, mating, territorial spacing)

Families are **statistical**, not taxonomic. Boundaries blur; overlap is normal. Researchers may name a family; creatures never receive the name.

## Signal Mutation

Individual emitters vary production parameters — intentional drift, error, or exploratory emission. Mutation introduces **neighboring points** in signal space.

Mutation matters when:

- a variant still activates a similar outcome prediction (stabilizes as family member)
- a variant activates a different outcome (splits a family or creates deception)
- a variant fails to activate any reliable prediction (dies out)

**Planned:** production parameters drawn from genome bias + individual noise; no fixed call catalog.

## Signal Drift

Populations separated by geography, lineage, or social cluster experience different sound→outcome pairings. **Drift** is divergence of signal families across groups without central authority.

Drift accelerates when:

- regulatory environments differ (one group near flowing moisture — researcher label: *river* — another near dry high ground)
- sender signatures are weakly coupled across groups
- sleep consolidation ([09_sleep.md](09_sleep.md)) reinforces local pairings over foreign ones

Drift is the raw material of dialect formation.

## Concept Activation

When a signal reliably activates a compressed memory region, the listener's **active concept set** shifts ([07_concepts.md](07_concepts.md)). Concept activation is the bridge between raw sound traces and efficient prediction.

A signal becomes "concept-like" when:

- activation spreads through a compressed cluster faster than through raw sensory nodes
- multiple distinct sensory paths converge on the same cluster via `sound_activates` edges
- prediction error drops when the cluster is active before action selection

**Planned:** `NodeKind::Sound` nodes and `EdgeType::SoundActivates` in `memory/nodes.rs` and `memory/edges.rs`; concept nodes remain placeholders until compression pipeline exists.

## Shared Concepts

A concept becomes **shared** when independent creatures activate overlapping clusters in response to similar signals — not when they share a symbol table.

Shared concepts require:

- repeated exposure in overlapping populations
- sufficient trust in sender predictions ([10_communication.md](10_communication.md) § Trust)
- settlement-scale interaction density ([22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md))

Shared does not mean identical graphs. Two creatures may activate related but non-isomorphic subgraphs that researchers later group under one label.

## Signal Sequences

Single signals compress local predictions. **Sequences** chain activations across time — each signal altering which subgraph is primed for the next.

Examples (researcher descriptions only):

- call → pause → call (turn-taking, joint attention)
- rising contour → sustained tone (approach trajectory in signal space)
- rapid triple → long fall (multi-step regulatory expectation)

Sequences enter memory as ordered `sound_activates` chains and `precedes` edges between sound nodes. Sleep may replay them without live input ([09_sleep.md](09_sleep.md)).

## Predictive Paths

A **predictive path** is a directed walk through memory from signal activation to expected regulatory outcome. Language-like behavior appears when multiple creatures share homologous paths — similar edge topology and outcome endpoints — even if node IDs differ.

Researchers map paths to narratives ([27_narrative_extraction_and_interpretation.md](27_narrative_extraction_and_interpretation.md)):

- path emergence (first reliable sound→outcome chain)
- path strengthening (trust, repetition)
- path branching (mutation, context split)
- path extinction (outcome no longer occurs)

## Proto-Syntax

**Proto-syntax** is not grammar rules in code. It is **stable ordering constraints** on signal sequences that improve prediction accuracy for listeners.

Emergent constraints may include:

- fixed slot order (attention marker before content-like cluster activation)
- boundary signals (rhythmic separators between sequence units)
- turn-taking rhythm (alternation patterns between signatures)

Proto-syntax appears only when sequence violations measurably increase prediction error. No Chomsky layer; no parser in creature code.

## Compression

Individual signals and sequences are expensive to produce and process. **Compression** merges frequently co-occurring signal→outcome paths into concept nodes — the same mechanism as sensory concept formation ([07_concepts.md](07_concepts.md)), extended to the auditory channel.

Compression tradeoffs:

| Uncompressed | Compressed |
|--------------|------------|
| High fidelity, high cost | Lower fidelity, high efficiency |
| Rapid local adaptation | Population-stable reference |
| Sensitive to mutation | Resistant to small drift |

Over-compression risks false confidence (myth-like clusters — researcher label in [16_culture.md](16_culture.md)).

## Communication Efficiency

Populations under interaction pressure favor signals that **maximize prediction change per unit cost** (energy, attention, range).

Efficiency pressures include:

- shorter durations when intensity suffices
- stereotyped rhythm when contour is redundant
- concept invocation instead of full sensory replay

Efficiency is not optimality — local minima, deception, and drift prevent global convergence.

## Storytelling

**Storytelling** is sequenced sound activation that externalizes one creature's memory subgraph into another's spreading activation — a temporal transfer of predictive structure ([10_communication.md](10_communication.md) § Storytelling).

A story, in mechanism terms:

1. Sender activates a known path in its own graph through ordered emission
2. Listener builds or strengthens homologous edges
3. Listener's future actions shift without direct experience of intervening causes

Not narrative prose. Not fiction opcode. Pattern transfer across signatures.

## Narratives

When signal sequences and shared concepts persist across generations, researchers extract **communication narratives** ([27_narrative_extraction_and_interpretation.md](27_narrative_extraction_and_interpretation.md) § Communication Narratives):

- new signal appears
- signal spreads through a settlement
- signal diverges between lineages
- signal disappears or merges

Narratives describe cultural evolution in prediction space — not plots imposed on creatures.

## Trust and Language

Trust is prediction confidence on per-sender subgraphs ([10_communication.md](10_communication.md) § Trust). Language-like stability **requires** trust: listeners must accept that a sender's signal predicts outcomes the listener cares about.

Low trust:

- blocks concept sharing
- amplifies drift
- favors redundant or redundant-seeming signals (repetition, intensity)

High trust:

- permits compression
- enables longer sequences
- allows efficient shorthand activation

Trust is never a separate global variable — only edge strength and error history on `signature`-linked subgraphs (`creatures/creature.rs`).

## Dialects

A **dialect** is a geographically or socially bounded region of signal space that activates overlapping but non-identical concept clusters compared to another region.

Dialect boundaries correlate with:

- settlement interaction networks ([22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md))
- migration history ([21_exploration_and_discovery.md](21_exploration_and_discovery.md))
- ecological differences (researcher labels: *water* availability, *shelter* topology, *food* distribution)

Mutual intelligibility is partial prediction overlap — not translation tables.

## Communication Lineages

Signals inherit through **lineages** — chains of transmission from signature to signature across generations ([13_inheritance.md](13_inheritance.md)).

Lineage tracking (researcher-facing, planned):

- parent emission → child exposure → child emission bias
- genome bias toward production parameters
- cultural copying when trust exceeds threshold

A communication lineage is analogous to a genetic lineage but operates on signal space coordinates and memory topology, not DNA sequences alone.

## Cultural Transmission

Cultural transmission is the **population persistence** of signal families, shared concepts, sequences, and proto-syntactic constraints beyond individual lifetimes ([16_culture.md](16_culture.md)).

Transmission channels:

- direct sound exposure between conspecifics
- sleep-consolidated reinforcement
- settlement-mediated high-density interaction
- landscape-indexed context (external memory — [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md))

Transmission fails when regulatory environments change faster than concept update — producing fossil signals that no longer predict.

## Research Interpretation

Researchers interpret emergent signal structure **after** simulation. Interpretation methods (planned):

- cluster analysis on emitted signal parameters
- graph homology between per-creature `sound_activates` subgraphs
- drift metrics between settlements
- lineage trees of signal families
- correlation with landscape features (researcher labels: *river*, *shelter*, moisture traces)

All interpretation labels are external. Export pipelines ([26_research_tools_and_observability.md](26_research_tools_and_observability.md)) may attach researcher metadata; creature runtime may not.

## Transcripts

A **transcript** is a researcher-facing time-ordered record of emissions and activations — not a creature-readable text.

Planned transcript fields:

- tick, sender `signature`, listener `signature` (if resolved)
- signal parameters (frequency, duration, rhythm, intensity, contour)
- activated node IDs / concept cluster IDs (export view)
- regulatory delta following activation
- researcher annotation slot (optional; never fed back to simulation)

Transcripts support narrative extraction queries ([27_narrative_extraction_and_interpretation.md](27_narrative_extraction_and_interpretation.md) § Example Queries).

## Sonification

**Sonification** maps simulation signal data to audio for human listening — an analysis and artistic channel, not creature perception.

Uses:

- validate that signal families are acoustically distinct
- hear drift and dialect divergence over deep time
- exhibition and installation outputs ([17_analysis_and_visualization.md](17_analysis_and_visualization.md))

Sonification does not define creature signal space; it reveals it to researchers.

## Language Emergence

**Language emergence** is the research outcome when all prior sections stabilize together:

- multidimensional signal space with mutable families
- shared compressed concepts activated by sound
- sequences with proto-syntactic ordering
- trust-mediated transmission across lineages
- dialects and partial mutual intelligibility
- narratives extractable from historical transcripts

Emergence is graded, not binary. The simulation may produce protolanguage-like structure without human-natural language. Success is measured by predictive coupling between creatures — not by vocabulary size.

Language is **never** hardcoded in creature code. No word list, no grammar module, no semantic type enum.

## Current implementation

| Component | Location | Status |
|-----------|----------|--------|
| `sound_ambient`, `sound_calls` sensors | `creatures/sensors.rs` | Read from propagated `SoundEvent`s |
| `SoundEvent`, world sound queue | `world/sound.rs` | Implemented |
| `Action::EmitSound` | `creatures/actions.rs` | Uses `genome.vocal_profile` (pitch, duration, amplitude, rhythm) |
| `SoundEvent::signal_family_id` | `world/sound.rs` | Hash of vocal profile for family clustering |
| `genome.vocal_profile` mutation | `creatures/genome.rs` | Offspring inherit slightly mutated profile |
| `CreatureSnapshot.vocal_profile` | `export/snapshots.rs` | Exported in world snapshots |
| `NodeKind::Sound`, `record_heard_sound` | `memory/graph.rs` | Implemented |
| `EdgeType::SoundActivates` | `memory/edges.rs` | Used |
| `Creature::signature` | `creatures/creature.rs` | Assigned at spawn |
| Tick log sound export | `export/logs.rs` | Count + optional slice |
| Listener signature resolution | — | Planned |
| `sound_activates` edge creation on hear | `memory/graph.rs` | Implemented |
| Signal family / drift tracking | `world/sound.rs` | `signal_family_id` from profile hash |
| Concept compression from sound | — | Planned |
| Transcripts / sonification export | — | Planned |

## Planned

- Propagation, attenuation, and occlusion in world
- Per-`signature` listener subgraphs and trust via edge confidence
- Settlement-scale dialect metrics
- Communication transcript export
- Sonification tooling in `analysis/`
- Integration with narrative extraction ([27_narrative_extraction_and_interpretation.md](27_narrative_extraction_and_interpretation.md))
- Research interpretation scripts (clustering, homology, drift)

## Cross-references

| Topic | Doc |
|-------|-----|
| Communication foundation | [10_communication.md](10_communication.md) |
| Memory graph | [06_memory.md](06_memory.md) |
| Concepts & compression | [07_concepts.md](07_concepts.md) |
| Sleep & rehearsal | [09_sleep.md](09_sleep.md) |
| Culture | [16_culture.md](16_culture.md) |
| Settlements | [22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md) |
| Narrative extraction | [27_narrative_extraction_and_interpretation.md](27_narrative_extraction_and_interpretation.md) |
| Research tools | [26_research_tools_and_observability.md](26_research_tools_and_observability.md) |
| Exploration & migration | [21_exploration_and_discovery.md](21_exploration_and_discovery.md) |
| Inheritance | [13_inheritance.md](13_inheritance.md) |

## Core Principle

Creatures do not speak words.

They emit traces, activate graphs, and shift each other's predictions.

What researchers call language is the long shadow of that process — compressed, sequenced, trusted, and transmitted across time.
