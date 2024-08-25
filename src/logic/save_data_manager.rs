pub mod save_data_manager {
    use std::path::PathBuf;

    use dirs::data_dir;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::structs::immutable::solution::solution::Solution;

    #[derive(Serialize, Deserialize)]
    pub struct PuzzleSaveData {
        pub puzzle_uuid: Uuid,
        pub solution: Solution,
        pub solved: bool,
    }

    pub fn save_progress(puzzle_uuid: Uuid, solution: Solution, solved: bool) {
        // TODO if puzzle was previously solved and is not currently solved, don't save solution
        let save_data = PuzzleSaveData {
            puzzle_uuid,
            solution,
            solved,
        };

        let save_data_json = serde_json::to_string(&save_data);
        let save_data_path = get_save_data_path(&puzzle_uuid.to_string());

        match save_data_json {
            Ok(json) => {
                let _ = std::fs::write(save_data_path, json);
            }
            Err(error) => {
                eprintln!("Error serializing save data: {}", error);
            }
        }
    }

    pub fn load_progress(puzzle_uuid: Uuid) -> Option<PuzzleSaveData> {
        let save_data_path = get_save_data_path(&puzzle_uuid.to_string());

        let save_data_json = std::fs::read_to_string(save_data_path);
        match save_data_json {
            Ok(json) => {
                let save_data: PuzzleSaveData = serde_json::from_str(&json).unwrap();
                return Some(save_data);
            }
            Err(_error) => {
                return None;
            }
        }
    }

    pub fn is_solved(puzzle_uuid: Uuid) -> bool {
        let save_data_path = get_save_data_path(&puzzle_uuid.to_string());

        let save_data_json = std::fs::read_to_string(save_data_path);
        match save_data_json {
            Ok(json) => {
                let save_data: PuzzleSaveData = serde_json::from_str(&json).unwrap();
                return save_data.solved;
            }
            Err(_error) => {
                return false;
            }
        }
    }

    fn get_save_data_path(puzzle_uuid: &str) -> PathBuf {
        // Get the base directory for game data
        let base_dir = data_dir().expect("Failed to get data directory");

        // Construct the path specific to your game
        let game_dir = base_dir.join("Nodal").join("saves");

        // Ensure the directory exists
        std::fs::create_dir_all(&game_dir).expect("Failed to create game save directory");

        // Construct the full path for the specific puzzle save file
        let path = game_dir.join(format!("{}.json", puzzle_uuid));

        path
    }
}
