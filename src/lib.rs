#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![allow(clippy::type_complexity)]

pub mod app;
pub mod component_container;
pub mod entities;
pub mod query;
pub mod query_parameters;
pub mod resource;
pub mod system;
pub mod system_parameters;
pub mod system_set;

#[cfg(test)]
mod tests {
    use crate::{
        app::App,
        entities::Entities,
        query::{Component, Query, Ref, RefMut},
        system_set::SystemSet,
    };

    #[test]
    fn test() {
        struct TestComponent {
            value: i32,
        }
        impl Component for TestComponent {}

        struct TestComponent2 {
            value: i32,
        }
        impl Component for TestComponent2 {}

        let mut app = App::new();

        let entity1 = app.create_entity();
        app.add_component(entity1, TestComponent { value: 42 });

        let entity2 = app.create_entity();
        app.add_component(entity2, TestComponent { value: 44 });
        app.add_component(entity2, TestComponent2 { value: 0 });

        app.run(|mut q: Query<'_, RefMut<TestComponent>>| {
            let [c1, c2] = q.get_many_mut([entity1, entity2]).unwrap();
            assert_eq!(c1.value, 42);
            assert_eq!(c2.value, 44);
            c1.value += 1;
            c2.value -= 1;
        });

        let mut set = SystemSet::new();
        set.register_system(
            |entities: Entities<'_>,
             q: Query<
                '_,
                (
                    Ref<TestComponent>,
                    Option<(Ref<TestComponent2>, Ref<TestComponent>)>,
                ),
            >| {
                for entity in entities.iter() {
                    let (c, c2) = q.get(entity).unwrap();
                    assert_eq!(c.value, 43);
                    if let Some((c2, c)) = c2 {
                        assert_eq!(c2.value, 0);
                        assert_eq!(c.value, 43);
                    }
                }
            },
        );
        app.run(&mut set);
    }
}
