extern crate sfml;

use sfml::graphics::Texture;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Recourses<K>
{
    textures: HashMap<K, Texture>,
}


#[allow(dead_code)]
impl<K> Recourses<K>
where K: Hash + Eq,
{
    pub fn new() -> Recourses<K>
    {
        Recourses {
            textures: HashMap::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<&Texture>  
    {
       self.textures.get(key) 
    }

    pub fn add(&mut self, key: K, value: Texture)
    {
        self.textures.insert(key, value);
    }

    pub fn add_from_file(&mut self, path: &str, name: K)
    {
        if let Some(text) = Texture::from_file(path)
        {
            self.add(name, text);
        }
    }
}
