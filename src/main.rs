use std::marker::PhantomData;

use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug)]
// #[derive(Debug)]
struct Pod<'a, T> {
    first: u32,
    second: &'a T,
    #[builder(default)]
    third: Option<f32>,
}

pub struct Assigned<T>(T);
pub struct Empty<T>(PhantomData<T>);

pub struct WithDefault<T>(T);

trait Assignable<From, To> {
    fn assign(self, from: From) -> Assigned<To>;
}

impl<T> Assignable<T, T> for Empty<T> {
    fn assign(self, from: T) -> Assigned<T> {
        Assigned(from)
    }
}

impl<T> Assignable<T, Option<T>> for Empty<Option<T>> {
    fn assign(self, from: T) -> Assigned<Option<T>> {
        Assigned(Some(from))
    }
}

impl<T> Assignable<T, T> for WithDefault<T> {
    fn assign(self, from: T) -> Assigned<T> {
        Assigned(from)
    }
}

impl<T> Assignable<T, Option<T>> for WithDefault<Option<T>> {
    fn assign(self, from: T) -> Assigned<Option<T>> {
        Assigned(Some(from))
    }
}

trait AssignedOrDefault {
    type ValueType;
    fn value_or_default(self) -> Self::ValueType;
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

impl<'a, T> PodBuilder2<'a, T, (Empty<u32>, Empty<&'a T>, WithDefault<Option<f32>>)> {
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

impl<'a, T, U, V, W> PodBuilder2<'a, T, (U, V, W)> {
    pub fn first<Origin>(self, first: Origin) -> PodBuilder2<'a, T, (Assigned<u32>, V, W)>
    where
        U: Assignable<Origin, u32>,
    {
        let state = (self.state.0.assign(first), self.state.1, self.state.2);
        PodBuilder2 {
            state,
            phantom: Default::default(),
        }
    }
}

impl<'a, T, U, V, W> PodBuilder2<'a, T, (U, V, W)> {
    pub fn second<Origin>(self, second: Origin) -> PodBuilder2<'a, T, (U, Assigned<&'a T>, W)>
    where
        V: Assignable<Origin, &'a T>,
    {
        let state = (self.state.0, self.state.1.assign(second), self.state.2);
        PodBuilder2 {
            state,
            phantom: Default::default(),
        }
    }
}

impl<'a, T, U, V, W> PodBuilder2<'a, T, (U, V, W)> {
    pub fn third<Origin>(self, third: Origin) -> PodBuilder2<'a, T, (U, V, Assigned<Option<f32>>)>
    where
        W: Assignable<Origin, Option<f32>>,
    {
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
    W: AssignedOrDefault<ValueType = Option<f32>>,
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
    let pod = PodBuilder2::new()
        .first(1)
        .third(Some(3.))
        .second(&"hi")
        .build();

    // println!("{:?}", pod);
}
