use std::ffi::OsStr;

pub mod cli;

pub struct App {
    pub notes: Vec<Note>,
}

pub struct Note {
    pub name: String,
    pub content: String,
}

impl App {
    pub fn new() -> Self {
        App { notes: Vec::new() }
    }

    pub fn load(
        &mut self,
        notes_dir: &std::path::PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for entry in std::fs::read_dir(notes_dir)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if Some(OsStr::new("txt")) != path.extension() {
                continue;
            }

            let name = path.file_stem().unwrap().to_str().unwrap().to_string();

            let content = std::fs::read_to_string(path)?;
            self.notes.push(Note { name, content });
        }

        Ok(())
    }

    pub fn save(&self, notes_dir: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        for note in &self.notes {
            let path = notes_dir.join(note.name.clone() + ".txt");
            std::fs::write(path, &note.content)?;
        }

        Ok(())
    }
}
