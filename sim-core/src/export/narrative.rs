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
    pub concept_spike_count: usize,
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

    for entry in tick_logs {
        total_births += entry.births.len();
        total_deaths += entry.deaths.len();
        total_digs += entry.action_counts.dig_count;
        total_concepts_formed += entry.concepts_formed;

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

    NarrativeSummary {
        events,
        total_births,
        total_deaths,
        total_digs,
        total_concepts_formed,
        concept_spike_count,
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
