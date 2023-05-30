use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    ops::Deref,
    rc::Rc,
};

use crate::context::ViewContext;
type ArkhamResult = anyhow::Result<ArkhamState>;
pub enum ArkhamState {
    Noop,
}

/// The container stores typed resource and state objects and provides
/// them to component functions.
#[derive(Default)]
pub struct Container {
    bindings: HashMap<TypeId, Box<dyn Any>>,
}

impl Container {
    /// insert a type binding into the container. This is used to provide an
    /// object to functions executed by Container::call.
    ///
    /// App::insert_ressource and App::isnert_state proxies to this function.
    pub(crate) fn bind<T: Any>(&mut self, val: T) {
        self.bindings.insert(val.type_id(), Box::new(val));
    }

    /// Get an object from the store by its type. This is a utility function
    /// to extract an object directly, instead of using the container to
    /// inject objects into a function's arguments.
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.bindings
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    /// Call a function while providing dependency injcetion.
    pub fn call<F, Args>(&self, view: &mut ViewContext, callable: &F)
    where
        F: Callable<Args>,
        Args: FromContainer,
    {
        let _ = callable.call(view, Args::from_container(self));
    }
}

/// A wrapper for state objcets. This internally holds a reference counted
/// poitner to the object and is used when injecting itno functions.
pub struct State<T: ?Sized>(Rc<RefCell<T>>);

impl<T> State<T> {
    /// Create a new state wrapper.
    pub fn new(val: T) -> Self {
        State(Rc::new(RefCell::new(val)))
    }

    /// Returns a mutable reference to the underlying state object.
    ///
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// struct MyState(i32);
    ///
    /// let state = State::new(MyState(4));
    /// state.get_mut().0 = 6;
    /// assert_eq!(state.get().0, 6);
    /// ```
    pub fn get_mut(&self) -> std::cell::RefMut<T> {
        RefCell::borrow_mut(&self.0)
    }

    // Returns an immutable reference to the underlying state object.
    /// Example:
    /// ```
    /// use arkham::prelude::*;
    /// struct MyState(i32);
    ///
    /// let state = State::new(MyState(4));
    /// assert_eq!(state.get().0, 4);
    /// ```
    pub fn get(&self) -> std::cell::Ref<T> {
        RefCell::borrow(&self.0)
    }
}

impl<T: ?Sized> Clone for State<T> {
    fn clone(&self) -> State<T> {
        State(self.0.clone())
    }
}

impl<T: ?Sized + 'static> FromContainer for State<T> {
    fn from_container(container: &Container) -> Self {
        container.get::<Self>().expect("type not found").clone()
    }
}

/// A wrapper for resources stored within the app. This wrapper is returned
/// when objects are injected into component functions and provide immutable
/// access
#[derive(Debug)]
pub struct Res<T: ?Sized>(Rc<T>);

impl<T> Res<T> {
    pub fn new(val: T) -> Self {
        Res(Rc::new(val))
    }
}

impl<T: ?Sized> Res<T> {
    pub fn get(&self) -> &T {
        self.0.as_ref()
    }
}

impl<T: ?Sized> Clone for Res<T> {
    fn clone(&self) -> Res<T> {
        Res(self.0.clone())
    }
}

impl<T: ?Sized> Deref for Res<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Rc<T> {
        &self.0
    }
}

impl<T: ?Sized + 'static> FromContainer for Res<T> {
    fn from_container(container: &Container) -> Self {
        container.get::<Self>().expect("type not found").clone()
    }
}

impl<Func> Callable<()> for Func
where
    Func: Fn(&mut ViewContext),
{
    #[inline]
    fn call(&self, view: &mut ViewContext, _args: ()) -> ArkhamResult {
        (self)(view);
        Ok(ArkhamState::Noop)
    }
}

/// Callable must be implemented for functions that can be used as component
/// functions. They are given a ViewContext for the component function and
/// injectable arguments.
pub trait Callable<Args> {
    fn call(&self, view: &mut ViewContext, args: Args) -> ArkhamResult;
}

/// FromContainer must be implmented for objects that can be injected into
/// component functions. This includes the Res and State structs.
pub trait FromContainer {
    fn from_container(container: &Container) -> Self;
}

impl FromContainer for () {
    #[inline]
    fn from_container(_container: &Container) -> Self {}
}

macro_rules! callable_tuple ({ $($param:ident)* } => {
    impl<Func, $($param,)*> Callable<($($param,)*)> for Func
    where
        Func: Fn(&mut ViewContext, $($param),*),
    {
        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, view: &mut ViewContext , ($($param,)*): ($($param,)*)) -> ArkhamResult{
            (self)(view, $($param,)*);
            Ok(ArkhamState::Noop)
        }
    }
});

// callable_tuple! {}
callable_tuple! { A }
callable_tuple! { A B }
callable_tuple! { A B C }
callable_tuple! { A B C D }
callable_tuple! { A B C D E }
callable_tuple! { A B C D E F }
callable_tuple! { A B C D E F G }
callable_tuple! { A B C D E F G H }
callable_tuple! { A B C D E F G H I }
callable_tuple! { A B C D E F G H I J }
callable_tuple! { A B C D E F G H I J K }
callable_tuple! { A B C D E F G H I J K L }

macro_rules! tuple_from_tm {
        ( $($T: ident )+ ) => {
            impl<$($T: FromContainer),+> FromContainer for ($($T,)+)
            {
                #[inline]
                fn from_container(container: &Container) -> Self {
                    ($($T::from_container(container),)+)
                }
            }
        };
    }

tuple_from_tm! { A }
tuple_from_tm! { A B }
tuple_from_tm! { A B C }
tuple_from_tm! { A B C D }
tuple_from_tm! { A B C D E }
tuple_from_tm! { A B C D E F }
tuple_from_tm! { A B C D E F G }
tuple_from_tm! { A B C D E F G H }
tuple_from_tm! { A B C D E F G H I }
tuple_from_tm! { A B C D E F G H I J }
tuple_from_tm! { A B C D E F G H I J K }
tuple_from_tm! { A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{container::Container, prelude::ViewContext};

    #[test]
    fn test_no_params() {
        fn test_f(_ctx: &mut ViewContext) {}
        let container = Rc::new(RefCell::new(Container::default()));
        let mut ctx = ViewContext::new(container.clone(), (1, 1).into());
        container.borrow().call(&mut ctx, &test_f);
    }
}
