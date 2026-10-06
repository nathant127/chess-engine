extern crate find_folder;

use std::ffi::OsStr;
use std::path::PathBuf;
use std::collections::HashMap;

use piston_window::{G2dTexture, Glyphs};
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;

use piston_window::*;

type Font = Rc<RefCell<Glyphs>>;

thread_local! {
    pub static RESOURCE_MANAGER: RefCell<ResourceManager> = RefCell::new(ResourceManager::new());
}

pub struct ResourceManager {
    resource_folders: Vec<PathBuf>,
    texture_context: Option<G2dTextureContext>,
    textures: HashMap<String, Rc<G2dTexture>>,
    fonts: HashMap<String, Rc<RefCell<Glyphs>>>,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        let man = ResourceManager {
            resource_folders: Vec::new(), 
            texture_context: None,
            textures: HashMap::new(),
            fonts: HashMap::new(),
        };        
        return man;
    }

    pub fn init(&mut self, tex_context: G2dTextureContext) {
        self.texture_context = Some(tex_context);
        // Adds an empty texture in to be used as a default return
        self.textures.insert("default".to_string(), Rc::new(G2dTexture::empty(&mut self.texture_context.as_mut().unwrap()).unwrap()));
    }

    /** Adds all valid files within folder into Resource Manager */
    pub fn add_folder(&mut self, folder: &str) -> Result<(), String> {
        if self.texture_context.is_none() {
            return Err("Uninitialized".to_string());
        }
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
                self.add_texture(&path);
            }
        }

        self.resource_folders.push(resource_folder);
        return Ok(());
    }

    fn add_texture(&mut self, path: &PathBuf) {
        if self.texture_context.is_none() {
            println!("Uninitialized");
            return;
        }
        let tex: Rc<G2dTexture> = Rc::new( 
            match G2dTexture::from_path(
                &mut self.texture_context.as_mut().unwrap(),
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

    pub fn get_texture(&self, name: &str) -> Rc<G2dTexture> {
        let tex = self.textures.get(&name.to_string());

        // Either return the requested texture OR the default texture if name is invalid
        let tex = match tex {
            Some (tex) => tex,
            None => {
                println!("Failed to retrieve Texture"); 
                self.textures.get(&"default".to_string()).unwrap()
            },
        };
        return tex.clone();
    }

    pub fn add_font(&mut self, path: &str, window: &mut PistonWindow) {
        let font = Rc::new(RefCell::new( 
            window.load_font(path).expect("Failed to load font")
        ));

        let fname = PathBuf::from(path).file_name().unwrap().to_os_string().into_string().unwrap();

        self.fonts.insert(fname, font);
    }

    pub fn get_font(&self, name: &str) -> Font {
        let font = self.fonts.get(&name.to_string());

        // Either return the requested texture OR the default texture if name is invalid
        let font = match font {
            Some (font) => font,
            None => {
                println!("Failed to retrieve Texture"); 
                self.fonts.get(&"default".to_string()).unwrap()
            },
        };
        return font.clone();
    }
}
