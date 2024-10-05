#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use typed_builder::TypedBuilder;
struct Pod {
    x: u32,
    s: u32,
}
#[automatically_derived]
impl Pod {
    /**
                Create a builder for building `Pod`.
                On the builder, call `.x(...)`, `.s(...)` to set the values of the fields.
                Finally, call `.build()` to create the instance of `Pod`.
                */
    #[allow(dead_code, clippy::default_trait_access)]
    fn builder() -> PodBuilder<((), ())> {
        PodBuilder {
            fields: ((), ()),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
struct PodBuilder<TypedBuilderFields = ((), ())> {
    fields: TypedBuilderFields,
    phantom: ::core::marker::PhantomData<()>,
}
#[automatically_derived]
impl<TypedBuilderFields> Clone for PodBuilder<TypedBuilderFields>
where
    TypedBuilderFields: Clone,
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
impl<__s> PodBuilder<((), __s)> {
    #[allow(clippy::used_underscore_binding, clippy::no_effect_underscore_binding)]
    pub fn x(self, x: u32) -> PodBuilder<((u32,), __s)> {
        let x = (x,);
        let ((), s) = self.fields;
        PodBuilder {
            fields: (x, s),
            phantom: self.phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
#[allow(clippy::exhaustive_enums)]
pub enum PodBuilder_Error_Repeated_field_x {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl<__s> PodBuilder<((u32,), __s)> {
    #[deprecated(note = "Repeated field x")]
    pub fn x(self, _: PodBuilder_Error_Repeated_field_x) -> PodBuilder<((u32,), __s)> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl<__x> PodBuilder<(__x, ())> {
    #[allow(clippy::used_underscore_binding, clippy::no_effect_underscore_binding)]
    pub fn s(self, s: u32) -> PodBuilder<(__x, (u32,))> {
        let s = (s,);
        let (x, ()) = self.fields;
        PodBuilder {
            fields: (x, s),
            phantom: self.phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
#[allow(clippy::exhaustive_enums)]
pub enum PodBuilder_Error_Repeated_field_s {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl<__x> PodBuilder<(__x, (u32,))> {
    #[deprecated(note = "Repeated field s")]
    pub fn s(self, _: PodBuilder_Error_Repeated_field_s) -> PodBuilder<(__x, (u32,))> {
        self
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
#[allow(clippy::exhaustive_enums)]
pub enum PodBuilder_Error_Missing_required_field_x {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
#[automatically_derived]
impl<__s> PodBuilder<((), __s)> {
    #[deprecated(note = "Missing required field x")]
    pub fn build(self, _: PodBuilder_Error_Missing_required_field_x) -> ! {
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
pub enum PodBuilder_Error_Missing_required_field_s {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
#[automatically_derived]
impl PodBuilder<((u32,), ())> {
    #[deprecated(note = "Missing required field s")]
    pub fn build(self, _: PodBuilder_Error_Missing_required_field_s) -> ! {
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
impl PodBuilder<((u32,), (u32,))> {
    #[allow(
        clippy::default_trait_access,
        clippy::used_underscore_binding,
        clippy::no_effect_underscore_binding
    )]
    pub fn build(self) -> Pod {
        let (x, s) = self.fields;
        let x = x.0;
        let s = s.0;
        #[allow(deprecated)] Pod { x, s }.into()
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Pod {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Pod",
            "x",
            &self.x,
            "s",
            &&self.s,
        )
    }
}
fn main() {
    let pod = Pod::builder().x(1).s(2).build();
    {
        ::std::io::_print(format_args!("{0:?}\n", pod));
    };
}
