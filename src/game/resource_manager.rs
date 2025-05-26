extern crate find_folder;

use std::ffi::OsStr;
use std::path::PathBuf;
use std::collections::HashMap;

use piston_window::G2dTexture;
use std::rc::Rc;
use std::fs;

use piston_window::*;

pub(super) struct ResourceManager {
    resource_folders: Vec<PathBuf>,
    texture_context: G2dTextureContext,
    textures: HashMap<String, Rc<G2dTexture>>
}

impl ResourceManager {
    pub fn new(texture_context: G2dTextureContext) -> ResourceManager {
        let mut man = ResourceManager {
            resource_folders: Vec::new(), 
            texture_context: texture_context,
            textures: HashMap::new()
        };
        // Adds an empty texture in to be used as a default return
        man.textures.insert("default".to_string(), Rc::new(G2dTexture::empty(&mut man.texture_context).unwrap()));
        return man;
    }

    /** Adds all valid files within folder into Resource Manager */
    pub fn add_folder(&mut self, folder: &str) -> Result<(), String> {
        let resource_folder = 
        match find_folder::Search::ParentsThenKids(3, 3).for_folder(folder) {
            Ok(folder) => folder,
            Err(e) => {
                let e = format!("Resource Manager failed to find resource folder {:} with err: {:}", folder, e);
                println!("{e}");
                return Err(e);
            },
        };

        let paths = match fs::read_dir(&resource_folder) {
            Ok(paths) => paths,
            Err(_e) => return Ok(()),
        };
        for path in paths {
            let path = path.unwrap().path();
            if *path.extension().unwrap_or(OsStr::new("")) == *OsStr::new("png") {
                println!("found img: {:}", path.as_os_str().to_str().unwrap_or("err"));
                self.add_texture(&path);
            }
        }

        self.resource_folders.push(resource_folder);
        return Ok(());
    }

    fn add_texture(&mut self, path: &PathBuf) {

        let tex: Rc<G2dTexture> = Rc::new( 
            match G2dTexture::from_path(
                &mut self.texture_context,
                path,
                Flip::None,
                &TextureSettings::new()) {
                    Ok(tex) => tex,
                    Err(e) => {
                        println!("Failed to interpret {:} as texture with err: {:}", path.to_str().unwrap(), e);
                        return;
                    }
            },
        );

        let fname = path.file_name().unwrap().to_os_string().into_string().unwrap();

        self.textures.insert(fname, tex);
    }

    pub fn get_texture(&mut self, name: &str) -> Rc<G2dTexture> {
        let tex = self.textures.get(&name.to_string());

        // Either return the requested texture OR the default texture if name is invalid
        let tex = match tex {
            Some (tex) => tex,
            None => self.textures.get(&"default".to_string()).unwrap(),
        };
        return tex.clone();
    }   
}
