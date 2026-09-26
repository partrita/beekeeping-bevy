use bevy::prelude::*;

#[derive(Resource)]
pub struct CentrifugeMinigame {
    pub is_active: bool,
    pub comb_species: String,
    pub current_rpm: f32,       // 0.0 to 100.0
    pub target_min: f32,        // 60.0
    pub target_max: f32,        // 85.0
    pub sweet_spot_timer: f32,  // Reaching 3.5s wins
    pub required_time: f32,
    pub success: bool,
    pub jars_available: u32,
}

impl Default for CentrifugeMinigame {
    fn default() -> Self {
        Self {
            is_active: false,
            comb_species: "Wildflower".to_string(),
            current_rpm: 0.0,
            target_min: 55.0,
            target_max: 80.0,
            sweet_spot_timer: 0.0,
            required_time: 3.5,
            success: false,
            jars_available: 5,
        }
    }
}

impl CentrifugeMinigame {
    pub fn pump(&mut self) {
        self.current_rpm = (self.current_rpm + 12.0).min(100.0);
    }

    pub fn tick(&mut self, delta: f32) {
        // Natural friction decay
        self.current_rpm = (self.current_rpm - delta * 20.0).max(0.0);

        if self.current_rpm >= self.target_min && self.current_rpm <= self.target_max {
            self.sweet_spot_timer += delta;
            if self.sweet_spot_timer >= self.required_time {
                self.success = true;
            }
        }
    }
}

#[derive(Resource)]
pub struct MicroscopeMinigame {
    pub is_active: bool,
    pub target_species: String,
    pub focus_dial: f32,         // 0 to 100
    pub magnification_dial: f32, // 0 to 100
    pub target_focus: f32,
    pub target_magnification: f32,
    pub analyzed: bool,
}

impl Default for MicroscopeMinigame {
    fn default() -> Self {
        Self {
            is_active: false,
            target_species: "common".to_string(),
            focus_dial: 20.0,
            magnification_dial: 20.0,
            target_focus: 65.0,
            target_magnification: 50.0,
            analyzed: false,
        }
    }
}

impl MicroscopeMinigame {
    pub fn check_alignment(&self) -> bool {
        (self.focus_dial - self.target_focus).abs() < 8.0
            && (self.magnification_dial - self.target_magnification).abs() < 8.0
    }
}
