use std::{marker::PhantomData, str::FromStr};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug)]
// #[derive(Debug)]
struct Pod<'a, T> {
    first: String,
    second: &'a T,
    #[builder(default)]
    third: f32,
}

pub struct Assigned<T>(T);
#[derive(Default)]
pub struct Empty {}

pub struct WithDefault<T>(T);

trait AssignedOrDefault {
    type ValueType;
    fn value_or_default(self) -> Self::ValueType;
}

trait Assignable<T> {
    fn assign(self, t: T) -> Assigned<T>;
}

impl<T> Assignable<T> for Empty {
    fn assign(self, t: T) -> Assigned<T> {
        Assigned(t)
    }
}

impl<T> Assignable<T> for WithDefault<T> {
    fn assign(self, t: T) -> Assigned<T> {
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

struct PodBuilder2<'a, T, State> {
    state: State,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> PodBuilder2<'a, T, (Empty, Empty, WithDefault<f32>)> {
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
    U: Assignable<String>,
{
    // @note(geo) we can even define an #[into] attribute that changes the signature
    // of the builder function here from String to impl Into<String> (also possible in general)
    // pub fn first(self, first: String) -> PodBuilder2<'a, T, (Assigned<String>, V, W)> {
    pub fn first(self, first: impl Into<String>) -> PodBuilder2<'a, T, (Assigned<String>, V, W)> {
        let state = (
            self.state.0.assign(first.into()),
            self.state.1,
            self.state.2,
        );
        PodBuilder2 {
            state,
            phantom: Default::default(),
        }
    }
}

impl<'a, T, U, V, W> PodBuilder2<'a, T, (U, V, W)>
where
    V: Assignable<&'a T>,
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
    W: Assignable<f32>,
{
    pub fn third(self, third: f32) -> PodBuilder2<'a, T, (U, V, Assigned<f32>)> {
        let state = (self.state.0, self.state.1, self.state.2.assign(third));
        PodBuilder2 {
            state,
            phantom: Default::default(),
        }
    }
}

impl<'a, T, U, V, W> PodBuilder2<'a, T, (U, V, W)>
where
    U: AssignedOrDefault<ValueType = String>,
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

    let pod = PodBuilder2::new().first("adda").second(&2.).build();
    let pod = PodBuilder2::new()
        .second(&1)
        .first(String::from("abc"))
        .build();
    let pod = PodBuilder2::new()
        .first("123")
        .third(3.)
        .second(&"hi")
        .build();

    // println!("{:?}", pod);
}
