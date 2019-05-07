use std::collections::HashMap;
use std::collections::hash_map::{Iter, IterMut};

use uuid::Uuid;

use crate::trackable::Trackable;


// TODO: [GameObjectStore] HashMap is bottleneck
// Optimize by using Hashbrown or comparable
#[derive(Default)]
pub struct GameObjectStore<T> {
    objects: HashMap<Uuid, T>,
}


impl<T: Trackable> GameObjectStore<T> {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add(&mut self, object: T) -> bool {
        if self.objects.contains_key(object.get_handle()) {
            return false;
        }

        self.objects.insert(*object.get_handle(), object);
        true
    }

    pub fn remove(&mut self, handle: &Uuid) -> Option<T> {
        self.objects.remove(handle)
    }

    pub fn get_object(&self, handle: &Uuid) -> Option<&T> {
        self.objects.get(handle)
    }

    pub fn get_object_mut(&mut self, handle: &Uuid) -> Option<&mut T> {
        self.objects.get_mut(handle)
    }

    pub fn get_objects(&self) -> Iter<'_, Uuid, T> {
        self.objects.iter()
    }

    pub fn get_objects_mut(&mut self) -> IterMut<'_, Uuid, T> {
        self.objects.iter_mut()
    }
}
