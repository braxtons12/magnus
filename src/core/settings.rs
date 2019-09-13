use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    graphics: GraphicsSettings
}

impl Settings {
    pub fn new(name: &str) -> Settings {
        let mut filename = String::from(name);
        filename.push_str(".json");
        let filename_temp = filename.clone();
        let file = fs::read_to_string(filename_temp);
        match file {
            Ok(i) => {
                let set_file = serde_json::from_str(&i);
                match set_file {
                    Ok(j) => j,
                    _ => {
                        debug!("Error deserializing settings json");
                        let settings = Settings { graphics: GraphicsSettings::new(None, None, None, None) };
                        let json = serde_json::to_string(&settings).unwrap();
                        let filename_temp2 = filename.clone();
                        match fs::write(filename_temp2, json) {
                            Ok(_) => debug!("New settings file written to disk"),
                            _ => debug!("Error writing settings file to disk: {}", filename)
                        };
                        settings
                    }
                }
            }
            _ => {
                debug!("Error reading settings json from disk: {}", filename);
                let settings = Settings { graphics: GraphicsSettings::new(None, None, None, None) };
                let json = serde_json::to_string(&settings).unwrap();
                let filename_temp2 = filename.clone();
                match fs::write(filename_temp2, json) {
                    Ok(_) => debug!("New settings file written to disk"),
                    _ => debug!("Error writing settings file to disk: {}", filename)
                };
                settings
            }
        }
    }

    pub fn graphics(&self) -> GraphicsSettings {
        self.graphics
    }

    pub fn set_graphics_mode(&mut self, mode: GraphicsMode) {
        self.graphics.mode = mode;
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct GraphicsSettings {
    width: i32,
    height: i32,
    mode: GraphicsMode,
    vulkan_id: usize
}

impl GraphicsSettings {
    pub fn new(id: Option<usize>, width: Option<i32>, height: Option<i32>, mode: Option<GraphicsMode>) -> GraphicsSettings {
        GraphicsSettings { 
            width: match width {
                Some(i) => i,
                None => 1280
            },
            height: match height {
                Some(i) => i,
                None => 720
            },
            mode: match mode {
                Some(i) => i,
                None => GraphicsMode::OpenGL
            },
            vulkan_id: match id {
            Some(i) => i,
            None => 0
            }
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn mode(&self) -> GraphicsMode {
        self.mode
    }

    pub fn vulkan_id(&self) -> usize {
        self.vulkan_id
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum GraphicsMode {
    DirectX,
    OpenGL,
    Vulkan
}
