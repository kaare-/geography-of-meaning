use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use crate::creatures::Creature;

/// Append creature positions on a schedule for 2D trajectory maps (x–y plane).
pub struct TrajectoryWriter {
    file: std::fs::File,
    every: u64,
    track_ids: Option<HashSet<u64>>,
}

impl TrajectoryWriter {
    pub fn open(path: &Path, every: u64, track_ids: Option<Vec<u64>>) -> io::Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        writeln!(
            file,
            "tick,creature_id,x,y,z,energy,hydration,sleeping"
        )?;
        Ok(Self {
            file,
            every,
            track_ids: track_ids.map(|ids| ids.into_iter().collect()),
        })
    }

    pub fn record(&mut self, tick: u64, creatures: &[Creature]) -> io::Result<()> {
        if self.every == 0 || tick % self.every != 0 {
            return Ok(());
        }
        for creature in creatures {
            if let Some(ref ids) = self.track_ids {
                if !ids.contains(&creature.id) {
                    continue;
                }
            }
            writeln!(
                self.file,
                "{},{},{:.3},{:.3},{:.3},{:.4},{:.4},{}",
                tick,
                creature.id,
                creature.position.x,
                creature.position.y,
                creature.position.z,
                creature.regulatory.energy,
                creature.regulatory.hydration,
                u8::from(creature.sleep.sleeping),
            )?;
        }
        self.file.flush()?;
        Ok(())
    }
}
