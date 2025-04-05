use bevy::prelude::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Config::load_config());
    }
}

#[derive(Resource)]
pub struct Config {
    pub iterations: u32,
    pub initial_map: Vec<Vec<bool>>,
    pub width: i32,
    pub height: i32,
    // pub spawn_alive_chance: f64,
    pub visuals: VisualisationConfig,
}

#[derive(serde::Deserialize)]
pub struct VisualisationConfig {
    pub cell_width: f32,
    pub cell_height: f32,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for VisualisationConfig {
    fn default() -> Self {
        Self {
            cell_width: 1.0,
            cell_height: 1.0,
            window_width: 1.0,
            window_height: 1.0,
        }
    }
}

impl Config {
    fn load_config() -> Config {
        let args = std::env::args().collect::<Vec<String>>();
        let initial_map_path = args
            .get(1)
            .expect("First program argument should be the path to the csv file with initial map");
        let iterations = args
            .get(2)
            .expect("Second program argument should be the number of iterations")
            .parse()
            .expect("Failed to parse number of iterations from second program argument");

        let initial_map: Vec<Vec<bool>> = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(initial_map_path)
            .expect("Failed to create file reader for initial map csv")
            .records()
            .map(|result| {
                result
                    .expect("Failed to read csv row")
                    .iter()
                    .map(|cell| cell == "1")
                    .collect()
            })
            .collect();

        #[cfg(feature = "windowed")]
        let visuals = match csv::ReaderBuilder::new().from_path("visualisation_config.csv") {
            Ok(mut reader) => reader
                .deserialize()
                .next()
                .expect("Failed to fetch first row of csv")
                .expect("Failed to deserialize into VisualisationConfig struct"),
            Err(_) => VisualisationConfig::default(),
        };

        #[cfg(feature = "headless")]
        let visuals = VisualisationConfig::default();

        Config {
            iterations,
            width: initial_map.len() as i32,
            height: initial_map[0].len() as i32,
            initial_map,
            visuals,
        }
    }
}
