#[macro_export]
macro_rules! get {
    ($query:expr, $entity:expr) => {
        match $query.get($entity) {
            Ok(data) => data,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_mut {
    ($query:expr, $entity:expr) => {
        match $query.get_mut($entity) {
            Ok(data) => data,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_or_return {
    ($query:expr, $entity:expr) => {
        match $query.get($entity) {
            Ok(data) => data,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_mut_or_return {
    ($query:expr, $entity:expr) => {
        match $query.get_mut($entity) {
            Ok(data) => data,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_or_continue {
    ($query:expr, $entity:expr) => {
        match $query.get($entity) {
            Ok(data) => data,
            _ => continue,
        }
    };
}

#[macro_export]
macro_rules! get_mut_or_continue {
    ($query:expr, $entity:expr) => {
        match $query.get_mut($entity) {
            Ok(data) => data,
            _ => continue,
        }
    };
}

#[macro_export]
macro_rules! get_single {
    ($query:expr) => {
        match $query.get_single() {
            Ok(data) => data,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_single_mut {
    ($query:expr) => {
        match $query.get_single_mut() {
            Ok(data) => data,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_single_or_return {
    ($query:expr) => {
        match $query.get_single() {
            Ok(data) => data,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_single_mut_or_return {
    ($query:expr) => {
        match $query.get_single_mut() {
            Ok(data) => data,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_single_or_continue {
    ($query:expr) => {
        match $query.get_single() {
            Ok(data) => data,
            _ => continue,
        }
    };
}

#[macro_export]
macro_rules! get_single_mut_or_continue {
    ($query:expr) => {
        match $query.get_single_mut() {
            Ok(data) => data,
            _ => continue,
        }
    };
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use bevy_ecs::prelude::*;

    use super::*;

    #[derive(Component)]
    struct TestComponent;

    #[test]
    fn test_get_on_empty_query() {
        let mut world = World::new();

        let system = world.register_system(|query: Query<(Entity, &TestComponent)>| {
            let _ = get!(query, Entity::from_raw(0));
            unreachable!("should not be called");
        });

        world.run_system(system).unwrap();
    }

    #[test]
    fn test_get_on_single_query() {
        let mut world = World::new();

        let entity = world.spawn(TestComponent).id();

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |query: Query<(Entity, &TestComponent)>| {
            let _ = get!(query, entity);
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_mut_on_empty_query() {
        let mut world = World::new();

        let system = world.register_system(|mut query: Query<(Entity, &mut TestComponent)>| {
            let _ = get_mut!(query, Entity::from_raw(0));
            unreachable!("should not be called");
        });

        world.run_system(system).unwrap();
    }

    #[test]
    fn test_get_mut_on_single_query() {
        let mut world = World::new();

        let entity = world.spawn(TestComponent).id();

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system =
            world.register_system(move |mut query: Query<(Entity, &mut TestComponent)>| {
                let _ = get_mut!(query, entity);
                *system_ref.lock().unwrap() = false;
            });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_or_return_on_empty_query() {
        let mut world = World::new();

        let system = world.register_system(|query: Query<(Entity, &TestComponent)>| {
            let _ = get_or_return!(query, Entity::from_raw(0));
            unreachable!("should not be called");
        });

        world.run_system(system).unwrap();
    }

    #[test]
    fn test_get_or_return_on_single_query() {
        let mut world = World::new();

        let entity = world.spawn(TestComponent).id();

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |query: Query<(Entity, &TestComponent)>| {
            let _ = get_or_return!(query, entity);
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_mut_or_return_on_empty_query() {
        let mut world = World::new();

        let system = world.register_system(|mut query: Query<(Entity, &TestComponent)>| {
            let _ = get_mut_or_return!(query, Entity::from_raw(0));
            unreachable!("should not be called");
        });

        world.run_system(system).unwrap();
    }

    #[test]
    fn test_get_mut_or_return_on_single_query() {
        let mut world = World::new();

        let entity = world.spawn(TestComponent).id();

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |mut query: Query<(Entity, &TestComponent)>| {
            let _ = get_mut_or_return!(query, entity);
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_or_continue_on_empty_query() {
        let mut world = World::new();

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |query: Query<(Entity, &TestComponent)>| {
            for _ in 0..10 {
                let _ = get_or_continue!(query, Entity::from_raw(0));
                unreachable!("should not be called");
            }
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_or_continue_on_single_query() {
        let mut world = World::new();

        let entity = world.spawn(TestComponent).id();

        let early_continue = Arc::new(Mutex::new(true));
        let system_ref = early_continue.clone();
        let system = world.register_system(move |query: Query<(Entity, &TestComponent)>| {
            for _ in 0..10 {
                let _ = get_or_continue!(query, entity);
                *system_ref.lock().unwrap() = false;
            }
        });

        world.run_system(system).unwrap();

        assert!(!*early_continue.lock().unwrap());
    }

    #[test]
    fn test_get_mut_or_continue_on_empty_query() {
        let mut world = World::new();

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |mut query: Query<(Entity, &TestComponent)>| {
            for _ in 0..10 {
                let _ = get_mut_or_continue!(query, Entity::from_raw(0));
                unreachable!("should not be called");
            }
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_mut_or_continue_on_single_query() {
        let mut world = World::new();

        let entity = world.spawn(TestComponent).id();

        let early_continue = Arc::new(Mutex::new(true));
        let system_ref = early_continue.clone();
        let system = world.register_system(move |mut query: Query<(Entity, &TestComponent)>| {
            for _ in 0..10 {
                let _ = get_mut_or_continue!(query, entity);
                *system_ref.lock().unwrap() = false;
            }
        });

        world.run_system(system).unwrap();

        assert!(!*early_continue.lock().unwrap());
    }

    #[test]
    fn test_get_single_on_empty_query() {
        let mut world = World::new();

        let system = world.register_system(|query: Query<(Entity, &TestComponent)>| {
            let _ = get_single!(query);
            unreachable!("should not be called");
        });

        world.run_system(system).unwrap();
    }

    #[test]
    fn test_get_single_on_single_query() {
        let mut world = World::new();

        world.spawn(TestComponent);

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |query: Query<(Entity, &TestComponent)>| {
            let _ = get_single!(query);
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_single_mut_on_empty_query() {
        let mut world = World::new();

        let system = world.register_system(|mut query: Query<(Entity, &TestComponent)>| {
            let _ = get_single_mut!(query);
            unreachable!("should not be called");
        });

        world.run_system(system).unwrap();
    }

    #[test]
    fn test_get_single_mut_on_single_query() {
        let mut world = World::new();

        world.spawn(TestComponent);

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |mut query: Query<(Entity, &TestComponent)>| {
            let _ = get_single_mut!(query);
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_single_or_return_on_empty_query() {
        let mut world = World::new();

        let system = world.register_system(|query: Query<(Entity, &TestComponent)>| {
            let _ = get_single_or_return!(query);
            unreachable!("should not be called");
        });

        world.run_system(system).unwrap();
    }

    #[test]
    fn test_get_single_or_return_on_single_query() {
        let mut world = World::new();

        world.spawn(TestComponent);

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |query: Query<(Entity, &TestComponent)>| {
            let _ = get_single_or_return!(query);
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_single_mut_or_return_on_empty_query() {
        let mut world = World::new();

        let system = world.register_system(|mut query: Query<(Entity, &TestComponent)>| {
            let _ = get_single_mut_or_return!(query);
            unreachable!("should not be called");
        });

        world.run_system(system).unwrap();
    }

    #[test]
    fn test_get_single_mut_or_return_on_single_query() {
        let mut world = World::new();

        world.spawn(TestComponent);

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |mut query: Query<(Entity, &TestComponent)>| {
            let _ = get_single_mut_or_return!(query);
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_single_or_continue_on_empty_query() {
        let mut world = World::new();

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |query: Query<(Entity, &TestComponent)>| {
            for _ in 0..10 {
                let _ = get_single_or_continue!(query);
                unreachable!("should not be called");
            }
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_single_or_continue_on_single_query() {
        let mut world = World::new();

        world.spawn(TestComponent);

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |query: Query<(Entity, &TestComponent)>| {
            for _ in 0..10 {
                let _ = get_single_or_continue!(query);
                *system_ref.lock().unwrap() = false;
            }
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_single_mut_or_continue_on_empty_query() {
        let mut world = World::new();

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |mut query: Query<(Entity, &TestComponent)>| {
            for _ in 0..10 {
                let _ = get_single_mut_or_continue!(query);
                unreachable!("should not be called");
            }
            *system_ref.lock().unwrap() = false;
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }

    #[test]
    fn test_get_single_mut_or_continue_on_single_query() {
        let mut world = World::new();

        world.spawn(TestComponent);

        let early_return = Arc::new(Mutex::new(true));
        let system_ref = early_return.clone();
        let system = world.register_system(move |mut query: Query<(Entity, &TestComponent)>| {
            for _ in 0..10 {
                let _ = get_single_mut_or_continue!(query);
                *system_ref.lock().unwrap() = false;
            }
        });

        world.run_system(system).unwrap();

        assert!(!*early_return.lock().unwrap());
    }
}
