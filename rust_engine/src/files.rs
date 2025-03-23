use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// AssetType enum to categorize different asset types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetType {
    Image,
    Sound,
    Video,
    Custom(u32), // Allow for custom asset types with a numeric ID
}

// Result type for asset operations
pub type AssetResult<T> = Result<T, AssetError>;

// Custom error type for asset operations
#[derive(Debug)]
pub enum AssetError {
    IoError(io::Error),
    InvalidAssetType,
    AssetNotFound,
    FormatError(String),
    Other(String),
}

impl From<io::Error> for AssetError {
    fn from(error: io::Error) -> Self {
        AssetError::IoError(error)
    }
}

// Asset struct to represent a loaded asset
#[derive(Debug, Clone)]
pub struct Asset {
    pub asset_type: AssetType,
    pub path: PathBuf,
    pub name: String,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

impl Asset {
    // Create a new asset
    pub fn new<P: AsRef<Path>>(asset_type: AssetType, path: P, name: String, data: Vec<u8>) -> Self {
        Asset {
            asset_type,
            path: path.as_ref().to_path_buf(),
            name,
            data,
            metadata: HashMap::new(),
        }
    }

    // Add metadata to the asset
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    // Create a copy of the asset with a new name
    pub fn create_copy(&self, new_name: &str) -> Self {
        let mut copy = self.clone();
        copy.name = new_name.to_string();
        copy
    }

    // Save the asset to a file
    pub fn save<P: AsRef<Path>>(&self, path: Option<P>) -> AssetResult<()> {
        let save_path = match path {
            Some(p) => p.as_ref().to_path_buf(),
            None => self.path.clone(),
        };
        
        // Create directory if it doesn't exist
        if let Some(parent) = save_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let mut file = File::create(save_path)?;
        file.write_all(&self.data)?;
        Ok(())
    }
}

// AssetManager to handle asset loading, caching, and manipulation
#[derive(Debug, Default)]
pub struct AssetManager {
    pub assets: HashMap<String, Asset>,
    base_paths: HashMap<AssetType, PathBuf>,
}

impl AssetManager {
    // Create a new asset manager
    pub fn new() -> Self {
        let mut manager = Self::default();
        // Setup default base paths
        manager.set_base_path(AssetType::Image, "assets/images");
        manager.set_base_path(AssetType::Sound, "assets/sounds");
        manager.set_base_path(AssetType::Video, "assets/videos");
        manager
    }

    // Set base path for a specific asset type
    pub fn set_base_path<P: AsRef<Path>>(&mut self, asset_type: AssetType, path: P) -> &mut Self {
        self.base_paths.insert(asset_type, path.as_ref().to_path_buf());
        self
    }

    // Get the full path for an asset based on its type
    pub fn get_asset_path<P: AsRef<Path>>(&self, asset_type: AssetType, relative_path: P) -> PathBuf {
        if let Some(base_path) = self.base_paths.get(&asset_type) {
            base_path.join(relative_path)
        } else {
            relative_path.as_ref().to_path_buf()
        }
    }

    // Load an asset from a file
    pub fn load_asset<P: AsRef<Path>>(&mut self, 
                                     asset_type: AssetType, 
                                     path: P, 
                                     name: Option<String>) -> AssetResult<&Asset> {
        let full_path = self.get_asset_path(asset_type, path.as_ref());
        let name = name.unwrap_or_else(|| {
            full_path.file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unnamed_asset".to_string())
        });

        // Check if asset is already loaded
        if self.assets.contains_key(&name) {
            return Ok(&self.assets[&name]);
        }

        // Load the asset data
        let mut file = File::open(&full_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        // Create and store the asset
        let asset = Asset::new(asset_type, full_path, name.clone(), data);
        self.assets.insert(name.clone(), asset);
        
        Ok(&self.assets[&name])
    }

    // Get a previously loaded asset by name
    pub fn get_asset(&self, name: &str) -> Option<&Asset> {
        self.assets.get(name)
    }

    // Get a mutable reference to a previously loaded asset by name
    pub fn get_asset_mut(&mut self, name: &str) -> Option<&mut Asset> {
        self.assets.get_mut(name)
    }

    // Remove an asset from the manager
    pub fn remove_asset(&mut self, name: &str) -> Option<Asset> {
        self.assets.remove(name)
    }

    // Duplicate an asset with a new name
    pub fn duplicate_asset(&mut self, original_name: &str, new_name: &str) -> AssetResult<&Asset> {
        if let Some(original) = self.assets.get(original_name) {
            let duplicate = original.create_copy(new_name);
            self.assets.insert(new_name.to_string(), duplicate);
            Ok(&self.assets[new_name])
        } else {
            Err(AssetError::AssetNotFound)
        }
    }

    // Apply a transformation function to an asset
    pub fn transform_asset<F>(&mut self, name: &str, transform_fn: F) -> AssetResult<&Asset>
    where
        F: FnOnce(&mut Asset),
    {
        if let Some(asset) = self.assets.get_mut(name) {
            transform_fn(asset);
            Ok(&self.assets[name])
        } else {
            Err(AssetError::AssetNotFound)
        }
    }

    // Load all assets of a specific type from a directory
    pub fn load_directory(&mut self, asset_type: AssetType, relative_dir: &str) -> AssetResult<Vec<String>> {
        let full_path = self.get_asset_path(asset_type, relative_dir);
        let mut loaded_asset_names = Vec::new();

        // First, collect all paths to avoid borrowing self in the loop
        let mut file_paths = Vec::new();
        for entry in fs::read_dir(full_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let file_name = path.file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unnamed".to_string());
                    
                file_paths.push((path, file_name));
            }
        }

        // Now load each asset
        for (path, file_name) in file_paths {
            self.load_asset(asset_type, &path, Some(file_name.clone()))?;
            loaded_asset_names.push(file_name);
        }

        Ok(loaded_asset_names)
    }

    // Clear all cached assets
    pub fn clear(&mut self) {
        self.assets.clear();
    }
}

// Functions for common asset transformations that developers can extend

// Helper function to create a copy of an asset with transforms
pub fn transform_copy<'a, F>(
    asset_manager: &'a mut AssetManager, 
    original_name: &str, 
    new_name: &str, 
    transform_fn: F
) -> AssetResult<&'a Asset>
where
    F: FnOnce(&mut Asset),
{
    // First create a copy
    asset_manager.duplicate_asset(original_name, new_name)?;
    
    // Then apply the transformation
    asset_manager.transform_asset(new_name, transform_fn)
}

// Example function for image asset manipulation (placeholder)
pub fn resize_image(_asset: &mut Asset, _width: u32, _height: u32) {
    // This would use an image processing library to resize the image
    // For example with the 'image' crate:
    // let img = image::load_from_memory(&asset.data).unwrap();
    // let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);
    // asset.data = Vec::new();
    // resized.write_to(&mut Cursor::new(&mut asset.data), image::ImageOutputFormat::Png).unwrap();
}

// Function to help with hot-reloading assets during development
pub fn watch_assets<F>(_asset_manager: &AssetManager, _callback: F) -> AssetResult<()>
where
    F: Fn(&str, &AssetType),
{
    // This would use a file watcher library like 'notify' to watch for file changes
    // and reload assets as they change
    // For example:
    // let (tx, rx) = std::sync::mpsc::channel();
    // let mut watcher = notify::recommended_watcher(tx)?;
    // 
    // for (asset_type, path) in &asset_manager.base_paths {
    //     watcher.watch(path, notify::RecursiveMode::Recursive)?;
    // }
    //
    // for res in rx {
    //     match res {
    //         Ok(event) => {
    //             // Determine asset type and notify callback
    //             let path = event.paths[0];
    //             // ...
    //         }
    //         Err(e) => println!("watch error: {:?}", e),
    //     }
    // }
    
    // This is just a placeholder - implementation would depend on actual requirements
    Ok(())
}

// Unit tests for the asset management system
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_asset_creation() {
        let data = vec![1, 2, 3, 4, 5];
        let asset = Asset::new(
            AssetType::Image,
            "test/path.png",
            "test_asset".to_string(),
            data.clone()
        );

        assert_eq!(asset.asset_type, AssetType::Image);
        assert_eq!(asset.name, "test_asset");
        assert_eq!(asset.data, data);
        assert_eq!(asset.metadata.len(), 0);
    }

    #[test]
    fn test_asset_metadata() {
        let data = vec![1, 2, 3];
        let asset = Asset::new(
            AssetType::Sound,
            "test/sound.wav",
            "test_sound".to_string(),
            data
        )
        .with_metadata("duration", "3.5")
        .with_metadata("format", "wav");

        assert_eq!(asset.metadata.len(), 2);
        assert_eq!(asset.metadata.get("duration"), Some(&"3.5".to_string()));
        assert_eq!(asset.metadata.get("format"), Some(&"wav".to_string()));
    }

    #[test]
    fn test_asset_manager_path_resolution() {
        let asset_manager = AssetManager::new();
        
        let image_path = asset_manager.get_asset_path(AssetType::Image, "player.png");
        let sound_path = asset_manager.get_asset_path(AssetType::Sound, "explosion.wav");
        
        assert_eq!(image_path, Path::new("assets/images/player.png"));
        assert_eq!(sound_path, Path::new("assets/sounds/explosion.wav"));
    }

    #[test]
    fn test_asset_duplication() {
        let mut asset_manager = AssetManager::new();
        
        // Add an asset manually
        let test_asset = Asset::new(
            AssetType::Image,
            "test.png",
            "original".to_string(),
            vec![1, 2, 3]
        );
        
        asset_manager.assets.insert("original".to_string(), test_asset);
        
        // Duplicate the asset
        let result = asset_manager.duplicate_asset("original", "copy");
        assert!(result.is_ok());
        
        // Check the copied asset
        let copy = asset_manager.get_asset("copy").expect("Copy should exist");
        assert_eq!(copy.name, "copy");
        assert_eq!(copy.data, vec![1, 2, 3]);
        assert_eq!(copy.asset_type, AssetType::Image);
    }

    #[test]
    fn test_asset_transformation() {
        let mut asset_manager = AssetManager::new();
        
        // Add an asset manually
        let test_asset = Asset::new(
            AssetType::Image,
            "test.png",
            "transform_test".to_string(),
            vec![1, 2, 3]
        );
        
        asset_manager.assets.insert("transform_test".to_string(), test_asset);
        
        // Transform the asset
        let result = asset_manager.transform_asset("transform_test", |asset| {
            asset.data.push(4);
            asset.metadata.insert("transformed".to_string(), "true".to_string());
        });
        
        assert!(result.is_ok());
        
        // Check the transformed asset
        let transformed = asset_manager.get_asset("transform_test").expect("Asset should exist");
        assert_eq!(transformed.data, vec![1, 2, 3, 4]);
        assert_eq!(transformed.metadata.get("transformed"), Some(&"true".to_string()));
    }
}
