use std::marker::PhantomData;

use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug)]
// #[derive(Debug)]
struct Pod<'a, T> {
    first: u32,
    second: &'a T,
    #[builder(default)]
    third: f32,
}

pub struct Assigned<T>(T);
pub struct Empty<T>(::std::marker::PhantomData<T>);

pub struct WithDefault<T>(T);

trait AssignedOrDefault {
    type ValueType;
    fn value_or_default(self) -> Self::ValueType;
}

trait Assignable<Origin: ?Sized> {
    type AssignedInner;
    fn assign(self, t: Origin) -> Assigned<Self::AssignedInner>;
}

impl<T> Assignable<T> for Empty<T> {
    type AssignedInner = T;
    fn assign(self, t: T) -> Assigned<Self::AssignedInner> {
        Assigned(t)
    }
}

impl<T> Assignable<T> for Empty<Option<T>> {
    type AssignedInner = Option<T>;

    fn assign(self, t: T) -> Assigned<Self::AssignedInner> {
        Assigned(Some(t))
    }
}

impl<T> Assignable<T> for WithDefault<T> {
    type AssignedInner = T;
    fn assign(self, t: T) -> Assigned<Self::AssignedInner> {
        Assigned(t)
    }
}

impl<T> AssignedOrDefault for Assigned<T> {
    type ValueType = T;

    fn value_or_default(self) -> Self::ValueType {
        self.0
    }
}

impl<T> AssignedOrDefault for WithDefault<T> {
    type ValueType = T;

    fn value_or_default(self) -> Self::ValueType {
        self.0
    }
}

impl<T> Default for Empty<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

struct PodBuilder2<'a, T, State> {
    state: State,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> PodBuilder2<'a, T, (Empty<u32>, Empty<&'a T>, WithDefault<f32>)> {
    pub fn new() -> Self {
        Self {
            state: (
                Empty::default(),
                Empty::default(),
                // @note(georgios) we need a closure in here that sets the default value
                // but that's no problem :)
                //
                WithDefault(Default::default()),
            ),
            phantom: Default::default(),
        }
    }
}

impl<'a, T, U, V, W> PodBuilder2<'a, T, (U, V, W)>
where
    U: Assignable<u32, AssignedInner = u32>,
{
    pub fn first(self, first: u32) -> PodBuilder2<'a, T, (Assigned<u32>, V, W)> {
        let state = (self.state.0.assign(first), self.state.1, self.state.2);
        PodBuilder2 {
            state,
            phantom: Default::default(),
        }
    }
}

impl<'a, T, U, V, W> PodBuilder2<'a, T, (U, V, W)>
where
    V: Assignable<&'a T, AssignedInner = &'a T>,
{
    pub fn second(self, second: &'a T) -> PodBuilder2<'a, T, (U, Assigned<&'a T>, W)> {
        let state = (self.state.0, self.state.1.assign(second), self.state.2);
        PodBuilder2 {
            state,
            phantom: Default::default(),
        }
    }
}

impl<'a, T, U, V, W> PodBuilder2<'a, T, (U, V, W)>
where
    W: Assignable<f32, AssignedInner = f32>,
{
    pub fn third(self, third: f32) -> PodBuilder2<'a, T, (U, V, Assigned<W::AssignedInner>)> {
        let state = (self.state.0, self.state.1, self.state.2.assign(third));
        PodBuilder2 {
            state,
            phantom: Default::default(),
        }
    }
}

impl<'a, T, U, V, W> PodBuilder2<'a, T, (U, V, W)>
where
    U: AssignedOrDefault<ValueType = u32>,
    V: AssignedOrDefault<ValueType = &'a T>,
    W: AssignedOrDefault<ValueType = f32>,
{
    pub fn build(self) -> Pod<'a, T> {
        Pod {
            first: self.state.0.value_or_default(),
            second: self.state.1.value_or_default(),
            third: self.state.2.value_or_default(),
        }
    }
}

// #[inline(never)]
// fn get_pod(x: u32, s: u32) -> Pod {
//     let pod = Pod::builder().x(x).s(s).build();
//     pod
// }

fn main() {
    // let pod = get_pod(0xdead, 0xbeef);
    // let pod = Pod { x: 313373, s: 1337 };
    // let pod = Pod::builder().x(1).s(2).build();

    let pod = PodBuilder2::new().first(1).second(&2).build();
    let pod = PodBuilder2::new().second(&1).first(2).build();
    let pod = PodBuilder2::new().first(1).third(3.).second(&"hi").build();

    // println!("{:?}", pod);
}
