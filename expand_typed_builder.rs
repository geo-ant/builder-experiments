#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use bon::Builder as BonBuilder;
use buildstructor::Builder as Buildstructor;
use derive_builder::Builder as DeriveBuilder;
use std::{fmt::Debug, marker::PhantomData, str::FromStr};
use typed_builder::TypedBuilder;
struct Pod<'a, S, T: ?Sized>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    first: S,
    second: &'a T,
    #[builder(default)]
    third: f32,
}
#[automatically_derived]
impl<'a, S, T: ?Sized> Pod<'a, S, T>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    /**
                Create a builder for building `Pod`.
                On the builder, call `.first(...)`, `.second(...)`, `.third(...)`(optional) to set the values of the fields.
                Finally, call `.build()` to create the instance of `Pod`.
                */
    #[allow(dead_code, clippy::default_trait_access)]
    fn builder() -> PodBuilder<'a, S, T, ((), (), ())> {
        PodBuilder {
            fields: ((), (), ()),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
struct PodBuilder<'a, S, T: ?Sized, TypedBuilderFields = ((), (), ())>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    fields: TypedBuilderFields,
    phantom: ::core::marker::PhantomData<
        (
            ::core::marker::PhantomData<&'a ()>,
            ::core::marker::PhantomData<S>,
            ::core::marker::PhantomData<T>,
        ),
    >,
}
#[automatically_derived]
impl<'a, S, T: ?Sized, TypedBuilderFields> Clone
for PodBuilder<'a, S, T, TypedBuilderFields>
where
    TypedBuilderFields: Clone,
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    #[allow(clippy::default_trait_access)]
    fn clone(&self) -> Self {
        Self {
            fields: self.fields.clone(),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl<'a, S, T: ?Sized, __second, __third> PodBuilder<'a, S, T, ((), __second, __third)>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    #[allow(clippy::used_underscore_binding, clippy::no_effect_underscore_binding)]
    pub fn first(self, first: S) -> PodBuilder<'a, S, T, ((S,), __second, __third)> {
        let first = (first,);
        let ((), second, third) = self.fields;
        PodBuilder {
            fields: (first, second, third),
            phantom: self.phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
#[allow(clippy::exhaustive_enums)]
pub enum PodBuilder_Error_Repeated_field_first {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl<'a, S, T: ?Sized, __second, __third> PodBuilder<'a, S, T, ((S,), __second, __third)>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    #[deprecated(note = "Repeated field first")]
    pub fn first(
        self,
        _: PodBuilder_Error_Repeated_field_first,
    ) -> PodBuilder<'a, S, T, ((S,), __second, __third)> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl<'a, S, T: ?Sized, __first, __third> PodBuilder<'a, S, T, (__first, (), __third)>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    #[allow(clippy::used_underscore_binding, clippy::no_effect_underscore_binding)]
    pub fn second(
        self,
        second: &'a T,
    ) -> PodBuilder<'a, S, T, (__first, (&'a T,), __third)> {
        let second = (second,);
        let (first, (), third) = self.fields;
        PodBuilder {
            fields: (first, second, third),
            phantom: self.phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
#[allow(clippy::exhaustive_enums)]
pub enum PodBuilder_Error_Repeated_field_second {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl<
    'a,
    S,
    T: ?Sized,
    __first,
    __third,
> PodBuilder<'a, S, T, (__first, (&'a T,), __third)>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    #[deprecated(note = "Repeated field second")]
    pub fn second(
        self,
        _: PodBuilder_Error_Repeated_field_second,
    ) -> PodBuilder<'a, S, T, (__first, (&'a T,), __third)> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl<'a, S, T: ?Sized, __first, __second> PodBuilder<'a, S, T, (__first, __second, ())>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    #[allow(clippy::used_underscore_binding, clippy::no_effect_underscore_binding)]
    pub fn third(self, third: f32) -> PodBuilder<'a, S, T, (__first, __second, (f32,))> {
        let third = (third,);
        let (first, second, ()) = self.fields;
        PodBuilder {
            fields: (first, second, third),
            phantom: self.phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
#[allow(clippy::exhaustive_enums)]
pub enum PodBuilder_Error_Repeated_field_third {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl<
    'a,
    S,
    T: ?Sized,
    __first,
    __second,
> PodBuilder<'a, S, T, (__first, __second, (f32,))>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    #[deprecated(note = "Repeated field third")]
    pub fn third(
        self,
        _: PodBuilder_Error_Repeated_field_third,
    ) -> PodBuilder<'a, S, T, (__first, __second, (f32,))> {
        self
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
#[allow(clippy::exhaustive_enums)]
pub enum PodBuilder_Error_Missing_required_field_first {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
#[automatically_derived]
impl<'a, S, T: ?Sized, __second, __third> PodBuilder<'a, S, T, ((), __second, __third)>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    #[deprecated(note = "Missing required field first")]
    pub fn build(self, _: PodBuilder_Error_Missing_required_field_first) -> ! {
        {
            #[cold]
            #[track_caller]
            #[inline(never)]
            const fn panic_cold_explicit() -> ! {
                ::core::panicking::panic_explicit()
            }
            panic_cold_explicit();
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
#[allow(clippy::exhaustive_enums)]
pub enum PodBuilder_Error_Missing_required_field_second {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
#[automatically_derived]
impl<'a, S, T: ?Sized, __third> PodBuilder<'a, S, T, ((S,), (), __third)>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    #[deprecated(note = "Missing required field second")]
    pub fn build(self, _: PodBuilder_Error_Missing_required_field_second) -> ! {
        {
            #[cold]
            #[track_caller]
            #[inline(never)]
            const fn panic_cold_explicit() -> ! {
                ::core::panicking::panic_explicit()
            }
            panic_cold_explicit();
        }
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl<
    'a,
    S,
    T: ?Sized,
    __third: ::typed_builder::Optional<f32>,
> PodBuilder<'a, S, T, ((S,), (&'a T,), __third)>
where
    S: std::fmt::Display,
    T: std::fmt::Debug + MyTrait,
{
    #[allow(
        clippy::default_trait_access,
        clippy::used_underscore_binding,
        clippy::no_effect_underscore_binding
    )]
    pub fn build(self) -> Pod<'a, S, T> {
        let (first, second, third) = self.fields;
        let first = first.0;
        let second = second.0;
        let third = ::typed_builder::Optional::into_value(
            third,
            || ::core::default::Default::default(),
        );
        #[allow(deprecated)]
        Pod::<'a, S, T> {
            first,
            second,
            third,
        }
            .into()
    }
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
impl MyTrait for str {
    type AssocType = usize;
}
pub struct Assigned<T: ?Sized>(T);
pub struct Empty;
#[automatically_derived]
impl ::core::default::Default for Empty {
    #[inline]
    fn default() -> Empty {
        Empty {}
    }
}
pub struct Placeholder<T>(PhantomData<T>);
impl<T> Default for Placeholder<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<T> Assignable<T> for Placeholder<T> {
    fn assign(self, t: T) -> Assigned<T> {
        Assigned(t)
    }
}
pub struct WithDefault<T>(T);
trait AssignedOrDefault {
    type ValueType: ?Sized;
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
impl PodBuilder2<(Empty, Empty, WithDefault<f32>, Empty)> {
    pub fn new() -> Self {
        Self {
            state: (
                Empty::default(),
                Empty::default(),
                WithDefault(Default::default()),
                Empty::default(),
            ),
        }
    }
}
impl<U, V, W, X> PodBuilder2<(U, V, W, X)> {
    pub fn first<S: std::fmt::Display>(
        self,
        first: S,
    ) -> PodBuilder2<(Assigned<S>, V, W, X)>
    where
        U: Assignable<S>,
    {
        let state = (
            self.state.0.assign(first),
            self.state.1,
            self.state.2,
            self.state.3,
        );
        PodBuilder2 { state }
    }
    pub fn second<'a, T>(self, second: &'a T) -> PodBuilder2<(U, Assigned<&'a T>, W, X)>
    where
        V: Assignable<&'a T>,
        T: std::fmt::Debug + MyTrait + ?Sized,
    {
        let state = (
            self.state.0,
            self.state.1.assign(second),
            self.state.2,
            self.state.3,
        );
        PodBuilder2 { state }
    }
    pub fn third(self, third: f32) -> PodBuilder2<(U, V, Assigned<f32>, X)>
    where
        W: Assignable<f32>,
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
impl<
    'a,
    T: ?Sized + std::fmt::Debug + MyTrait,
    U,
    W,
    X,
> PodBuilder2<(U, Assigned<&'a T>, W, X)> {
    pub fn forth(
        self,
        forth: T::AssocType,
    ) -> PodBuilder2<(U, Assigned<&'a T>, W, Assigned<T::AssocType>)>
    where
        X: Assignable<T::AssocType>,
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
    U: AssignedOrDefault<ValueType = S>,
    V: AssignedOrDefault<ValueType = &'a T>,
    W: AssignedOrDefault<ValueType = f32>,
    X: AssignedOrDefault<ValueType = T::AssocType>,
    T: std::fmt::Debug + 'a + MyTrait + ?Sized,
    S: std::fmt::Display,
{
    pub fn build(self) -> Pod<'a, S, T> {
        Pod {
            first: self.state.0.value_or_default(),
            second: self.state.1.value_or_default(),
            third: self.state.2.value_or_default(),
            forth: self.state.3.value_or_default(),
        }
    }
}
fn main() {
    let stemcell = PodBuilder2::new().first("hi").third(1337.);
    let arg_count = std::env::args().count();
    if arg_count > 3 {
        let pod = stemcell.second(&1i32).forth(1f32);
    } else {
        let pod = stemcell.second("string").forth(1).build();
    }
}
