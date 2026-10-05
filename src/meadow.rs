use bevy::prelude::*;
use rand::Rng;
use crate::colony::{Colony, Deliveries};
use crate::hiveview::PendingDrops;

// Interior-only sim core: day/night + weather dice drive the colony.
// No meadow, no flowers, no player — the hive IS the world.

// Keep DayNight timings per shared understanding (90s day / 30s night).
#[derive(Resource)]
pub struct DayNight {
    pub t: f32,
    pub day_len: f32,
    pub night_len: f32,
    pub day_count: u32,
}

impl Default for DayNight {
    fn default() -> Self {
        Self {
            t: 0.0,
            day_len: 90.0,
            night_len: 30.0,
            day_count: 1,
        }
    }
}

impl DayNight {
    pub fn is_night(&self) -> bool {
        self.t >= self.day_len
    }

    pub fn label(&self, weather: &Weather) -> String {
        let base = if self.is_night() {
            "🌙 밤 (벌들이 모여 쉽니다)"
        } else {
            "☀️ 낮 (외역벌 출동)"
        };
        format!("{} · {} · {}일차", base, weather.label(), self.day_count)
    }

    #[cfg(test)]
    pub fn set_t(&mut self, t: f32) {
        self.t = t;
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Weather {
    #[default]
    Sunny,
    Cloudy,
    Rain,
}

impl Weather {
    pub fn multiplier(self) -> f32 {
        match self {
            Weather::Sunny => 1.2,
            Weather::Cloudy => 1.0,
            Weather::Rain => 0.5,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Weather::Sunny => "☀️맑음",
            Weather::Cloudy => "☁️흐림",
            Weather::Rain => "🌧️비",
        }
    }

    pub fn roll(rng: &mut rand::rngs::ThreadRng) -> Self {
        let v: f32 = rng.gen_range(0.0..1.0);
        if v < 0.6 {
            Weather::Sunny
        } else if v < 0.9 {
            Weather::Cloudy
        } else {
            Weather::Rain
        }
    }
}

pub fn tick_daynight(time: Res<Time>, mut day: ResMut<DayNight>, mut weather: ResMut<Weather>) {
    day.t += time.delta_secs();
    if day.t >= day.day_len + day.night_len {
        day.t -= day.day_len + day.night_len;
        day.day_count += 1;
        let mut rng = rand::thread_rng();
        *weather = Weather::roll(&mut rng);
    }
}

/// Single-screen setup: camera + one colony. Interior visuals are spawned
/// by hiveview at startup (no OnEnter round-trip anymore).
pub fn setup_hive(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((Colony::default(), Transform::default()));
}

/// Colony ticks every frame from weather multiplier (flowers deleted).
pub fn tick_colony(
    time: Res<Time>,
    night: Res<DayNight>,
    weather: Res<Weather>,
    mut pending: ResMut<PendingDrops>,
    mut hive: Query<&mut Colony>,
) {
    let richness = weather.multiplier();
    let mut drops = Deliveries {
        nectar: pending.nectar,
        pollen: pending.pollen,
    };
    pending.nectar = 0.0;
    pending.pollen = 0.0;
    if let Ok(mut col) = hive.get_single_mut() {
        col.tick(time.delta_secs(), night.is_night(), richness, &mut drops);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_night_falls_and_lifts() {
        let mut d = DayNight::default();
        assert!(!d.is_night());
        d.set_t(95.0);
        assert!(d.is_night());
        d.set_t(125.0);
        assert!(d.is_night());
    }

    #[test]
    fn test_weather_multipliers() {
        assert!((Weather::Sunny.multiplier() - 1.2).abs() < 1e-5);
        assert!((Weather::Cloudy.multiplier() - 1.0).abs() < 1e-5);
        assert!((Weather::Rain.multiplier() - 0.5).abs() < 1e-5);
    }
}
