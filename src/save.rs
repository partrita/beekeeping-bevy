use crate::colony::Colony;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SaveData {
    pub eggs: f32,
    pub larvae: f32,
    pub pupae: f32,
    pub workers: f32,
    pub drones: f32,
    pub honey: f32,
    pub pollen: f32,
    pub built: f32,
    pub total_emerged: u32,
    pub jars: u32,
    pub day_count: u32,
    pub saved_at: u64,
}

impl SaveData {
    pub fn away_secs(&self) -> f32 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(self.saved_at);
        now.saturating_sub(self.saved_at) as f32
    }
}

fn path() -> std::path::PathBuf {
    std::path::PathBuf::from("save.json")
}

fn bak_path() -> std::path::PathBuf {
    std::path::PathBuf::from("save.bak")
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Plain key=value format (no serde dep): save.json + save.bak rotation.
pub fn save(col: &Colony, jars: u32, day_count: u32) {
    let body = format!(
        "eggs={}\nlarvae={}\npupae={}\nworkers={}\ndrones={}\nhoney={}\npollen={}\nbuilt={}\nemerged={}\njars={}\nday={}\nts={}\n",
        col.eggs,
        col.larvae,
        col.pupae,
        col.workers,
        col.drones,
        col.honey,
        col.pollen,
        col.built,
        col.total_emerged,
        jars,
        day_count,
        now_secs()
    );
    // Rotate previous save to .bak (best effort).
    let _ = std::fs::copy(path(), bak_path());
    let _ = std::fs::write(path(), body);
}

pub fn load() -> Option<SaveData> {
    let text = std::fs::read_to_string(path())
        .or_else(|_| std::fs::read_to_string(bak_path()))
        .ok()?;
    let get = |key: &str| -> Option<f32> {
        for line in text.lines() {
            if let Some(v) = line.strip_prefix(&format!("{key}=")) {
                return v.trim().parse().ok();
            }
        }
        None
    };
    let get_u = |key: &str| -> u32 { get(key).unwrap_or(0.0) as u32 };
    let saved_at = get("ts").unwrap_or(0.0) as u64;
    // Empty/garbage file → None.
    if saved_at == 0 && get("eggs").is_none() {
        return None;
    }
    Some(SaveData {
        eggs: get("eggs").unwrap_or(4.0),
        larvae: get("larvae").unwrap_or(6.0),
        pupae: get("pupae").unwrap_or(6.0),
        workers: get("workers").unwrap_or(240.0),
        drones: get("drones").unwrap_or(12.0),
        honey: get("honey").unwrap_or(8.0),
        pollen: get("pollen").unwrap_or(4.0),
        built: get("built").unwrap_or(40.0),
        total_emerged: get_u("emerged"),
        jars: get_u("jars"),
        day_count: get_u("day").max(1),
        saved_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_away_secs_nonnegative() {
        let s = SaveData {
            eggs: 0.0,
            larvae: 0.0,
            pupae: 0.0,
            workers: 10.0,
            drones: 0.0,
            honey: 5.0,
            pollen: 5.0,
            built: 40.0,
            total_emerged: 0,
            jars: 0,
            day_count: 1,
            saved_at: now_secs(),
        };
        assert!(s.away_secs() >= 0.0);
    }
}
