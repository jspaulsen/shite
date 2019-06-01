use std::collections::hash_map::{
    Iter,
    IterMut,
};
use std::collections::HashMap;

use nphysics2d::object::BodyHandle;
use uuid::Uuid;

use crate::trackable::Trackable;

// TODO(#24): [GameObjectStore] HashMap is bottleneck
// Optimize by using Hashbrown or comparable
#[derive(Default)]
pub struct GameObjectStore<T> {
    objects: HashMap<Uuid, T>,
}

#[derive(Default)]
pub struct PhysicsObjectMap<T> {
    objects: HashMap<BodyHandle, T>,
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

impl<T> PhysicsObjectMap<T> {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add(&mut self, handle: BodyHandle, object: T) -> bool {
        if self.objects.contains_key(&handle) {
            return false;
        }

        self.objects.insert(handle, object);
        true
    }

    pub fn remove(&mut self, handle: BodyHandle) -> Option<T> {
        self.objects.remove(&handle)
    }

    pub fn get_object(&self, handle: BodyHandle) -> Option<&T> {
        self.objects.get(&handle)
    }

    pub fn get_object_mut(&mut self, handle: BodyHandle) -> Option<&mut T> {
        self.objects.get_mut(&handle)
    }

    pub fn get_objects(&self) -> Iter<'_, BodyHandle, T> {
        self.objects.iter()
    }

    pub fn get_objects_mut(&mut self) -> IterMut<'_, BodyHandle, T> {
        self.objects.iter_mut()
    }
}
