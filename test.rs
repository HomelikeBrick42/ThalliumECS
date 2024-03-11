#![feature(prelude_import)]
#![forbid(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod app {
    use crate::{
        entities::Entity, query::Component, resource::Resource,
        system::{
            ComponentContainer, ComponentMap, ResourceMap, RunState, System,
            SystemWrapper,
        },
    };
    use hashbrown::HashMap;
    use parking_lot::RwLock;
    use slotmap::SlotMap;
    use std::{any::TypeId, marker::PhantomData};
    pub struct App {
        resources: ResourceMap,
        entities: SlotMap<Entity, ()>,
        components: ComponentMap,
        systems: Vec<Box<dyn System>>,
    }
    impl App {
        pub fn new() -> Self {
            Self {
                resources: HashMap::new(),
                entities: SlotMap::with_key(),
                components: HashMap::new(),
                systems: Vec::new(),
            }
        }
        pub fn add_resource<R>(&mut self, resource: R)
        where
            R: Resource,
        {
            self.resources.insert(TypeId::of::<R>(), RwLock::new(Box::new(resource)));
        }
        pub fn remove_resource<R>(&mut self) -> Option<R>
        where
            R: Resource,
        {
            self.resources
                .remove(&TypeId::of::<R>())
                .map(|resource| *resource.into_inner().downcast::<R>().unwrap())
        }
        pub fn create_entity(&mut self) -> Entity {
            self.entities.insert(())
        }
        pub fn destroy_entity(&mut self, entity: Entity) {
            self.entities.remove(entity);
        }
        pub fn add_component<C>(&mut self, entity: Entity, component: C)
        where
            C: Component,
        {
            if !self.entities.contains_key(entity) {
                return;
            }
            self.components
                .entry(TypeId::of::<C>())
                .or_insert_with(|| RwLock::new(Box::new(ComponentContainer::<C>::new())))
                .get_mut()
                .downcast_mut::<ComponentContainer<C>>()
                .unwrap()
                .insert(entity, component);
        }
        pub fn remove_component<C>(&mut self, entity: Entity) -> Option<C>
        where
            C: Component,
        {
            if !self.entities.contains_key(entity) {
                return None;
            }
            self.components
                .get_mut(&TypeId::of::<C>())?
                .get_mut()
                .downcast_mut::<ComponentContainer<C>>()
                .unwrap()
                .remove(entity)
        }
        pub fn register_system<S, Marker>(&mut self, system: S)
        where
            SystemWrapper<S, Marker>: System,
        {
            self.systems.push(Box::new(SystemWrapper(system, PhantomData)));
        }
        pub fn run_once<S, Marker>(&mut self, system: S)
        where
            SystemWrapper<S, Marker>: System,
        {
            SystemWrapper(system, PhantomData)
                .run(RunState {
                    resources: &self.resources,
                    entities: &self.entities,
                    components: &self.components,
                });
        }
        pub fn run_registered(&mut self) {
            for system in &mut self.systems {
                system
                    .run(RunState {
                        resources: &self.resources,
                        entities: &self.entities,
                        components: &self.components,
                    });
            }
        }
    }
    impl Default for App {
        fn default() -> Self {
            Self::new()
        }
    }
}
pub mod entities {
    use std::any::TypeId;
    use crate::{
        system::{BorrowType, RunState},
        system_parameters::SystemParameter,
    };
    use slotmap::SlotMap;
    #[repr(transparent)]
    pub struct Entity(::slotmap::KeyData);
    #[automatically_derived]
    impl ::core::marker::Copy for Entity {}
    #[automatically_derived]
    impl ::core::clone::Clone for Entity {
        #[inline]
        fn clone(&self) -> Entity {
            let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Entity {
        #[inline]
        fn default() -> Entity {
            Entity(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Entity {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Entity {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Entity {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Entity {
        #[inline]
        fn eq(&self, other: &Entity) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Entity {
        #[inline]
        fn cmp(&self, other: &Entity) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Entity {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Entity,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Entity {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Entity {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Entity", &&self.0)
        }
    }
    impl ::slotmap::__impl::From<::slotmap::KeyData> for Entity {
        fn from(k: ::slotmap::KeyData) -> Self {
            Entity(k)
        }
    }
    unsafe impl ::slotmap::Key for Entity {
        fn data(&self) -> ::slotmap::KeyData {
            self.0
        }
    }
    pub struct Entities<'a> {
        entities: &'a SlotMap<Entity, ()>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for Entities<'a> {
        #[inline]
        fn clone(&self) -> Entities<'a> {
            let _: ::core::clone::AssertParamIsClone<&'a SlotMap<Entity, ()>>;
            *self
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::Copy for Entities<'a> {}
    impl<'a> Entities<'a> {
        pub fn iter(&self) -> impl ExactSizeIterator<Item = Entity> + 'a {
            self.entities.keys()
        }
    }
    impl<'a> SystemParameter for Entities<'a> {
        type This<'this> = Entities<'this>;
        type Lock<'state> = &'state SlotMap<Entity, ()>;
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            state.entities
        }
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            Entities { entities: state }
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
    }
}
pub mod query {
    use crate::{
        entities::Entity, query_parameters::QueryParameter,
        system::{BorrowType, RunState},
        system_parameters::SystemParameter,
    };
    use slotmap::{SecondaryMap, SlotMap};
    use std::{any::TypeId, marker::PhantomData};
    pub trait Component: Sized + Send + Sync + 'static {}
    pub struct Ref<C>(
        PhantomData<fn() -> C>,
    )
    where
        C: Component;
    pub struct RefMut<C>(
        PhantomData<fn() -> C>,
    )
    where
        C: Component;
    pub struct Query<'a, Q>
    where
        Q: QueryParameter,
    {
        entities: &'a SlotMap<Entity, ()>,
        container: Q::ComponentContainer<'a>,
    }
    impl<'a, Q> SystemParameter for Query<'a, Q>
    where
        Q: QueryParameter,
    {
        type This<'this> = Query<'this, Q>;
        type Lock<'state> = (
            &'state SlotMap<Entity, ()>,
            Q::ComponentContainerLock<'state>,
        );
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            (state.entities, Q::lock(state))
        }
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            let (entities, state) = state;
            Query {
                entities,
                container: Q::construct(state),
            }
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            Q::get_component_types()
        }
    }
    impl<'a, Q> Query<'a, Q>
    where
        Q: QueryParameter,
    {
        pub fn get<'b>(
            &'b self,
            entity: Entity,
        ) -> Option<
            <Q::ComponentContainer<'a> as ComponentContainerTrait<'a>>::Parameter<'b>,
        > {
            if self.entities.contains_key(entity) {
                self.container.get(entity)
            } else {
                None
            }
        }
        pub fn get_mut<'b>(
            &'b mut self,
            entity: Entity,
        ) -> Option<
            <Q::ComponentContainer<'a> as ComponentContainerTrait<'a>>::ParameterMut<'b>,
        > {
            if self.entities.contains_key(entity) {
                self.container.get_mut(entity)
            } else {
                None
            }
        }
        pub fn get_many_mut<'b, const N: usize>(
            &'b mut self,
            entities: [Entity; N],
        ) -> Option<
            [<Q::ComponentContainer<
                'a,
            > as ComponentContainerTrait<'a>>::ParameterMut<'b>; N],
        > {
            if entities.iter().all(|&entity| self.entities.contains_key(entity)) {
                self.container.get_many_mut(entities)
            } else {
                None
            }
        }
    }
    pub trait ComponentContainerTrait<'a>: Send + Sync {
        type Parameter<'param> where Self: 'param;
        type ParameterMut<'param> where Self: 'param;
        fn get(&self, entity: Entity) -> Option<Self::Parameter<'_>>;
        fn get_mut(&mut self, entity: Entity) -> Option<Self::ParameterMut<'_>>;
        fn get_many_mut<const N: usize>(
            &mut self,
            entities: [Entity; N],
        ) -> Option<[Self::ParameterMut<'_>; N]>;
    }
    impl<'a, C> ComponentContainerTrait<'a> for Option<&'a SecondaryMap<Entity, C>>
    where
        C: Component,
    {
        type Parameter<'param> = &'param C where Self: 'param;
        type ParameterMut<'param> = &'param C where Self: 'param;
        fn get(&self, entity: Entity) -> Option<Self::Parameter<'_>> {
            SecondaryMap::get(self.as_ref()?, entity)
        }
        fn get_mut(&mut self, entity: Entity) -> Option<Self::ParameterMut<'_>> {
            SecondaryMap::get(self.as_mut()?, entity)
        }
        fn get_many_mut<const N: usize>(
            &mut self,
            entities: [Entity; N],
        ) -> Option<[Self::ParameterMut<'_>; N]> {
            let container = self.as_mut()?;
            if entities.iter().all(|&entity| container.contains_key(entity)) {
                Some(entities.map(|entity| container.get(entity).unwrap()))
            } else {
                None
            }
        }
    }
    impl<'a, C> ComponentContainerTrait<'a> for Option<&'a mut SecondaryMap<Entity, C>>
    where
        C: Component,
    {
        type Parameter<'param> = &'param C where Self: 'param;
        type ParameterMut<'param> = &'param mut C where Self: 'param;
        fn get(&self, entity: Entity) -> Option<Self::Parameter<'_>> {
            SecondaryMap::get(self.as_ref()?, entity)
        }
        fn get_mut(&mut self, entity: Entity) -> Option<Self::ParameterMut<'_>> {
            SecondaryMap::get_mut(self.as_mut()?, entity)
        }
        fn get_many_mut<const N: usize>(
            &mut self,
            entities: [Entity; N],
        ) -> Option<[Self::ParameterMut<'_>; N]> {
            SecondaryMap::get_disjoint_mut(self.as_mut()?, entities)
        }
    }
    impl<'a> ComponentContainerTrait<'a> for () {
        type Parameter<'param> = () where Self: 'param;
        type ParameterMut<'param> = () where Self: 'param;
        fn get(&self, entity: Entity) -> Option<Self::Parameter<'_>> {
            let () = self;
            Some(())
        }
        fn get_mut(&mut self, entity: Entity) -> Option<Self::ParameterMut<'_>> {
            let () = self;
            Some(())
        }
        fn get_many_mut<const Len: usize>(
            &mut self,
            entities: [Entity; Len],
        ) -> Option<[Self::ParameterMut<'_>; Len]> {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<'a, A> ComponentContainerTrait<'a> for (A,)
    where
        A: ComponentContainerTrait<'a>,
    {
        type Parameter<'param> = (A::ParameterMut<'param>,) where Self: 'param;
        type ParameterMut<'param> = (A::ParameterMut<'param>,) where Self: 'param;
        fn get(&self, entity: Entity) -> Option<Self::Parameter<'_>> {
            let (A,) = self;
            Some((A.get(entity)?,))
        }
        fn get_mut(&mut self, entity: Entity) -> Option<Self::ParameterMut<'_>> {
            let (A,) = self;
            Some((A.get_mut(entity)?,))
        }
        fn get_many_mut<const Len: usize>(
            &mut self,
            entities: [Entity; Len],
        ) -> Option<[Self::ParameterMut<'_>; Len]> {
            ::core::panicking::panic("not yet implemented")
        }
    }
}
pub mod query_parameters {
    use std::any::TypeId;
    use parking_lot::{
        MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLockReadGuard, RwLockWriteGuard,
    };
    use crate::{
        query::{Component, ComponentContainerTrait, Ref, RefMut},
        system::{BorrowType, ComponentContainer, RunState},
    };
    pub trait QueryParameter {
        type ComponentContainerLock<'a>;
        type ComponentContainer<'a>: ComponentContainerTrait<'a>;
        fn lock(state: RunState<'_>) -> Self::ComponentContainerLock<'_>;
        fn construct<'a>(
            lock: &'a mut Self::ComponentContainerLock<'_>,
        ) -> Self::ComponentContainer<'a>;
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)>;
    }
    impl<C> QueryParameter for Ref<C>
    where
        C: Component,
    {
        type ComponentContainerLock<'a> = Option<
            MappedRwLockReadGuard<'a, ComponentContainer<C>>,
        >;
        type ComponentContainer<'a> = Option<&'a ComponentContainer<C>>;
        fn lock(state: RunState<'_>) -> Self::ComponentContainerLock<'_> {
            Some(
                RwLockReadGuard::map(
                    state
                        .components
                        .get(&TypeId::of::<C>())?
                        .try_read()
                        .expect("the lock should always be available"),
                    |components| {
                        components.downcast_ref::<ComponentContainer<C>>().unwrap()
                    },
                ),
            )
        }
        fn construct<'a>(
            lock: &'a mut Self::ComponentContainerLock<'_>,
        ) -> Self::ComponentContainer<'a> {
            Some(lock.as_mut()?)
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::once((TypeId::of::<C>(), BorrowType::Immutable))
        }
    }
    impl<C> QueryParameter for RefMut<C>
    where
        C: Component,
    {
        type ComponentContainerLock<'a> = Option<
            MappedRwLockWriteGuard<'a, ComponentContainer<C>>,
        >;
        type ComponentContainer<'a> = Option<&'a mut ComponentContainer<C>>;
        fn lock(state: RunState<'_>) -> Self::ComponentContainerLock<'_> {
            Some(
                RwLockWriteGuard::map(
                    state
                        .components
                        .get(&TypeId::of::<C>())?
                        .try_write()
                        .expect("the lock should always be available"),
                    |components| {
                        components.downcast_mut::<ComponentContainer<C>>().unwrap()
                    },
                ),
            )
        }
        fn construct<'a>(
            lock: &'a mut Self::ComponentContainerLock<'_>,
        ) -> Self::ComponentContainer<'a> {
            Some(lock.as_mut()?)
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::once((TypeId::of::<C>(), BorrowType::Mutable))
        }
    }
}
pub mod resource {
    use std::ops::{Deref, DerefMut};
    pub trait Resource: Sized + Send + Sync + 'static {}
    pub struct Res<'a, T>
    where
        T: Resource,
    {
        pub(crate) inner: &'a T,
    }
    impl<'a, T> Deref for Res<'a, T>
    where
        T: Resource,
    {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            self.inner
        }
    }
    pub struct ResMut<'a, T>
    where
        T: Resource,
    {
        pub(crate) inner: &'a mut T,
    }
    impl<'a, T> Deref for ResMut<'a, T>
    where
        T: Resource,
    {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            self.inner
        }
    }
    impl<'a, T> DerefMut for ResMut<'a, T>
    where
        T: Resource,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.inner
        }
    }
}
pub mod system {
    use crate::{entities::Entity, system_parameters::SystemParameter};
    use hashbrown::HashMap;
    use parking_lot::RwLock;
    use slotmap::{SecondaryMap, SlotMap};
    use std::{
        any::{Any, TypeId},
        marker::PhantomData,
    };
    pub(crate) type ResourceMap = HashMap<TypeId, RwLock<Box<dyn Any + Send + Sync>>>;
    pub(crate) type ComponentMap = HashMap<TypeId, RwLock<Box<dyn Any + Send + Sync>>>;
    pub(crate) type ComponentContainer<T> = SecondaryMap<Entity, T>;
    pub struct RunState<'a> {
        pub(crate) resources: &'a ResourceMap,
        pub(crate) entities: &'a SlotMap<Entity, ()>,
        pub(crate) components: &'a ComponentMap,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for RunState<'a> {
        #[inline]
        fn clone(&self) -> RunState<'a> {
            let _: ::core::clone::AssertParamIsClone<&'a ResourceMap>;
            let _: ::core::clone::AssertParamIsClone<&'a SlotMap<Entity, ()>>;
            let _: ::core::clone::AssertParamIsClone<&'a ComponentMap>;
            *self
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::Copy for RunState<'a> {}
    pub enum BorrowType {
        Immutable,
        Mutable,
    }
    pub trait System: Send + Sync + 'static {
        fn run(&mut self, state: RunState<'_>);
        fn get_resource_types(&self) -> Vec<(TypeId, BorrowType)>;
        fn get_component_types(&self) -> Vec<(TypeId, BorrowType)>;
    }
    pub struct SystemWrapper<F, Marker>(
        pub(crate) F,
        pub(crate) PhantomData<fn(Marker)>,
    );
    impl<F, Marker> System for SystemWrapper<F, Marker>
    where
        Marker: 'static,
        F: SystemFunction<Marker>,
    {
        fn run(&mut self, state: RunState<'_>) {
            SystemFunction::run(&mut self.0, state);
        }
        fn get_resource_types(&self) -> Vec<(TypeId, BorrowType)> {
            F::get_resource_types().collect()
        }
        fn get_component_types(&self) -> Vec<(TypeId, BorrowType)> {
            F::get_component_types().collect()
        }
    }
    pub trait SystemFunction<Marker>: Send + Sync + 'static {
        fn run(&mut self, state: RunState<'_>);
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)>;
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)>;
    }
    impl<Func> SystemFunction<fn()> for Func
    where
        for<'a> Func: FnMut() + FnMut() + Send + Sync + 'static,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            self()
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
    }
    impl<Func, A> SystemFunction<fn(A)> for Func
    where
        for<'a> Func: FnMut(A) + FnMut(A::This<'a>) + Send + Sync + 'static,
        A: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            self(A::construct(&mut A))
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty().chain(A::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty().chain(A::get_component_types())
        }
    }
    impl<Func, A, B> SystemFunction<fn(A, B)> for Func
    where
        for<'a> Func: FnMut(A, B) + FnMut(A::This<'a>, B::This<'a>) + Send + Sync
            + 'static,
        A: SystemParameter,
        B: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            self(A::construct(&mut A), B::construct(&mut B))
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
        }
    }
    impl<Func, A, B, C> SystemFunction<fn(A, B, C)> for Func
    where
        for<'a> Func: FnMut(A, B, C) + FnMut(A::This<'a>, B::This<'a>, C::This<'a>)
            + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            self(A::construct(&mut A), B::construct(&mut B), C::construct(&mut C))
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
        }
    }
    impl<Func, A, B, C, D> SystemFunction<fn(A, B, C, D)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D)
            + FnMut(A::This<'a>, B::This<'a>, C::This<'a>, D::This<'a>) + Send + Sync
            + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
        }
    }
    impl<Func, A, B, C, D, E> SystemFunction<fn(A, B, C, D, E)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E)
            + FnMut(A::This<'a>, B::This<'a>, C::This<'a>, D::This<'a>, E::This<'a>)
            + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
        }
    }
    impl<Func, A, B, C, D, E, F> SystemFunction<fn(A, B, C, D, E, F)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
        }
    }
    impl<Func, A, B, C, D, E, F, G> SystemFunction<fn(A, B, C, D, E, F, G)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F, G)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
                G::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            #[allow(non_snake_case)]
            let mut G = G::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
                G::construct(&mut G),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
        }
    }
    impl<Func, A, B, C, D, E, F, G, H> SystemFunction<fn(A, B, C, D, E, F, G, H)>
    for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F, G, H)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
                G::This<'a>,
                H::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            #[allow(non_snake_case)]
            let mut G = G::lock(state);
            #[allow(non_snake_case)]
            let mut H = H::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
                G::construct(&mut G),
                H::construct(&mut H),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
        }
    }
    impl<Func, A, B, C, D, E, F, G, H, I> SystemFunction<fn(A, B, C, D, E, F, G, H, I)>
    for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F, G, H, I)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
                G::This<'a>,
                H::This<'a>,
                I::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            #[allow(non_snake_case)]
            let mut G = G::lock(state);
            #[allow(non_snake_case)]
            let mut H = H::lock(state);
            #[allow(non_snake_case)]
            let mut I = I::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
                G::construct(&mut G),
                H::construct(&mut H),
                I::construct(&mut I),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
        }
    }
    impl<
        Func,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
    > SystemFunction<fn(A, B, C, D, E, F, G, H, I, J)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F, G, H, I, J)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
                G::This<'a>,
                H::This<'a>,
                I::This<'a>,
                J::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            #[allow(non_snake_case)]
            let mut G = G::lock(state);
            #[allow(non_snake_case)]
            let mut H = H::lock(state);
            #[allow(non_snake_case)]
            let mut I = I::lock(state);
            #[allow(non_snake_case)]
            let mut J = J::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
                G::construct(&mut G),
                H::construct(&mut H),
                I::construct(&mut I),
                J::construct(&mut J),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
        }
    }
    impl<
        Func,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
    > SystemFunction<fn(A, B, C, D, E, F, G, H, I, J, K)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F, G, H, I, J, K)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
                G::This<'a>,
                H::This<'a>,
                I::This<'a>,
                J::This<'a>,
                K::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            #[allow(non_snake_case)]
            let mut G = G::lock(state);
            #[allow(non_snake_case)]
            let mut H = H::lock(state);
            #[allow(non_snake_case)]
            let mut I = I::lock(state);
            #[allow(non_snake_case)]
            let mut J = J::lock(state);
            #[allow(non_snake_case)]
            let mut K = K::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
                G::construct(&mut G),
                H::construct(&mut H),
                I::construct(&mut I),
                J::construct(&mut J),
                K::construct(&mut K),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
        }
    }
    impl<
        Func,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
    > SystemFunction<fn(A, B, C, D, E, F, G, H, I, J, K, L)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F, G, H, I, J, K, L)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
                G::This<'a>,
                H::This<'a>,
                I::This<'a>,
                J::This<'a>,
                K::This<'a>,
                L::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
        L: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            #[allow(non_snake_case)]
            let mut G = G::lock(state);
            #[allow(non_snake_case)]
            let mut H = H::lock(state);
            #[allow(non_snake_case)]
            let mut I = I::lock(state);
            #[allow(non_snake_case)]
            let mut J = J::lock(state);
            #[allow(non_snake_case)]
            let mut K = K::lock(state);
            #[allow(non_snake_case)]
            let mut L = L::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
                G::construct(&mut G),
                H::construct(&mut H),
                I::construct(&mut I),
                J::construct(&mut J),
                K::construct(&mut K),
                L::construct(&mut L),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
                .chain(L::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
                .chain(L::get_component_types())
        }
    }
    impl<
        Func,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
    > SystemFunction<fn(A, B, C, D, E, F, G, H, I, J, K, L, M)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F, G, H, I, J, K, L, M)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
                G::This<'a>,
                H::This<'a>,
                I::This<'a>,
                J::This<'a>,
                K::This<'a>,
                L::This<'a>,
                M::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
        L: SystemParameter,
        M: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            #[allow(non_snake_case)]
            let mut G = G::lock(state);
            #[allow(non_snake_case)]
            let mut H = H::lock(state);
            #[allow(non_snake_case)]
            let mut I = I::lock(state);
            #[allow(non_snake_case)]
            let mut J = J::lock(state);
            #[allow(non_snake_case)]
            let mut K = K::lock(state);
            #[allow(non_snake_case)]
            let mut L = L::lock(state);
            #[allow(non_snake_case)]
            let mut M = M::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
                G::construct(&mut G),
                H::construct(&mut H),
                I::construct(&mut I),
                J::construct(&mut J),
                K::construct(&mut K),
                L::construct(&mut L),
                M::construct(&mut M),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
                .chain(L::get_resource_types())
                .chain(M::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
                .chain(L::get_component_types())
                .chain(M::get_component_types())
        }
    }
    impl<
        Func,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
    > SystemFunction<fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F, G, H, I, J, K, L, M, N)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
                G::This<'a>,
                H::This<'a>,
                I::This<'a>,
                J::This<'a>,
                K::This<'a>,
                L::This<'a>,
                M::This<'a>,
                N::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
        L: SystemParameter,
        M: SystemParameter,
        N: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            #[allow(non_snake_case)]
            let mut G = G::lock(state);
            #[allow(non_snake_case)]
            let mut H = H::lock(state);
            #[allow(non_snake_case)]
            let mut I = I::lock(state);
            #[allow(non_snake_case)]
            let mut J = J::lock(state);
            #[allow(non_snake_case)]
            let mut K = K::lock(state);
            #[allow(non_snake_case)]
            let mut L = L::lock(state);
            #[allow(non_snake_case)]
            let mut M = M::lock(state);
            #[allow(non_snake_case)]
            let mut N = N::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
                G::construct(&mut G),
                H::construct(&mut H),
                I::construct(&mut I),
                J::construct(&mut J),
                K::construct(&mut K),
                L::construct(&mut L),
                M::construct(&mut M),
                N::construct(&mut N),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
                .chain(L::get_resource_types())
                .chain(M::get_resource_types())
                .chain(N::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
                .chain(L::get_component_types())
                .chain(M::get_component_types())
                .chain(N::get_component_types())
        }
    }
    impl<
        Func,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
    > SystemFunction<fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
                G::This<'a>,
                H::This<'a>,
                I::This<'a>,
                J::This<'a>,
                K::This<'a>,
                L::This<'a>,
                M::This<'a>,
                N::This<'a>,
                O::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
        L: SystemParameter,
        M: SystemParameter,
        N: SystemParameter,
        O: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            #[allow(non_snake_case)]
            let mut G = G::lock(state);
            #[allow(non_snake_case)]
            let mut H = H::lock(state);
            #[allow(non_snake_case)]
            let mut I = I::lock(state);
            #[allow(non_snake_case)]
            let mut J = J::lock(state);
            #[allow(non_snake_case)]
            let mut K = K::lock(state);
            #[allow(non_snake_case)]
            let mut L = L::lock(state);
            #[allow(non_snake_case)]
            let mut M = M::lock(state);
            #[allow(non_snake_case)]
            let mut N = N::lock(state);
            #[allow(non_snake_case)]
            let mut O = O::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
                G::construct(&mut G),
                H::construct(&mut H),
                I::construct(&mut I),
                J::construct(&mut J),
                K::construct(&mut K),
                L::construct(&mut L),
                M::construct(&mut M),
                N::construct(&mut N),
                O::construct(&mut O),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
                .chain(L::get_resource_types())
                .chain(M::get_resource_types())
                .chain(N::get_resource_types())
                .chain(O::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
                .chain(L::get_component_types())
                .chain(M::get_component_types())
                .chain(N::get_component_types())
                .chain(O::get_component_types())
        }
    }
    impl<
        Func,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
    > SystemFunction<fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)> for Func
    where
        for<'a> Func: FnMut(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
            + FnMut(
                A::This<'a>,
                B::This<'a>,
                C::This<'a>,
                D::This<'a>,
                E::This<'a>,
                F::This<'a>,
                G::This<'a>,
                H::This<'a>,
                I::This<'a>,
                J::This<'a>,
                K::This<'a>,
                L::This<'a>,
                M::This<'a>,
                N::This<'a>,
                O::This<'a>,
                P::This<'a>,
            ) + Send + Sync + 'static,
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
        L: SystemParameter,
        M: SystemParameter,
        N: SystemParameter,
        O: SystemParameter,
        P: SystemParameter,
    {
        fn run(&mut self, state: RunState<'_>) {
            _ = state;
            #[allow(non_snake_case)]
            let mut A = A::lock(state);
            #[allow(non_snake_case)]
            let mut B = B::lock(state);
            #[allow(non_snake_case)]
            let mut C = C::lock(state);
            #[allow(non_snake_case)]
            let mut D = D::lock(state);
            #[allow(non_snake_case)]
            let mut E = E::lock(state);
            #[allow(non_snake_case)]
            let mut F = F::lock(state);
            #[allow(non_snake_case)]
            let mut G = G::lock(state);
            #[allow(non_snake_case)]
            let mut H = H::lock(state);
            #[allow(non_snake_case)]
            let mut I = I::lock(state);
            #[allow(non_snake_case)]
            let mut J = J::lock(state);
            #[allow(non_snake_case)]
            let mut K = K::lock(state);
            #[allow(non_snake_case)]
            let mut L = L::lock(state);
            #[allow(non_snake_case)]
            let mut M = M::lock(state);
            #[allow(non_snake_case)]
            let mut N = N::lock(state);
            #[allow(non_snake_case)]
            let mut O = O::lock(state);
            #[allow(non_snake_case)]
            let mut P = P::lock(state);
            self(
                A::construct(&mut A),
                B::construct(&mut B),
                C::construct(&mut C),
                D::construct(&mut D),
                E::construct(&mut E),
                F::construct(&mut F),
                G::construct(&mut G),
                H::construct(&mut H),
                I::construct(&mut I),
                J::construct(&mut J),
                K::construct(&mut K),
                L::construct(&mut L),
                M::construct(&mut M),
                N::construct(&mut N),
                O::construct(&mut O),
                P::construct(&mut P),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
                .chain(L::get_resource_types())
                .chain(M::get_resource_types())
                .chain(N::get_resource_types())
                .chain(O::get_resource_types())
                .chain(P::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
                .chain(L::get_component_types())
                .chain(M::get_component_types())
                .chain(N::get_component_types())
                .chain(O::get_component_types())
                .chain(P::get_component_types())
        }
    }
}
pub mod system_parameters {
    use crate::{
        resource::{Res, ResMut, Resource},
        system::{BorrowType, RunState},
    };
    use parking_lot::{
        MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLockReadGuard, RwLockWriteGuard,
    };
    use std::any::TypeId;
    pub trait SystemParameter: Send + Sync {
        type This<'this>;
        type Lock<'state>;
        fn lock(state: RunState<'_>) -> Self::Lock<'_>;
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this>;
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)>;
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)>;
    }
    impl<'a, R> SystemParameter for Res<'a, R>
    where
        R: Resource,
    {
        type This<'this> = Res<'this, R>;
        type Lock<'state> = MappedRwLockReadGuard<'state, R>;
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            RwLockReadGuard::map(
                state
                    .resources
                    .get(&TypeId::of::<R>())
                    .expect("Non-Option Res expects the resource to always exist")
                    .try_read()
                    .expect("the lock should always be available"),
                |resource| resource.downcast_ref::<R>().unwrap(),
            )
        }
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            Res { inner: state }
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::once((TypeId::of::<R>(), BorrowType::Immutable))
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
    }
    impl<'a, R> SystemParameter for ResMut<'a, R>
    where
        R: Resource,
    {
        type This<'this> = ResMut<'this, R>;
        type Lock<'state> = MappedRwLockWriteGuard<'state, R>;
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            RwLockWriteGuard::map(
                state
                    .resources
                    .get(&TypeId::of::<R>())
                    .expect("Non-Option ResMut expects the resource to always exist")
                    .try_write()
                    .expect("the lock should always be available"),
                |resource| resource.downcast_mut::<R>().unwrap(),
            )
        }
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            ResMut { inner: state }
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::once((TypeId::of::<R>(), BorrowType::Mutable))
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
    }
    impl<'a, R> SystemParameter for Option<Res<'a, R>>
    where
        R: Resource,
    {
        type This<'this> = Option<Res<'this, R>>;
        type Lock<'state> = Option<MappedRwLockReadGuard<'state, R>>;
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            Some(
                RwLockReadGuard::map(
                    state
                        .resources
                        .get(&TypeId::of::<R>())?
                        .try_read()
                        .expect("the lock should always be available"),
                    |resource| resource.downcast_ref::<R>().unwrap(),
                ),
            )
        }
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            Some(Res { inner: state.as_ref()? })
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::once((TypeId::of::<R>(), BorrowType::Immutable))
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
    }
    impl<'a, R> SystemParameter for Option<ResMut<'a, R>>
    where
        R: Resource,
    {
        type This<'this> = Option<ResMut<'this, R>>;
        type Lock<'state> = Option<MappedRwLockWriteGuard<'state, R>>;
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            Some(
                RwLockWriteGuard::map(
                    state
                        .resources
                        .get(&TypeId::of::<R>())?
                        .try_write()
                        .expect("the lock should always be available"),
                    |resource| resource.downcast_mut::<R>().unwrap(),
                ),
            )
        }
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            Some(ResMut { inner: state.as_mut()? })
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::once((TypeId::of::<R>(), BorrowType::Mutable))
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
    }
    impl SystemParameter for () {
        type This<'this> = ();
        type Lock<'state> = ();
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            ()
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let () = state;
            ()
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
        }
    }
    impl<A> SystemParameter for (A,)
    where
        A: SystemParameter,
    {
        type This<'this> = (A::This<'this>,);
        type Lock<'state> = (A::Lock<'state>,);
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (A::lock(state),)
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A,) = state;
            (A::construct(A),)
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty().chain(A::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty().chain(A::get_component_types())
        }
    }
    impl<A, B> SystemParameter for (A, B)
    where
        A: SystemParameter,
        B: SystemParameter,
    {
        type This<'this> = (A::This<'this>, B::This<'this>);
        type Lock<'state> = (A::Lock<'state>, B::Lock<'state>);
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (A::lock(state), B::lock(state))
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B) = state;
            (A::construct(A), B::construct(B))
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
        }
    }
    impl<A, B, C> SystemParameter for (A, B, C)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
    {
        type This<'this> = (A::This<'this>, B::This<'this>, C::This<'this>);
        type Lock<'state> = (A::Lock<'state>, B::Lock<'state>, C::Lock<'state>);
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (A::lock(state), B::lock(state), C::lock(state))
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C) = state;
            (A::construct(A), B::construct(B), C::construct(C))
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
        }
    }
    impl<A, B, C, D> SystemParameter for (A, B, C, D)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (A::lock(state), B::lock(state), C::lock(state), D::lock(state))
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D) = state;
            (A::construct(A), B::construct(B), C::construct(C), D::construct(D))
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
        }
    }
    impl<A, B, C, D, E> SystemParameter for (A, B, C, D, E)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
        }
    }
    impl<A, B, C, D, E, F> SystemParameter for (A, B, C, D, E, F)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
        }
    }
    impl<A, B, C, D, E, F, G> SystemParameter for (A, B, C, D, E, F, G)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
            G::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
            G::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
                G::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F, G) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
                G::construct(G),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
        }
    }
    impl<A, B, C, D, E, F, G, H> SystemParameter for (A, B, C, D, E, F, G, H)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
            G::This<'this>,
            H::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
            G::Lock<'state>,
            H::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
                G::lock(state),
                H::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F, G, H) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
                G::construct(G),
                H::construct(H),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
        }
    }
    impl<A, B, C, D, E, F, G, H, I> SystemParameter for (A, B, C, D, E, F, G, H, I)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
            G::This<'this>,
            H::This<'this>,
            I::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
            G::Lock<'state>,
            H::Lock<'state>,
            I::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
                G::lock(state),
                H::lock(state),
                I::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F, G, H, I) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
                G::construct(G),
                H::construct(H),
                I::construct(I),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
        }
    }
    impl<A, B, C, D, E, F, G, H, I, J> SystemParameter for (A, B, C, D, E, F, G, H, I, J)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
            G::This<'this>,
            H::This<'this>,
            I::This<'this>,
            J::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
            G::Lock<'state>,
            H::Lock<'state>,
            I::Lock<'state>,
            J::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
                G::lock(state),
                H::lock(state),
                I::lock(state),
                J::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F, G, H, I, J) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
                G::construct(G),
                H::construct(H),
                I::construct(I),
                J::construct(J),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
        }
    }
    impl<A, B, C, D, E, F, G, H, I, J, K> SystemParameter
    for (A, B, C, D, E, F, G, H, I, J, K)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
            G::This<'this>,
            H::This<'this>,
            I::This<'this>,
            J::This<'this>,
            K::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
            G::Lock<'state>,
            H::Lock<'state>,
            I::Lock<'state>,
            J::Lock<'state>,
            K::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
                G::lock(state),
                H::lock(state),
                I::lock(state),
                J::lock(state),
                K::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F, G, H, I, J, K) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
                G::construct(G),
                H::construct(H),
                I::construct(I),
                J::construct(J),
                K::construct(K),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
        }
    }
    impl<A, B, C, D, E, F, G, H, I, J, K, L> SystemParameter
    for (A, B, C, D, E, F, G, H, I, J, K, L)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
        L: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
            G::This<'this>,
            H::This<'this>,
            I::This<'this>,
            J::This<'this>,
            K::This<'this>,
            L::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
            G::Lock<'state>,
            H::Lock<'state>,
            I::Lock<'state>,
            J::Lock<'state>,
            K::Lock<'state>,
            L::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
                G::lock(state),
                H::lock(state),
                I::lock(state),
                J::lock(state),
                K::lock(state),
                L::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F, G, H, I, J, K, L) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
                G::construct(G),
                H::construct(H),
                I::construct(I),
                J::construct(J),
                K::construct(K),
                L::construct(L),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
                .chain(L::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
                .chain(L::get_component_types())
        }
    }
    impl<A, B, C, D, E, F, G, H, I, J, K, L, M> SystemParameter
    for (A, B, C, D, E, F, G, H, I, J, K, L, M)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
        L: SystemParameter,
        M: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
            G::This<'this>,
            H::This<'this>,
            I::This<'this>,
            J::This<'this>,
            K::This<'this>,
            L::This<'this>,
            M::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
            G::Lock<'state>,
            H::Lock<'state>,
            I::Lock<'state>,
            J::Lock<'state>,
            K::Lock<'state>,
            L::Lock<'state>,
            M::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
                G::lock(state),
                H::lock(state),
                I::lock(state),
                J::lock(state),
                K::lock(state),
                L::lock(state),
                M::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F, G, H, I, J, K, L, M) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
                G::construct(G),
                H::construct(H),
                I::construct(I),
                J::construct(J),
                K::construct(K),
                L::construct(L),
                M::construct(M),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
                .chain(L::get_resource_types())
                .chain(M::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
                .chain(L::get_component_types())
                .chain(M::get_component_types())
        }
    }
    impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> SystemParameter
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
        L: SystemParameter,
        M: SystemParameter,
        N: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
            G::This<'this>,
            H::This<'this>,
            I::This<'this>,
            J::This<'this>,
            K::This<'this>,
            L::This<'this>,
            M::This<'this>,
            N::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
            G::Lock<'state>,
            H::Lock<'state>,
            I::Lock<'state>,
            J::Lock<'state>,
            K::Lock<'state>,
            L::Lock<'state>,
            M::Lock<'state>,
            N::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
                G::lock(state),
                H::lock(state),
                I::lock(state),
                J::lock(state),
                K::lock(state),
                L::lock(state),
                M::lock(state),
                N::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F, G, H, I, J, K, L, M, N) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
                G::construct(G),
                H::construct(H),
                I::construct(I),
                J::construct(J),
                K::construct(K),
                L::construct(L),
                M::construct(M),
                N::construct(N),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
                .chain(L::get_resource_types())
                .chain(M::get_resource_types())
                .chain(N::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
                .chain(L::get_component_types())
                .chain(M::get_component_types())
                .chain(N::get_component_types())
        }
    }
    impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> SystemParameter
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
        L: SystemParameter,
        M: SystemParameter,
        N: SystemParameter,
        O: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
            G::This<'this>,
            H::This<'this>,
            I::This<'this>,
            J::This<'this>,
            K::This<'this>,
            L::This<'this>,
            M::This<'this>,
            N::This<'this>,
            O::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
            G::Lock<'state>,
            H::Lock<'state>,
            I::Lock<'state>,
            J::Lock<'state>,
            K::Lock<'state>,
            L::Lock<'state>,
            M::Lock<'state>,
            N::Lock<'state>,
            O::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
                G::lock(state),
                H::lock(state),
                I::lock(state),
                J::lock(state),
                K::lock(state),
                L::lock(state),
                M::lock(state),
                N::lock(state),
                O::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
                G::construct(G),
                H::construct(H),
                I::construct(I),
                J::construct(J),
                K::construct(K),
                L::construct(L),
                M::construct(M),
                N::construct(N),
                O::construct(O),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
                .chain(L::get_resource_types())
                .chain(M::get_resource_types())
                .chain(N::get_resource_types())
                .chain(O::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
                .chain(L::get_component_types())
                .chain(M::get_component_types())
                .chain(N::get_component_types())
                .chain(O::get_component_types())
        }
    }
    impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> SystemParameter
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
    where
        A: SystemParameter,
        B: SystemParameter,
        C: SystemParameter,
        D: SystemParameter,
        E: SystemParameter,
        F: SystemParameter,
        G: SystemParameter,
        H: SystemParameter,
        I: SystemParameter,
        J: SystemParameter,
        K: SystemParameter,
        L: SystemParameter,
        M: SystemParameter,
        N: SystemParameter,
        O: SystemParameter,
        P: SystemParameter,
    {
        type This<'this> = (
            A::This<'this>,
            B::This<'this>,
            C::This<'this>,
            D::This<'this>,
            E::This<'this>,
            F::This<'this>,
            G::This<'this>,
            H::This<'this>,
            I::This<'this>,
            J::This<'this>,
            K::This<'this>,
            L::This<'this>,
            M::This<'this>,
            N::This<'this>,
            O::This<'this>,
            P::This<'this>,
        );
        type Lock<'state> = (
            A::Lock<'state>,
            B::Lock<'state>,
            C::Lock<'state>,
            D::Lock<'state>,
            E::Lock<'state>,
            F::Lock<'state>,
            G::Lock<'state>,
            H::Lock<'state>,
            I::Lock<'state>,
            J::Lock<'state>,
            K::Lock<'state>,
            L::Lock<'state>,
            M::Lock<'state>,
            N::Lock<'state>,
            O::Lock<'state>,
            P::Lock<'state>,
        );
        #[allow(clippy::unused_unit)]
        fn lock(state: RunState<'_>) -> Self::Lock<'_> {
            _ = state;
            (
                A::lock(state),
                B::lock(state),
                C::lock(state),
                D::lock(state),
                E::lock(state),
                F::lock(state),
                G::lock(state),
                H::lock(state),
                I::lock(state),
                J::lock(state),
                K::lock(state),
                L::lock(state),
                M::lock(state),
                N::lock(state),
                O::lock(state),
                P::lock(state),
            )
        }
        #[allow(clippy::unused_unit)]
        fn construct<'this>(state: &'this mut Self::Lock<'_>) -> Self::This<'this> {
            #[allow(non_snake_case)]
            let (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) = state;
            (
                A::construct(A),
                B::construct(B),
                C::construct(C),
                D::construct(D),
                E::construct(E),
                F::construct(F),
                G::construct(G),
                H::construct(H),
                I::construct(I),
                J::construct(J),
                K::construct(K),
                L::construct(L),
                M::construct(M),
                N::construct(N),
                O::construct(O),
                P::construct(P),
            )
        }
        fn get_resource_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_resource_types())
                .chain(B::get_resource_types())
                .chain(C::get_resource_types())
                .chain(D::get_resource_types())
                .chain(E::get_resource_types())
                .chain(F::get_resource_types())
                .chain(G::get_resource_types())
                .chain(H::get_resource_types())
                .chain(I::get_resource_types())
                .chain(J::get_resource_types())
                .chain(K::get_resource_types())
                .chain(L::get_resource_types())
                .chain(M::get_resource_types())
                .chain(N::get_resource_types())
                .chain(O::get_resource_types())
                .chain(P::get_resource_types())
        }
        fn get_component_types() -> impl Iterator<Item = (TypeId, BorrowType)> {
            std::iter::empty()
                .chain(A::get_component_types())
                .chain(B::get_component_types())
                .chain(C::get_component_types())
                .chain(D::get_component_types())
                .chain(E::get_component_types())
                .chain(F::get_component_types())
                .chain(G::get_component_types())
                .chain(H::get_component_types())
                .chain(I::get_component_types())
                .chain(J::get_component_types())
                .chain(K::get_component_types())
                .chain(L::get_component_types())
                .chain(M::get_component_types())
                .chain(N::get_component_types())
                .chain(O::get_component_types())
                .chain(P::get_component_types())
        }
    }
}
