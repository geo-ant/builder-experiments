#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use bon::Builder as BonBuilder;
use std::{marker::PhantomData, str::FromStr};
use typed_builder::TypedBuilder;
struct Pod<'a, S: std::fmt::Display, T: std::fmt::Debug> {
    first: S,
    second: &'a T,
    #[builder(default)]
    third: f32,
}
#[automatically_derived]
impl<
    'a,
    S: ::core::fmt::Debug + std::fmt::Display,
    T: ::core::fmt::Debug + std::fmt::Debug,
> ::core::fmt::Debug for Pod<'a, S, T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Pod",
            "first",
            &self.first,
            "second",
            &self.second,
            "third",
            &&self.third,
        )
    }
}
#[automatically_derived]
impl<'a, S: std::fmt::Display, T: std::fmt::Debug> Pod<'a, S, T> {
    ///Create an instance of [`Pod`] using the builder syntax
    #[inline(always)]
    #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
    fn builder() -> PodBuilder<'a, S, T> {
        PodBuilder {
            __private_phantom: ::core::marker::PhantomData,
            __private_named_members: (
                ::bon::private::Unset(::bon::private::Required),
                ::bon::private::Unset(::bon::private::Required),
                ::bon::private::Unset(::bon::private::Optional),
            ),
        }
    }
}
#[doc(hidden)]
type __PodBuilderInitialState = (
    ::bon::private::Unset<::bon::private::Required>,
    ::bon::private::Unset<::bon::private::Required>,
    ::bon::private::Unset<::bon::private::Optional>,
);
#[must_use = "the builder does nothing until you call `build()` on it to finish building"]
///Use builder syntax to set the required parameters and finish by calling the method [`Self::build()`].
#[allow(unused_parens)]
#[allow(clippy::struct_field_names, clippy::type_complexity)]
struct PodBuilder<
    'a,
    S: std::fmt::Display,
    T: std::fmt::Debug,
    ___State = __PodBuilderInitialState,
> {
    /**Please don't touch this field. It's an implementation detail that is exempt from the API stability guarantees. This field couldn't be hidden using Rust's privacy syntax. The details about this are described in [the blog post](https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).
        */
    __private_phantom: ::core::marker::PhantomData<
        (
            ::core::marker::PhantomData<Pod<'a, S, T>>,
            ::core::marker::PhantomData<S>,
            ::core::marker::PhantomData<&'a T>,
            ::core::marker::PhantomData<f32>,
            ::core::marker::PhantomData<S>,
            ::core::marker::PhantomData<T>,
            ::core::marker::PhantomData<___State>,
        ),
    >,
    /**Please don't touch this field. It's an implementation detail that is exempt from the API stability guarantees. This field couldn't be hidden using Rust's privacy syntax. The details about this are described in [the blog post](https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).
        */
    __private_named_members: ___State,
}
#[allow(non_camel_case_types)]
#[doc(hidden)]
struct PodBuilder__first;
#[allow(non_camel_case_types)]
#[doc(hidden)]
struct PodBuilder__second;
#[allow(non_camel_case_types)]
#[doc(hidden)]
struct PodBuilder__third;
#[allow(unused_parens)]
#[automatically_derived]
impl<
    'a,
    S: std::fmt::Display,
    T: std::fmt::Debug,
    __First,
    __Second,
    __Third,
> PodBuilder<'a, S, T, (__First, __Second, __Third)> {
    ///Finishes building and returns the requested object.
    #[inline(always)]
    #[allow(clippy::inline_always, clippy::future_not_send)]
    #[must_use = "building a struct without using it is likely a bug"]
    fn build(self) -> Pod<'a, S, T>
    where
        __First: ::bon::private::IntoSet<S, PodBuilder__first>,
        __Second: ::bon::private::IntoSet<&'a T, PodBuilder__second>,
        __Third: ::bon::private::IntoSet<Option<f32>, PodBuilder__third>,
    {
        let first: S = ::bon::private::IntoSet::<
            S,
            PodBuilder__first,
        >::into_set(self.__private_named_members.0);
        let second: &'a T = ::bon::private::IntoSet::<
            &'a T,
            PodBuilder__second,
        >::into_set(self.__private_named_members.1);
        let third: f32 = ::bon::private::IntoSet::<
            Option<f32>,
            PodBuilder__third,
        >::into_set(self.__private_named_members.2)
            .unwrap_or_default();
        Pod { first, second, third }
    }
    ///Sets the value of `first`. See [`Pod::builder()`] for more info.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    fn first(
        self,
        value: S,
    ) -> PodBuilder<'a, S, T, (::bon::private::Set<S>, __Second, __Third)>
    where
        __First: ::bon::private::IsUnset,
    {
        PodBuilder {
            __private_phantom: ::core::marker::PhantomData,
            __private_named_members: (
                ::bon::private::Set(value),
                self.__private_named_members.1,
                self.__private_named_members.2,
            ),
        }
    }
    ///Sets the value of `second`. See [`Pod::builder()`] for more info.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    fn second(
        self,
        value: &'a T,
    ) -> PodBuilder<'a, S, T, (__First, ::bon::private::Set<&'a T>, __Third)>
    where
        __Second: ::bon::private::IsUnset,
    {
        PodBuilder {
            __private_phantom: ::core::marker::PhantomData,
            __private_named_members: (
                self.__private_named_members.0,
                ::bon::private::Set(value),
                self.__private_named_members.2,
            ),
        }
    }
    ///Same as [`Self::third`], but accepts an `Option` as input. See that method's documentation for more details.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    fn maybe_third(
        self,
        value: Option<f32>,
    ) -> PodBuilder<'a, S, T, (__First, __Second, ::bon::private::Set<Option<f32>>)>
    where
        __Third: ::bon::private::IsUnset,
    {
        PodBuilder {
            __private_phantom: ::core::marker::PhantomData,
            __private_named_members: (
                self.__private_named_members.0,
                self.__private_named_members.1,
                ::bon::private::Set(value),
            ),
        }
    }
    ///Sets the value of `third`. See [`Pod::builder()`] for more info.
    #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
    #[inline(always)]
    fn third(
        self,
        value: f32,
    ) -> PodBuilder<'a, S, T, (__First, __Second, ::bon::private::Set<Option<f32>>)>
    where
        __Third: ::bon::private::IsUnset,
    {
        self.maybe_third(Some(value))
    }
}
pub struct Assigned<T>(T);
pub struct Empty;
#[automatically_derived]
impl ::core::default::Default for Empty {
    #[inline]
    fn default() -> Empty {
        Empty {}
    }
}
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
            state: (Empty::default(), Empty::default(), WithDefault(Default::default())),
        }
    }
}
impl<U, V, W> PodBuilder2<(U, V, W)> {
    pub fn first<S: std::fmt::Display>(
        self,
        first: S,
    ) -> PodBuilder2<(Assigned<S>, V, W)>
    where
        U: Assignable<S>,
    {
        let state = (self.state.0.assign(first), self.state.1, self.state.2);
        PodBuilder2 { state }
    }
    pub fn second<'a, T>(self, second: &'a T) -> PodBuilder2<(U, Assigned<&'a T>, W)>
    where
        V: Assignable<&'a T>,
        T: std::fmt::Debug,
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
    T: std::fmt::Debug + 'a,
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
fn main() {
    let pod = PodBuilder2::new().first("adda").second(&2.).build();
    let pod = PodBuilder2::new().second(&1).first(String::from("abc")).build();
    let pod = PodBuilder2::new().first("123").third(3.).second(&"hi").build();
    let stemcell = PodBuilder2::new().first("hi");
    let some_count = std::env::args().count();
    if some_count > 2 {
        let pod = stemcell.second(&1).build();
    } else {
        let pod = stemcell.second(&"hi").build();
    }
}
