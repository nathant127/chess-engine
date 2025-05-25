extern crate find_folder;

use std::ffi::OsStr;
use std::path::PathBuf;
use std::collections::HashMap;

use piston_window::G2dTexture;
use std::rc::Rc;
use std::fs;

pub(super) struct ResourceManager {
    resource_folders: Vec<PathBuf>,
    textures: HashMap<String, Rc<G2dTexture>>
}

impl ResourceManager {
    fn new(folder: &str) -> ResourceManager {
        

        ResourceManager {
            resource_folders: Vec::new(), 
            textures: HashMap::new()
        }
    }

    /** Adds all valid files within folder into Resource Manager */
    fn add_folder(&mut self, folder: &str) -> Result<(), String> {
        let resource_folder = 
        match find_folder::Search::ParentsThenKids(3, 3).for_folder(folder) {
            Ok(folder) => folder,
            Err(e) => return Err(format!("Resource Manager failed to find resource folder {:} with err: {:}", folder, e)),
        };

        self.resource_folders.push(resource_folder);

        let paths = match fs::read_dir(self.resource_folders.last().unwrap()) {
            Ok(paths) => paths,
            Err(_e) => return Ok(()),
        };
        for path in paths {
            let path = path.unwrap().path();
            if *path.extension().unwrap_or(OsStr::new("")) == *OsStr::new("png") {
                self.add_texture(&path);
            }
        }
        return Ok(());
    }

    fn add_texture(&mut self, path: &PathBuf) {
    }
}
