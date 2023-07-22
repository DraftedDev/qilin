use std::fs;
use std::path::PathBuf;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerPrefs {
    path: PathBuf,
    data: HashMap<String, Storable>,
}

impl PlayerPrefs {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            data: HashMap::new(),
        }
    }

    pub fn save(&mut self) -> Option<()> {
        if self.path.exists() {
            fs::write(&self.path, serde_json::to_string(&self.data).unwrap()).ok();
        } else {
            fs::File::create(&self.path).ok()?;
            fs::write(&self.path, serde_json::to_string(&self.data).unwrap()).ok();
        }

        self.reload()?;
        Some(())
    }

    pub fn get(&self, key: String) -> Option<&Storable> {
        self.data.get(&key)
    }

    pub fn insert(&mut self, key: String, value: Storable) -> Option<()> {
        self.data.insert(key, value);
        Some(())
    }

    pub fn contains(&self, key: String) -> bool {
        self.data.contains_key(&key)
    }

    pub fn remove(&mut self, key: String) -> Option<()> {
        self.data.remove(&key)?;
        Some(())
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn reload(&mut self) -> Option<()> {
        self.data = serde_json::from_str(
            fs::read_to_string(&self.path).ok()?.as_str()
        ).ok()?;
        Some(())
    }

}

#[derive(Serialize, Deserialize)]
pub enum Storable {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),

    Vec(Vec<Storable>),

    StringVec(Vec<String>),
    IntVec(Vec<i32>),
    FloatVec(Vec<f32>),
    BoolVec(Vec<bool>),
}

impl Storable {
    pub fn as_string(&self) -> Option<&String> {
        if let Storable::String(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<&i32> {
        if let Storable::Int(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<&f32> {
        if let Storable::Float(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<&bool> {
        if let Storable::Bool(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_vec(&self) -> Option<&Vec<Storable>> {
        if let Storable::Vec(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_string_vec(&self) -> Option<&Vec<String>> {
        if let Storable::StringVec(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_int_vec(&self) -> Option<&Vec<i32>> {
        if let Storable::IntVec(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_float_vec(&self) -> Option<&Vec<f32>> {
        if let Storable::FloatVec(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_bool_vec(&self) -> Option<&Vec<bool>> {
        if let Storable::BoolVec(val) = self {
            Some(val)
        } else {
            None
        }
    }
}