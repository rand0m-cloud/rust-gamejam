use bevy::{prelude::*, utils::HashMap};

use heron::{CollisionData, CollisionEvent};

/// Component which will be filled (if present) with a list of entities with which the current entity is currently in contact.
#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct Collisions(HashMap<Entity, CollisionData>);

impl Collisions {
    /// Returns the number of colliding entities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there is no colliding entities.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the collisions contains the specified entity.
    #[must_use]
    pub fn contains(&self, entity: &Entity) -> bool {
        self.0.contains_key(entity)
    }

    /// An iterator visiting all colliding entities in arbitrary order.
    #[deprecated(note = "Please use `entities()` instead")]
    #[doc(hidden)]
    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities()
    }

    /// An iterator visiting all colliding entities in arbitrary order.
    pub fn entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.0.keys().copied()
    }

    /// An iterator visiting all data from colliding entities in arbitrary order.
    pub fn collision_data(&self) -> impl Iterator<Item = &CollisionData> + '_ {
        self.0.values()
    }
}

/// Adds entity to CollidingEntities on starting collision and removes from it when the
/// collision end.
pub fn update_collisions_system(
    mut collision_events: EventReader<'_, '_, CollisionEvent>,
    mut collisions: Query<'_, '_, &mut Collisions>,
) {
    for event in collision_events.iter() {
        let (data1, data2) = event.clone().data();
        let (entity1, entity2) = (data1.rigid_body_entity(), data2.rigid_body_entity());
        if event.is_started() {
            if let Ok(mut entities) = collisions.get_mut(entity1) {
                entities.0.insert(entity2, data2);
            }
            if let Ok(mut entities) = collisions.get_mut(entity2) {
                entities.0.insert(entity1, data1);
            }
        } else {
            if let Ok(mut entities) = collisions.get_mut(entity1) {
                entities.0.remove(&entity2);
            }
            if let Ok(mut entities) = collisions.get_mut(entity2) {
                entities.0.remove(&entity1);
            }
        }
    }
}
