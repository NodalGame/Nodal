pub mod puzzle_manager {
    use bevy::prelude::*;

    use std::{
        fs::{self, File},
        io::Read,
        path::Path,
    };

    use bevy::utils::HashMap;
    use uuid::Uuid;
    use walkdir::WalkDir;

    use crate::Puzzle;

    #[derive(Resource, Debug)]
    pub struct PuzzleManager {
        puzzles: HashMap<Uuid, String>, // TODO campaign puzzles need relationship with "parent" puzzle needing solve prior to loading
    }

    impl PuzzleManager {
        pub fn new() -> Self {
            PuzzleManager {
                puzzles: HashMap::new(),
            }
        }

        /// Populates locally stored puzzles (from ../assets/campaign/puzzles/)
        pub fn populate_campaign(&mut self) -> serde_json::Result<()> {
            let mut puzzle_idx = 0;
            loop {
                let path = "../assets/campaign/puzzles/puzzle".to_owned() + &puzzle_idx.to_string() + ".json";
                let puzzle: Puzzle = match serde_json::from_str(&path) {
                    Ok(data) => data,
                    Err(e) => break,
                };

                self.add_puzzle(puzzle.uuid, path.to_string());
                puzzle_idx += 1;
            }
            Ok(())
        }

        pub fn populate(&mut self, directory_path: &String) -> serde_json::Result<()> {
            // for entry in WalkDir::new(directory_path)
            //     .into_iter()
            //     .filter_map(|e| e.ok())
            //     .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
            // {
                // let path = entry.path();

                // println!("{}", path.to_string_lossy());

                // // Open the file
                // let mut file = File::open(path).expect("Failed to open the file");

                // // Read the file contents into a string
                // let mut contents = String::new();
                // file.read_to_string(&mut contents)
                //     .expect("Failed to read the file");
            let mut puzzle_idx = 0;
            loop {
                let path = directory_path.clone() + "/puzzle" + &puzzle_idx.to_string() + ".json";
                let puzzle: Puzzle = match serde_json::from_str(&path) {
                    Ok(data) => data,
                    Err(e) => break,
                };
                
                self.add_puzzle(puzzle.uuid, path);
                puzzle_idx += 1;
            }
            // }
            Ok(())
        }

        fn add_puzzle(&mut self, uuid: Uuid, path: String) {
            self.puzzles.insert(uuid, path);
        }

        pub fn load_puzzle(&self, uuid: &Uuid) -> Option<Puzzle> {
            self.puzzles.get(uuid).and_then(|path| {
                let data = fs::read_to_string(path).ok()?;
                serde_json::from_str(&data).ok()
            })
        }

        pub fn get_puzzle_uuids(&self) -> Vec<Uuid> {
            let mut uuids = self
                .puzzles
                .iter()
                .map(|(&uuid, _)| uuid)
                .collect::<Vec<Uuid>>();
            uuids.sort_by(|a, b| {
                let path_a = self.puzzles.get(a).unwrap();
                let path_b = self.puzzles.get(b).unwrap();
                path_a.cmp(path_b)
            });
            uuids
        }
    }
}
