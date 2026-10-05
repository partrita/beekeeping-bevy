use bevy::prelude::*;

// Colony simulation: castes, brood pipeline, stores, comb building.
//
// The design is a zen garden: one immortal queen, self-balancing castes,
// no extinction. Foragers visibly haul every drop of nectar home (see
// hiveview), nurses tend brood, builders raise wax, drones drift.
// The beekeeper watches; intervention is optional.

/// Nectar/pollen dropped off by returning foragers (filled by the
/// walk system, drained by the tick below; one frame of lag is harmless).
#[derive(Debug, Clone, Default)]
pub struct Deliveries {
    pub nectar: f32,
    pub pollen: f32,
}

/// Pressed honey jars on the beekeeper's shelf.
#[derive(Debug, Clone, Default, Resource)]
pub struct HoneyJars(pub u32);

#[derive(Debug, Clone, Component)]
pub struct Colony {
    pub eggs: f32,
    pub larvae: f32,
    pub pupae: f32,
    pub workers: f32,
    pub drones: f32,
    pub honey: f32,
    pub pollen: f32,
    pub built: f32, // wax cells raised (0..=126)
    pub total_emerged: u32,
    pub emerged_frac: f32,
    pub lay_display: f32, // eggs/sec right now (queen animation + readout)
    pub starving: bool,
}

impl Default for Colony {
    fn default() -> Self {
        // Must fit the starting 40 cells with room to lay:
        // 4+6+6 brood + 8 honey + 4 pollen = 28, leaving ~12 free.
        Self {
            eggs: 4.0,
            larvae: 6.0,
            pupae: 6.0,
            workers: 240.0,
            drones: 12.0,
            honey: 8.0,
            pollen: 4.0,
            built: 40.0,
            total_emerged: 0,
            emerged_frac: 0.0,
            lay_display: 0.0,
            starving: false,
        }
    }
}

impl Colony {
    pub const MAX_CELLS: f32 = 126.0;
    pub const EGG_TIME: f32 = 6.0;
    pub const LARVA_TIME: f32 = 10.0;
    pub const PUPA_TIME: f32 = 14.0;
    pub const WORKER_LIFE: f32 = 240.0;
    pub const DRONE_LIFE: f32 = 300.0;
    pub const DRONE_FRAC: f32 = 0.08;
    pub const POLLEN_CAP: f32 = 120.0;
    /// Honey units needed to press one jar.
    pub const HONEY_PER_JAR: f32 = 10.0;
    /// The queen keeps a reserve the beekeeper cannot harvest.
    pub const HONEY_RESERVE: f32 = 12.0;

    pub fn brood(&self) -> f32 {
        self.eggs + self.larvae + self.pupae
    }

    /// Free wax cells for laying and storage (stores never share a cell
    /// with brood: honey AND pollen both occupy comb).
    pub fn empty_cells(&self) -> f32 {
        (self.built - self.brood() - self.honey - self.pollen).clamp(0.0, Self::MAX_CELLS)
    }

    /// Forager share rises automatically when stores run low.
    pub fn forager_ratio(&self) -> f32 {
        if self.honey < 10.0 {
            0.7
        } else {
            0.5
        }
    }

    pub fn foragers(&self) -> f32 {
        self.workers * self.forager_ratio()
    }

    pub fn nurses(&self) -> f32 {
        self.workers * (1.0 - self.forager_ratio())
    }

    /// Harvest pressed honey into jars. Returns jars filled.
    pub fn harvest(&mut self) -> u32 {
        let spare = (self.honey - Self::HONEY_RESERVE).max(0.0);
        let jars = (spare / Self::HONEY_PER_JAR).floor() as u32;
        self.honey -= jars as f32 * Self::HONEY_PER_JAR;
        jars
    }

    /// Advance the colony by `dt` seconds.
    ///
    /// * `night` — bees rest; laying and foraging slow down.
    /// * `richness` — meadow nectar flow (0.2..2.0).
    /// * `drop` — visual forager deliveries, drained here.
    pub fn tick(&mut self, dt: f32, night: bool, richness: f32, drop: &mut Deliveries) {
        let dt = dt.max(0.0);
        if dt <= 0.0 {
            return;
        }
        let night_f = if night { 0.4 } else { 1.0 };
        // Cold small clusters raise brood slowly.
        let warmth = (self.workers / 250.0).clamp(0.3, 1.0);

        // 1. Laying: the queen never fully stops (zen garden).
        let rate = (4.0 * warmth * night_f).max(0.5);
        self.lay_display = rate;
        self.eggs += (rate * dt).min(self.empty_cells());

        // 2. Hatching: egg -> larva.
        let hatch = (self.eggs * (dt / Self::EGG_TIME)).min(self.eggs);
        self.eggs -= hatch;
        self.larvae += hatch;

        // 3. Nursing: larva -> pupa needs food + nurses.
        let fed = self.honey > 0.5 && self.pollen > 0.5;
        let food_f = if fed { 1.0 } else { 0.15 };
        let nurse_f = if self.larvae > 1.0 {
            (self.nurses() / (self.larvae * 0.8 + 1.0)).clamp(0.2, 1.0)
        } else {
            1.0
        };
        let pupate = (self.larvae * (dt / Self::LARVA_TIME) * food_f * nurse_f).min(self.larvae);
        self.larvae -= pupate;
        self.pupae += pupate;
        self.honey = (self.honey - pupate * 0.2).max(0.0);
        self.pollen = (self.pollen - pupate * 0.15).max(0.0);

        // 4. Emerging: pupa -> worker/drone (drone brood culled if too many).
        let emerge = (self.pupae * (dt / Self::PUPA_TIME)).min(self.pupae);
        let drone_over = self.drones > self.workers * 0.25 + 10.0;
        let new_drones = if drone_over { 0.0 } else { emerge * Self::DRONE_FRAC };
        let new_workers = emerge - emerge * Self::DRONE_FRAC;
        self.pupae -= emerge;
        self.workers += new_workers;
        self.drones += new_drones;
        self.emerged_frac += new_workers + new_drones;
        let whole = self.emerged_frac.floor();
        self.total_emerged += whole as u32;
        self.emerged_frac -= whole;

        // 5. Foraging: passive trickle + visual drop-offs from returning bees.
        let day_f = if night { 0.2 } else { 1.0 };
        let passive = self.foragers() * 0.004 * richness * day_f * dt;
        self.honey += passive.min(self.empty_cells());
        self.pollen = (self.pollen + self.foragers() * 0.0012 * richness * dt).min(Self::POLLEN_CAP);
        self.honey += drop.nectar.min(self.empty_cells().max(0.0));
        self.pollen = (self.pollen + drop.pollen).min(Self::POLLEN_CAP);
        drop.nectar = 0.0;
        drop.pollen = 0.0;

        // 6. Building: wax rises at the comb edge, paid in honey.
        let room = Self::MAX_CELLS - self.built;
        if room > 0.0 && self.honey > 1.0 {
            let want = self.nurses() * 0.5 * 0.05 * dt;
            let affordable = self.honey / 0.3;
            let built = want.min(room).min(affordable);
            self.built += built;
            self.honey = (self.honey - built * 0.3).max(0.0);
        }

        // 7. Aging: the hungry evict drones first.
        let mut worker_death = self.workers * (dt / Self::WORKER_LIFE);
        let mut drone_death = self.drones * (dt / Self::DRONE_LIFE);
        if self.honey <= 0.5 {
            drone_death += self.drones * dt * 0.05;
            worker_death *= 2.0;
        }
        self.workers = (self.workers - worker_death).max(0.0);
        self.drones = (self.drones - drone_death).max(0.0);

        self.starving = self.honey <= 0.5 && (self.larvae > 1.0 || self.workers > 10.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step(c: &mut Colony, secs: f32, night: bool, richness: f32) {
        let mut drop = Deliveries::default();
        let mut t = 0.0;
        while t < secs {
            c.tick(0.1, night, richness, &mut drop);
            t += 0.1;
        }
    }

    #[test]
    fn test_pipeline_grows_colony() {
        let mut c = Colony::default();
        step(&mut c, 40.0, false, 1.0);
        assert!(c.total_emerged > 0, "nothing emerged: {c:?}");
        assert!(c.workers > 100.0, "colony collapsed: {c:?}");
        assert!(c.brood() > 0.0, "pipeline empty: {c:?}");
    }

    #[test]
    fn test_deliveries_become_honey() {
        let mut c = Colony::default();
        c.built = Colony::MAX_CELLS;
        c.honey = 10.0;
        c.eggs = 0.0;
        c.larvae = 0.0;
        c.pupae = 0.0;
        let before = c.honey;
        let mut drop = Deliveries { nectar: 5.0, pollen: 2.0 };
        c.tick(0.1, false, 0.0, &mut drop);
        assert!(c.honey > before, "drop-off lost: {c:?}");
        assert_eq!(drop.nectar, 0.0);
    }

    #[test]
    fn test_starvation_flags_but_recovers() {
        let mut c = Colony::default();
        c.honey = 0.0;
        c.pollen = 0.0;
        c.larvae = 40.0;
        step(&mut c, 8.0, false, 0.0);
        assert!(c.starving, "no hunger flag: {c:?}");
        // The garden never dies: queen keeps laying, foragers keep flying.
        step(&mut c, 120.0, false, 1.5);
        assert!(c.workers > 20.0, "colony died: {c:?}");
    }

    #[test]
    fn test_full_comb_blocks_laying() {
        let mut c = Colony::default();
        c.built = Colony::MAX_CELLS;
        c.honey = Colony::MAX_CELLS - 2.0;
        c.eggs = 0.0;
        c.larvae = 0.0;
        c.pupae = 0.0;
        step(&mut c, 5.0, false, 1.0);
        assert!(c.brood() <= 6.0, "laid beyond wax: {c:?}");
    }

    #[test]
    fn test_harvest_keeps_reserve() {
        let mut c = Colony::default();
        c.honey = 50.0;
        let jars = c.harvest();
        assert_eq!(jars, 3); // (50-12)/10
        assert!((c.honey - 20.0).abs() < 1e-4);
        // Below reserve: nothing to take.
        c.honey = 15.0;
        assert_eq!(c.harvest(), 0);
        assert!((c.honey - 15.0).abs() < 1e-4);
    }

    #[test]
    fn test_night_slows_laying() {
        let mut day = Colony::default();
        let mut night_c = Colony::default();
        step(&mut day, 10.0, false, 1.0);
        step(&mut night_c, 10.0, true, 1.0);
        assert!(night_c.lay_display < day.lay_display);
    }
}
