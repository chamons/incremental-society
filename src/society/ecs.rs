use specs::prelude::*;

pub trait EasyEcs<T: Component> {
    fn grab(&self, entity: Entity) -> &T;
    fn has(&self, entity: Entity) -> bool;
    fn has_no(&self, entity: Entity) -> bool;
}

impl<'a, T: Component> EasyEcs<T> for ReadStorage<'a, T> {
    fn grab(&self, entity: Entity) -> &T {
        self.get(entity).unwrap()
    }
    fn has(&self, entity: Entity) -> bool {
        self.get(entity).is_some()
    }
    fn has_no(&self, entity: Entity) -> bool {
        self.get(entity).is_none()
    }
}

impl<'a, T: Component> EasyEcs<T> for WriteStorage<'a, T> {
    fn grab(&self, entity: Entity) -> &T {
        self.get(entity).unwrap()
    }
    fn has(&self, entity: Entity) -> bool {
        self.get(entity).is_some()
    }
    fn has_no(&self, entity: Entity) -> bool {
        self.get(entity).is_none()
    }
}

pub trait EasyMutEcs<T: Component> {
    fn grab_mut(&mut self, entity: Entity) -> &mut T;
}

impl<'a, T: Component> EasyMutEcs<T> for WriteStorage<'a, T> {
    fn grab_mut(&mut self, entity: Entity) -> &mut T {
        self.get_mut(entity).unwrap()
    }
}

pub trait EasyMutWorld<T: Component> {
    fn shovel(&mut self, entity: Entity, item: T);
}

impl<T: Component> EasyMutWorld<T> for World {
    fn shovel(&mut self, entity: Entity, item: T) {
        self.write_storage::<T>().insert(entity, item).unwrap();
    }
}
