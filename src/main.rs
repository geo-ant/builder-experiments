use bon::Builder as BonBuilder;
use buildstructor::Builder as Buildstructor;
use derive_builder::Builder as DeriveBuilder;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};
use typed_builder::TypedBuilder;

mod crosschecks;

mod blog;

// #[derive(Debug, BonBuilder)]
// those constructor crates are mutually exclusive
// #[derive(TypedBuilder)]
// #[derive(Debug)]
// #[derive(Buildstructor)]
// #[derive(typestate_builder::TypestateBuilder)] //<- not implemented
struct Pod<'a, S, T>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTraitX,
{
    first: S,
    second: &'a T,
    // #[builder(default)]
    third: f32,
    // this depends on the type of field second nontrivially
    forth: T::AssocType,
}

/// dummy trait for enforcing more complicated relationships
trait MyTraitX {
    type AssocType: Clone;
}

impl MyTraitX for f32 {
    type AssocType = i32;
}

impl MyTraitX for i32 {
    type AssocType = f32;
}

impl MyTraitX for &str {
    type AssocType = usize;
}

pub struct AssignedX<T: ?Sized>(T);
#[derive(Default)]
//@note this could be the unit type directly
pub struct EmptyX;

// this might not look necessary at first but it allows us to introduce the
// generic arguments bit by bit.
//@note this could be phantom data directly
pub struct PlaceholderX<T>(PhantomData<T>);

impl<T> Default for PlaceholderX<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> AssignableX<T> for PlaceholderX<T> {
    fn assign(self, t: T) -> AssignedX<T> {
        AssignedX(t)
    }
}

pub struct WithDefaultX<T>(T);

trait HasValueX {
    type ValueType: ?Sized;
    fn value(self) -> Self::ValueType;
}

trait AssignableX<T> {
    fn assign(self, t: T) -> AssignedX<T>;
}

impl<T> AssignableX<T> for EmptyX {
    fn assign(self, t: T) -> AssignedX<T> {
        AssignedX(t)
    }
}

impl<T> AssignableX<T> for WithDefaultX<T> {
    fn assign(self, t: T) -> AssignedX<T> {
        AssignedX(t)
    }
}

impl<T> HasValueX for AssignedX<T> {
    type ValueType = T;

    fn value(self) -> Self::ValueType {
        self.0
    }
}

impl<T> HasValueX for WithDefaultX<T> {
    type ValueType = T;

    fn value(self) -> Self::ValueType {
        self.0
    }
}

struct PodBuilder2<State> {
    state: State,
}

impl PodBuilder2<(EmptyX, EmptyX, WithDefaultX<f32>, EmptyX)> {
    pub fn new() -> Self {
        Self {
            state: (
                EmptyX::default(),
                EmptyX::default(),
                // @note(georgios) we need a closure in here that sets the default value
                // but that's no problem :)
                //
                WithDefaultX(Default::default()),
                EmptyX::default(),
            ),
        }
    }
}

impl<U, V, W, X> PodBuilder2<(U, V, W, X)> {
    // @note(geo) we can even define an #[into] attribute that changes the signature
    // of the builder function here from String to impl Into<String> (also possible in general)
    // pub fn first(self, first: String) -> PodBuilder2<'a, T, (Assigned<String>, V, W)> {
    pub fn first<S: std::fmt::Display>(self, first: S) -> PodBuilder2<(AssignedX<S>, V, W, X)>
    where
        U: AssignableX<S>,
    {
        let state = (
            self.state.0.assign(first),
            self.state.1,
            self.state.2,
            self.state.3,
        );
        PodBuilder2 { state }
    }

    pub fn second<'a, T>(self, second: &'a T) -> PodBuilder2<(U, AssignedX<&'a T>, W, X)>
    where
        V: AssignableX<&'a T>,
        T: std::fmt::Debug + MyTraitX,
    {
        let state = (
            self.state.0,
            self.state.1.assign(second),
            self.state.2,
            self.state.3,
        );
        PodBuilder2 { state }
    }
    pub fn third(self, third: f32) -> PodBuilder2<(U, V, AssignedX<f32>, X)>
    where
        W: AssignableX<f32>,
    {
        let state = (
            self.state.0,
            self.state.1,
            self.state.2.assign(third),
            self.state.3,
        );
        PodBuilder2 { state }
    }
}
//@note(geo) this is conceivable, but not really useful. We should enforce a build
// order in these cases, where the generics that this guy depends on have already
// been set.
// // for types that have complex dependencies on the other types we have to
// // make up two impl blocks. One in case the type that they are dependent on was
// // already assigned and one where it was not
// impl<V, W, X> PodBuilder2<(Empty, V, W, X)> {
//     pub fn forth<'a, T: MyTrait + Debug>(
//         self,
//         forth: T::AssocType,
//     ) -> PodBuilder2<(Placeholder<&'a T>, V, W, Assigned<T::AssocType>)>
//     where
//         X: Assignable<T::AssocType>,
//     {
//         let state = (
//             Placeholder::default(),
//             self.state.1,
//             self.state.2,
//             self.state.3.assign(forth),
//         );
//         PodBuilder2 { state }
//     }
// }

impl<'a, T: std::fmt::Debug + MyTraitX, U, W, X> PodBuilder2<(U, AssignedX<&'a T>, W, X)> {
    pub fn forth(
        self,
        forth: T::AssocType,
    ) -> PodBuilder2<(U, AssignedX<&'a T>, W, AssignedX<T::AssocType>)>
    where
        X: AssignableX<T::AssocType>,
    {
        let state = (
            self.state.0,
            self.state.1,
            self.state.2,
            self.state.3.assign(forth),
        );
        PodBuilder2 { state }
    }
}

impl<'a, S, T, U, V, W, X> PodBuilder2<(U, V, W, X)>
where
    U: HasValueX<ValueType = S>,
    V: HasValueX<ValueType = &'a T>,
    W: HasValueX<ValueType = f32>,
    X: HasValueX<ValueType = T::AssocType>,
    T: std::fmt::Debug + 'a + MyTraitX,
    S: std::fmt::Display,
{
    pub fn build(self) -> Pod<'a, S, T> {
        Pod {
            first: self.state.0.value(),
            second: self.state.1.value(),
            third: self.state.2.value(),
            forth: self.state.3.value(),
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

    // let pod = PodBuilder2::new().first("adda").second(&2.).build();
    // let pod = PodBuilder2::new()
    //     .second(&1)
    //     .first(String::from("abc"))
    //     .build();
    // let pod = PodBuilder2::new()
    //     .first("123")
    //     .third(3.)
    //     .second(&"hi")
    //     .build();

    let stemcell = PodBuilder2::new().first("hi").third(1337.);

    let arg_count = std::env::args().count();

    if arg_count > 3 {
        let pod = stemcell.second(&1i32).forth(1f32); //.build(); //.build(); //.build();:
    } else {
        let pod = stemcell.second(&"string").forth(1).build();
        // let pod = stemcell.second(&"hi").build();
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
