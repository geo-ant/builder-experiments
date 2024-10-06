use bon::Builder as BonBuilder;
use buildstructor::Builder as Buildstructor;
use derive_builder::Builder as DeriveBuilder;
use std::{marker::PhantomData, str::FromStr};
use typed_builder::TypedBuilder;

#[derive(Debug, BonBuilder)]
// #[derive(TypedBuilder)] // either this or bon
// #[derive(Debug)]
struct Pod<'a, S, T>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    first: S,
    second: &'a T,
    #[builder(default)]
    third: f32,
}

/// dummy trait for enforcing more complicated relationships
trait MyTrait {
    type AssocType: Clone;
}

impl MyTrait for f32 {
    type AssocType = i32;
}

impl MyTrait for i32 {
    type AssocType = f32;
}

impl MyTrait for &str {
    type AssocType = usize;
}

pub struct Assigned<T>(T);
#[derive(Default)]
//@note this could be the unit type directly
pub struct Empty;

// this might not look necessary at first but it allows us to introduce the
// generic arguments bit by bit.
//@note this could be phantom data directly
pub struct Placeholder<T>(PhantomData<T>);

impl<T> Assignable<T> for Placeholder<T> {
    fn assign(self, t: T) -> Assigned<T> {
        Assigned(t)
    }
}

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

struct PodBuilder2<State> {
    state: State,
}

impl PodBuilder2<(Empty, Empty, WithDefault<f32>)> {
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
        }
    }
}

impl<U, V, W> PodBuilder2<(U, V, W)> {
    // @note(geo) we can even define an #[into] attribute that changes the signature
    // of the builder function here from String to impl Into<String> (also possible in general)
    // pub fn first(self, first: String) -> PodBuilder2<'a, T, (Assigned<String>, V, W)> {
    pub fn first<S: std::fmt::Display>(self, first: S) -> PodBuilder2<(Assigned<S>, V, W)>
    where
        U: Assignable<S>,
    {
        let state = (self.state.0.assign(first), self.state.1, self.state.2);
        PodBuilder2 { state }
    }

    pub fn second<'a, T>(self, second: &'a T) -> PodBuilder2<(U, Assigned<&'a T>, W)>
    where
        V: Assignable<&'a T>,
        T: std::fmt::Debug + MyTrait,
    {
        let state = (self.state.0, self.state.1.assign(second), self.state.2);
        PodBuilder2 { state }
    }
    pub fn third(self, third: f32) -> PodBuilder2<(U, V, Assigned<f32>)>
    where
        W: Assignable<f32>,
    {
        let state = (self.state.0, self.state.1, self.state.2.assign(third));
        PodBuilder2 { state }
    }
}

impl<'a, S, T, U, V, W> PodBuilder2<(U, V, W)>
where
    U: AssignedOrDefault<ValueType = S>,
    V: AssignedOrDefault<ValueType = &'a T>,
    W: AssignedOrDefault<ValueType = f32>,
    T: std::fmt::Debug + 'a + MyTrait,
    S: std::fmt::Display,
{
    pub fn build(self) -> Pod<'a, S, T> {
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

    let stemcell = PodBuilder2::new().first("hi");

    let arg_count = std::env::args().count();

    if arg_count > 3 {
        let pod = stemcell.second(&1).build();
    } else {
        let pod = stemcell.second(&"hi").build();
    }

    // @note this code does not work for typed-builder
    // let stemcell = Pod::builder().first("hello".into()).third(1337.);

    // if some_count > 2 {
    //     let pod = stemcell.second(&1).build();
    // } else {
    //     let pod = stemcell.second(&"hi").build();
    // }

    // println!("{:?}", pod);
}
