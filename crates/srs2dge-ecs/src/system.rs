use specs::DispatcherBuilder;

//

pub trait System {
    fn add_system(&self, dispatcher_builder: &mut DispatcherBuilder<'static, 'static>);
}

//

impl<F: Fn() + Send + Clone + 'static> System for F {
    fn add_system(&self, dispatcher_builder: &mut DispatcherBuilder<'static, 'static>) {
        struct X<F: Fn()> {
            inner: F,
        }
        impl<'a, F: Fn()> specs::System<'a> for X<F> {
            type SystemData = ();
            fn run(&mut self, _: Self::SystemData) {
                (self.inner)()
            }
        }
        dispatcher_builder.add(
            X {
                inner: self.clone(),
            },
            "",
            &[],
        );
    }
}
/* impl<F: Fn(usize) + Send + Clone + 'static> System for F {
    fn add_system(&self, dispatcher_builder: &mut DispatcherBuilder<'static, 'static>) {
        struct X<F: Fn()> {
            inner: F,
        }
        impl<'a, F: Fn()> specs::System<'a> for X<F> {
            type SystemData = ();
            fn run(&mut self, _: Self::SystemData) {
                (self.inner)()
            }
        }
        dispatcher_builder.add(
            X {
                inner: self.clone(),
            },
            "",
            &[],
        );
    }
} */
// impl<T, F: Fn(T) + Send + 'static> System<((), T)> for F {}
// impl<T, U, F: Fn(T, U) + Send + 'static> System<((), T, U)> for F {}
// impl<T, U, V, F: Fn(T, U, V) + Send + 'static> System<((), T, U, V)> for F {}

//
