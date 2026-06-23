use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::export::logs::TickLogEntry;
use crate::export::ExportError;

const CONCEPT_SPIKE_THRESHOLD: u32 = 2;

#[derive(Debug, Serialize)]
pub struct NarrativeEvent {
    pub kind: &'static str,
    pub tick: u64,
    pub significance: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creature_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NarrativeSummary {
    pub events: Vec<NarrativeEvent>,
    pub total_births: usize,
    pub total_deaths: usize,
    pub total_digs: u32,
    pub total_concepts_formed: u32,
    pub population_concept_count: usize,
    /// Highest sum of `concept_count` across all creatures in a single tick.
    pub peak_population_concept_count: usize,
    pub peak_population_concept_count_tick: u64,
    pub concept_spike_count: usize,
    pub mean_tick_displacement: f32,
    pub mean_novel_sensor_fraction: f32,
    pub total_imagination_events: u32,
}

pub fn extract_narrative(tick_logs: &[TickLogEntry]) -> NarrativeSummary {
    let mut events = Vec::new();
    let mut saw_birth = false;
    let mut saw_death = false;
    let mut saw_dig = false;
    let mut total_births = 0usize;
    let mut total_deaths = 0usize;
    let mut total_digs = 0u32;
    let mut total_concepts_formed = 0u32;
    let mut concept_spike_count = 0usize;
    let mut peak_population_concept_count = 0usize;
    let mut peak_population_concept_count_tick = 0u64;
    let mut displacement_sum = 0.0f32;
    let mut novel_fraction_sum = 0.0f32;
    let mut total_imagination_events = 0u32;

    for entry in tick_logs {
        total_births += entry.births.len();
        total_deaths += entry.deaths.len();
        total_digs += entry.action_counts.dig_count;
        total_concepts_formed += entry.concepts_formed;
        total_imagination_events += entry.imagination_events;
        displacement_sum += entry.mean_displacement;
        novel_fraction_sum += entry.novel_sensor_fraction;

        let tick_concept_total: usize = entry.creatures.iter().map(|c| c.concept_count).sum();
        if tick_concept_total > peak_population_concept_count {
            peak_population_concept_count = tick_concept_total;
            peak_population_concept_count_tick = entry.tick;
        }

        if !entry.births.is_empty() && !saw_birth {
            saw_birth = true;
            let birth = &entry.births[0];
            events.push(NarrativeEvent {
                kind: "first_birth",
                tick: entry.tick,
                significance: 1.0,
                creature_id: Some(birth.offspring_id),
                count: None,
                detail: Some(format!("parent {}", birth.parent_id)),
            });
        }

        if !entry.deaths.is_empty() && !saw_death {
            saw_death = true;
            let death = &entry.deaths[0];
            events.push(NarrativeEvent {
                kind: "first_death",
                tick: entry.tick,
                significance: 1.0,
                creature_id: Some(death.creature_id),
                count: None,
                detail: Some(format!("{:?}", death.cause)),
            });
        }

        if entry.action_counts.dig_count > 0 && !saw_dig {
            saw_dig = true;
            events.push(NarrativeEvent {
                kind: "first_dig",
                tick: entry.tick,
                significance: 1.0,
                creature_id: None,
                count: Some(entry.action_counts.dig_count),
                detail: None,
            });
        }

        if entry.concepts_formed >= CONCEPT_SPIKE_THRESHOLD {
            concept_spike_count += 1;
            let significance = (entry.concepts_formed as f32 / CONCEPT_SPIKE_THRESHOLD as f32)
                .min(3.0);
            events.push(NarrativeEvent {
                kind: "concept_formation_spike",
                tick: entry.tick,
                significance,
                creature_id: None,
                count: Some(entry.concepts_formed),
                detail: None,
            });
        }

        if entry.births.len() >= 3 {
            events.push(NarrativeEvent {
                kind: "birth_burst",
                tick: entry.tick,
                significance: entry.births.len() as f32,
                creature_id: None,
                count: Some(entry.births.len() as u32),
                detail: None,
            });
        }
    }

    // End-of-run sum of concept_count across living creatures (not peak).
    let population_concept_count: usize = tick_logs
        .last()
        .map(|e| e.creatures.iter().map(|c| c.concept_count).sum())
        .unwrap_or(0);

    NarrativeSummary {
        events,
        total_births,
        total_deaths,
        total_digs,
        total_concepts_formed,
        population_concept_count,
        peak_population_concept_count,
        peak_population_concept_count_tick,
        concept_spike_count,
        mean_tick_displacement: if tick_logs.is_empty() {
            0.0
        } else {
            displacement_sum / tick_logs.len() as f32
        },
        mean_novel_sensor_fraction: if tick_logs.is_empty() {
            0.0
        } else {
            novel_fraction_sum / tick_logs.len() as f32
        },
        total_imagination_events,
    }
}

pub fn write_narrative_summary(
    tick_logs: &[TickLogEntry],
    path: &Path,
) -> Result<(), ExportError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let summary = extract_narrative(tick_logs);
    let json = serde_json::to_string_pretty(&summary)?;
    fs::write(path, json)?;
    Ok(())
}
