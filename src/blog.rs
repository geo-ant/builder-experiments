#![allow(unused)]

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

struct Pod2<'a, S, T>
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

struct Pod2Builder<F1, F2, F3, F4, F5> {
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

impl Pod2Builder<Empty, Empty, Empty, Empty, Empty> {
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

impl<F1, F2, F3, F4, F5> Pod2Builder<F1, F2, F3, F4, F5> {
    fn field1(self, field1: f32) -> Pod2Builder<Assigned<f32>, F2, F3, F4, F5>
    where
        F1: Assignable<f32>,
    {
        Pod2Builder {
            field1: Assigned(field1),
            field2: self.field2,
            field3: self.field3,
            field4: self.field4,
            field5: self.field5,
        }
    }

    fn field2<S>(self, field2: S) -> Pod2Builder<F1, Assigned<S>, F3, F4, F5>
    where
        S: Display,
        F2: Assignable<S>,
    {
        Pod2Builder {
            field1: self.field1,
            field2: Assigned(field2),
            field3: self.field3,
            field4: self.field4,
            field5: self.field5,
        }
    }

    fn build<'a, S, T>(self) -> Pod2<'a, S, T>
    where
        T: 'a + Debug + MyTrait,
        S: std::fmt::Display,
        F1: HasValue<ValueType = f32>,
        F2: HasValue<ValueType = S>,
        F3: HasValue<ValueType = &'a T>,
        F4: HasValue<ValueType = T::AssocType>,
        F5: HasValue<ValueType = Option<T>>,
    {
        Pod2 {
            field1: self.field1.value(),
            field2: self.field2.value(),
            field3: self.field3.value(),
            field4: self.field4.value(),
            field5: self.field5.value(),
        }
    }
}

/// Assigning field 3: if field 5 is not set
impl<F1, F2, F4> Pod2Builder<F1, F2, Empty, F4, Empty> {
    fn field3<'a, T>(self, field3: &'a T) -> Pod2Builder<F1, F2, Assigned<&'a T>, F4, Empty>
    where
        T: 'a + Debug + MyTrait,
    {
        Pod2Builder {
            field1: self.field1,
            field2: self.field2,
            field3: Assigned(field3),
            field4: self.field4,
            field5: self.field5,
        }
    }
}

/// Assigning field 3: if field 5 is set.
impl<'a, F1, F2, F4, T> Pod2Builder<F1, F2, Empty, F4, Assigned<Option<T>>>
where
    T: 'a + Debug + MyTrait,
{
    fn field3(
        self,
        field3: &'a T,
    ) -> Pod2Builder<F1, F2, Assigned<&'a T>, F4, Assigned<Option<T>>> {
        Pod2Builder {
            field1: self.field1,
            field2: self.field2,
            field3: Assigned(field3),
            field4: self.field4,
            field5: self.field5,
        }
    }
}

/// assigning field 5 if field 3 is set
impl<'a, T, F1, F2, F4> Pod2Builder<F1, F2, Assigned<&'a T>, F4, Empty>
where
    T: 'a + Debug + MyTrait,
{
    fn field5(
        self,
        field5: Option<T>,
    ) -> Pod2Builder<F1, F2, Assigned<&'a T>, F4, Assigned<Option<T>>> {
        Pod2Builder {
            field1: self.field1,
            field2: self.field2,
            field3: self.field3,
            field4: self.field4,
            field5: Assigned(field5),
        }
    }
}

/// assigning field 5 if field 3 is not set
impl<F1, F2, F4> Pod2Builder<F1, F2, Empty, F4, Empty> {
    fn field5<T>(self, field5: Option<T>) -> Pod2Builder<F1, F2, Empty, F4, Assigned<Option<T>>>
    where
        T: Debug + MyTrait,
    {
        Pod2Builder {
            field1: self.field1,
            field2: self.field2,
            field3: Empty,
            field4: self.field4,
            field5: Assigned(field5),
        }
    }
}

/// Assigning field 4
/// requires either field 3 or field 5 to be set so that we can deduce the type
/// this is if field 3 is set (regardless of the other fields)
// @note(geo) although unintuitive it might be better in more complicated cases
// to explicitly work through all possible combinatorial combinations to not
// run into duplicate implementations.
impl<'a, T, F1, F2, F5> Pod2Builder<F1, F2, Assigned<&'a T>, Empty, F5>
where
    T: 'a + Debug + MyTrait,
{
    fn field4(
        self,
        field4: T::AssocType,
    ) -> Pod2Builder<F1, F2, Assigned<&'a T>, Assigned<T::AssocType>, F5> {
        Pod2Builder {
            field1: self.field1,
            field2: self.field2,
            field3: self.field3,
            field4: Assigned(field4),
            field5: self.field5,
        }
    }
}

// assigning field 4, if field 3 is not set but field 5 is.
impl<'a, T, F1, F2> Pod2Builder<F1, F2, Empty, Empty, Assigned<Option<T>>>
where
    T: 'a + Debug + MyTrait,
{
    fn field4(
        self,
        field4: T::AssocType,
    ) -> Pod2Builder<F1, F2, Empty, Assigned<T::AssocType>, Assigned<Option<T>>> {
        Pod2Builder {
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
    foo(true);
    foo(false);
}

impl MyTrait for i32 {
    type AssocType = f32;
}

impl MyTrait for String {
    type AssocType = usize;
}

fn foo(cond: bool) {
    let builder = Pod2Builder::new().field1(1.);
    if cond {
        let builder = builder.field3(&1i32).field2("foo");
    } else {
        let builder = builder.field2(1).field3(&String::from("hi"));
    }
}
