use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    graphics: GraphicsSettings
}

impl Settings {
    pub fn new(name: &str, graphics_mode: GraphicsMode) -> Settings {
        let mut filename = String::from(name);
        filename.push_str(".json");
        let filename_temp = filename.clone();
        let settings = Settings { graphics: GraphicsSettings::new(None, Some((800, 600)), Some(graphics_mode)) };
        let json = serde_json::to_string(&settings).unwrap();
        match fs::write(filename_temp, json) {
            Ok(_) => debug!("New settings file written to disk"),
            _ => debug!("Error writing settings file to disk: {}", filename)
        };
        settings
    }

    pub fn read(name: &str) -> Result<Settings, String>  {
        let mut filename = String::from(name);
        filename.push_str(".json");
        let file = fs::read_to_string(filename);
        match file {
            Ok(i) => match serde_json::from_str(&i) {
                Ok(j) => j,
                _ => Err("Failed to deserialize settings".to_string())
            },
            _ => Err("Failed to read settings from disk".to_string())
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
    width: u32,
    height: u32,
    mode: GraphicsMode,
    vulkan_id: usize
}

impl GraphicsSettings {
    pub fn new(id: Option<usize>, size: Option<(u32, u32)>, mode: Option<GraphicsMode>) -> GraphicsSettings {
        GraphicsSettings {
            width: match size {
                Some((w,_h)) => w,
                None => 1280
            },
            height: match size {
                Some((_w,h)) => h,
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
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
