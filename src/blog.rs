#![allow(unused)]

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

struct Pod<'a, S, T>
where
    S: Display,
    T: Debug + MyTrait,
{
    field1: f32,
    field2: S,
    field3: &'a T,
    field4: T::AssocType,
    field5: Option<T>,
}

// this is just a bogus
// trait that gives us
// associated types
trait MyTrait {
    type AssocType;
}

struct PodBuilder<F1, F2, F3, F4, F5> {
    field1: F1,
    field2: F2,
    field3: F3,
    field4: F4,
    field5: F5,
}

#[derive(Default)]
struct Empty;
struct Assigned<T>(T);
// struct Placeholder<T>(PhantomData<T>);
trait HasValue {
    type ValueType;
    fn value(self) -> Self::ValueType;
}
// trait IsEmpty {}
trait Assignable<T> {}

impl<T> Assignable<T> for Empty {}
// impl<T> Assignable<T> for Placeholder<T>{}

// impl IsEmpty for Empty {}
impl<T> HasValue for Assigned<T> {
    type ValueType = T;

    fn value(self) -> Self::ValueType {
        self.0
    }
}

impl PodBuilder<Empty, Empty, Empty, Empty, Empty> {
    pub fn new() -> Self {
        Self {
            field1: Default::default(),
            field2: Default::default(),
            field3: Default::default(),
            field4: Default::default(),
            field5: Default::default(),
        }
    }
}

impl<F1, F2, F3, F4, F5> PodBuilder<F1, F2, F3, F4, F5> {
    fn field1(self, field1: f32) -> PodBuilder<Assigned<f32>, F2, F3, F4, F5>
    where
        F1: Assignable<f32>,
    {
        PodBuilder {
            field1: Assigned(field1),
            field2: self.field2,
            field3: self.field3,
            field4: self.field4,
            field5: self.field5,
        }
    }

    fn field2<S>(self, field2: S) -> PodBuilder<F1, Assigned<S>, F3, F4, F5>
    where
        S: Display,
        F2: Assignable<S>,
    {
        PodBuilder {
            field1: self.field1,
            field2: Assigned(field2),
            field3: self.field3,
            field4: self.field4,
            field5: self.field5,
        }
    }

    fn field3<'a, T>(self, field3: &'a T) -> PodBuilder<F1, F2, Assigned<&'a T>, F4, F5>
    where
        T: Debug + MyTrait,
        F3: Assignable<&'a T>,
    {
        PodBuilder {
            field1: self.field1,
            field2: self.field2,
            field3: Assigned(field3),
            field4: self.field4,
            field5: self.field5,
        }
    }

    fn field5<T>(self, field5: Option<T>) -> PodBuilder<F1, F2, F3, F4, Assigned<Option<T>>>
    where
        T: Debug + MyTrait,
        F5: Assignable<Option<T>>,
    {
        PodBuilder {
            field1: self.field1,
            field2: self.field2,
            field3: self.field3,
            field4: self.field4,
            field5: Assigned(field5),
        }
    }

    fn build<'a, S, T>(self) -> Pod<'a, S, T>
    where
        T: Debug + MyTrait,
        S: std::fmt::Display,
        F1: HasValue<ValueType = f32>,
        F2: HasValue<ValueType = S>,
        F3: HasValue<ValueType = &'a T>,
        F4: HasValue<ValueType = T::AssocType>,
        F5: HasValue<ValueType = Option<T>>,
    {
        Pod {
            field1: self.field1.value(),
            field2: self.field2.value(),
            field3: self.field3.value(),
            field4: self.field4.value(),
            field5: self.field5.value(),
        }
    }
}

/// Assigning field 4
/// requires either field 3 or field 5 to be set so that we can deduce the type
/// this is if field 3 is set (regardless of the other fields)
// @note(geo) although unintuitive it might be better in more complicated cases
// to explicitly work through all possible combinatorial combinations to not
// run into duplicate implementations.
impl<'a, T, F1, F2, F5> PodBuilder<F1, F2, Assigned<&'a T>, Empty, F5>
where
    T: Debug + MyTrait,
{
    fn field4(
        self,
        field4: T::AssocType,
    ) -> PodBuilder<F1, F2, Assigned<&'a T>, Assigned<T::AssocType>, F5> {
        PodBuilder {
            field1: self.field1,
            field2: self.field2,
            field3: self.field3,
            field4: Assigned(field4),
            field5: self.field5,
        }
    }
}

// assigning field 4, if field 3 is not set but field 5 is.
impl<'a, T, F1, F2> PodBuilder<F1, F2, Empty, Empty, Assigned<Option<T>>>
where
    T: Debug + MyTrait,
{
    fn field4(
        self,
        field4: T::AssocType,
    ) -> PodBuilder<F1, F2, Empty, Assigned<T::AssocType>, Assigned<Option<T>>> {
        PodBuilder {
            field1: self.field1,
            field2: self.field2,
            field3: Empty,
            field4: Assigned(field4),
            field5: self.field5,
        }
    }
}

#[test]
fn test_foo() {
    foo(true, true);
    foo(true, false);
    foo(false, true);
    foo(false, false);
}

impl MyTrait for i32 {
    type AssocType = f32;
}

impl MyTrait for String {
    type AssocType = usize;
}

fn foo(cond: bool, othercond: bool) {
    let s = String::new();
    let builder = PodBuilder::new().field1(1.);
    let pod = PodBuilder::new()
        .field2(1f64)
        .field1(337.)
        .field5(Some("hi".into()))
        .field3(&String::new())
        .field4(1)
        .build();
    if cond {
        let builder = builder
            // .field3(&1i32)
            .field2("foo");
        // .field4(1)
        // .build();
        if othercond {
            let builder = builder.field5(None).field4(1).field3(&s).build();
        } else {
            let builder = builder.field5(None).field4(1.).field3(&1).build();
        }
    } else {
        let builder = builder
            .field3(&s)
            .field5(Some("hi".into()))
            .field2(1)
            .field4(2)
            .build();
    }
}
