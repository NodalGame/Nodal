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
        puzzles: HashMap<Uuid, String>,
    }

    impl PuzzleManager {
        pub fn new() -> Self {
            PuzzleManager {
                puzzles: HashMap::new(),
            }
        }

        pub fn populate(&mut self, directory_path: &Path) -> serde_json::Result<()> {
            for entry in WalkDir::new(directory_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
            {
                let path = entry.path();

                println!("{}", path.to_string_lossy());

                // Open the file
                let mut file = File::open(path).expect("Failed to open the file");

                // Read the file contents into a string
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Failed to read the file");

                let puzzle: Puzzle = match serde_json::from_str(&contents) {
                    Ok(data) => data,
                    Err(e) => panic!("Failed to load puzzle: {:?}", e),
                };
                self.add_puzzle(puzzle.uuid, path.to_string_lossy().into_owned());
            }
            Ok(())
        }

        fn add_puzzle(&mut self, uuid: Uuid, path: String) {
            println!("inserting puzzle into manager {} : {}", uuid, path);
            self.puzzles.insert(uuid, path);
        }

        pub fn load_puzzle(&self, uuid: &Uuid) -> Option<Puzzle> {
            self.puzzles.get(uuid).and_then(|path| {
                let data = fs::read_to_string(path).ok()?;
                serde_json::from_str(&data).ok()
            })
        }

        pub fn get_puzzle_uuids(&self) -> Vec<Uuid> {
            self.puzzles.keys().cloned().collect()
        }
    }
}
