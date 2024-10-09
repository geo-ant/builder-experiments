mod crosschecks {
    pub struct Foo<S, T> {
        first: S,
        second: T,
    }
    impl<S, T> Foo<S, T> {
        fn builder() -> FooBuilder {
            FooBuilder {}
        }
    }
    struct FooBuilder {}
    pub struct FooTypedBuilder<S, T> {
        first: S,
        second: T,
    }
    #[automatically_derived]
    impl<S, T> FooTypedBuilder<S, T> {
        /**
                Create a builder for building `FooTypedBuilder`.
                On the builder, call `.first(...)`, `.second(...)` to set the values of the fields.
                Finally, call `.build()` to create the instance of `FooTypedBuilder`.
                */
        #[allow(dead_code, clippy::default_trait_access)]
        pub fn builder() -> FooTypedBuilderBuilder<S, T, ((), ())> {
            FooTypedBuilderBuilder {
                fields: ((), ()),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub struct FooTypedBuilderBuilder<S, T, TypedBuilderFields = ((), ())> {
        fields: TypedBuilderFields,
        phantom: ::core::marker::PhantomData<
            (::core::marker::PhantomData<S>, ::core::marker::PhantomData<T>),
        >,
    }
    #[automatically_derived]
    impl<S, T, TypedBuilderFields> Clone
    for FooTypedBuilderBuilder<S, T, TypedBuilderFields>
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
    impl<S, T, __second> FooTypedBuilderBuilder<S, T, ((), __second)> {
        #[allow(clippy::used_underscore_binding, clippy::no_effect_underscore_binding)]
        pub fn first(self, first: S) -> FooTypedBuilderBuilder<S, T, ((S,), __second)> {
            let first = (first,);
            let ((), second) = self.fields;
            FooTypedBuilderBuilder {
                fields: (first, second),
                phantom: self.phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[allow(clippy::exhaustive_enums)]
    pub enum FooTypedBuilderBuilder_Error_Repeated_field_first {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl<S, T, __second> FooTypedBuilderBuilder<S, T, ((S,), __second)> {
        #[deprecated(note = "Repeated field first")]
        pub fn first(
            self,
            _: FooTypedBuilderBuilder_Error_Repeated_field_first,
        ) -> FooTypedBuilderBuilder<S, T, ((S,), __second)> {
            self
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl<S, T, __first> FooTypedBuilderBuilder<S, T, (__first, ())> {
        #[allow(clippy::used_underscore_binding, clippy::no_effect_underscore_binding)]
        pub fn second(self, second: T) -> FooTypedBuilderBuilder<S, T, (__first, (T,))> {
            let second = (second,);
            let (first, ()) = self.fields;
            FooTypedBuilderBuilder {
                fields: (first, second),
                phantom: self.phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[allow(clippy::exhaustive_enums)]
    pub enum FooTypedBuilderBuilder_Error_Repeated_field_second {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    #[automatically_derived]
    impl<S, T, __first> FooTypedBuilderBuilder<S, T, (__first, (T,))> {
        #[deprecated(note = "Repeated field second")]
        pub fn second(
            self,
            _: FooTypedBuilderBuilder_Error_Repeated_field_second,
        ) -> FooTypedBuilderBuilder<S, T, (__first, (T,))> {
            self
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    #[allow(clippy::exhaustive_enums)]
    pub enum FooTypedBuilderBuilder_Error_Missing_required_field_first {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    #[automatically_derived]
    impl<S, T, __second> FooTypedBuilderBuilder<S, T, ((), __second)> {
        #[deprecated(note = "Missing required field first")]
        pub fn build(
            self,
            _: FooTypedBuilderBuilder_Error_Missing_required_field_first,
        ) -> ! {
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
    pub enum FooTypedBuilderBuilder_Error_Missing_required_field_second {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    #[automatically_derived]
    impl<S, T> FooTypedBuilderBuilder<S, T, ((S,), ())> {
        #[deprecated(note = "Missing required field second")]
        pub fn build(
            self,
            _: FooTypedBuilderBuilder_Error_Missing_required_field_second,
        ) -> ! {
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
    impl<S, T> FooTypedBuilderBuilder<S, T, ((S,), (T,))> {
        #[allow(
            clippy::default_trait_access,
            clippy::used_underscore_binding,
            clippy::no_effect_underscore_binding
        )]
        pub fn build(self) -> FooTypedBuilder<S, T> {
            let (first, second) = self.fields;
            let first = first.0;
            let second = second.0;
            #[allow(deprecated)]
            FooTypedBuilder::<S, T> {
                first,
                second,
            }
                .into()
        }
    }
    pub struct FooBon<S, T> {
        first: S,
        second: T,
    }
    #[automatically_derived]
    impl<S, T> FooBon<S, T> {
        ///Create an instance of [`FooBon`] using the builder syntax
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
        pub fn builder() -> FooBonBuilder<S, T> {
            FooBonBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_named_members: (
                    ::bon::private::Unset(::bon::private::Required),
                    ::bon::private::Unset(::bon::private::Required),
                ),
            }
        }
    }
    #[doc(hidden)]
    pub type __FooBonBuilderInitialState = (
        ::bon::private::Unset<::bon::private::Required>,
        ::bon::private::Unset<::bon::private::Required>,
    );
    #[must_use = "the builder does nothing until you call `build()` on it to finish building"]
    ///Use builder syntax to set the required parameters and finish by calling the method [`Self::build()`].
    #[allow(unused_parens)]
    #[allow(clippy::struct_field_names, clippy::type_complexity)]
    pub struct FooBonBuilder<S, T, ___State = __FooBonBuilderInitialState> {
        /**Please don't touch this field. It's an implementation detail that is exempt from the API stability guarantees. This field couldn't be hidden using Rust's privacy syntax. The details about this are described in [the blog post](https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).
        */
        __private_phantom: ::core::marker::PhantomData<
            (
                ::core::marker::PhantomData<FooBon<S, T>>,
                ::core::marker::PhantomData<S>,
                ::core::marker::PhantomData<T>,
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
    pub struct FooBonBuilder__first;
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub struct FooBonBuilder__second;
    #[allow(unused_parens)]
    #[automatically_derived]
    impl<S, T, __First, __Second> FooBonBuilder<S, T, (__First, __Second)> {
        ///Finishes building and returns the requested object.
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::future_not_send)]
        #[must_use = "building a struct without using it is likely a bug"]
        pub fn build(self) -> FooBon<S, T>
        where
            __First: ::bon::private::IntoSet<S, FooBonBuilder__first>,
            __Second: ::bon::private::IntoSet<T, FooBonBuilder__second>,
        {
            let first: S = ::bon::private::IntoSet::<
                S,
                FooBonBuilder__first,
            >::into_set(self.__private_named_members.0);
            let second: T = ::bon::private::IntoSet::<
                T,
                FooBonBuilder__second,
            >::into_set(self.__private_named_members.1);
            FooBon { first, second }
        }
        ///Sets the value of `first`. See [`FooBon::builder()`] for more info.
        #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
        #[inline(always)]
        pub fn first(
            self,
            value: S,
        ) -> FooBonBuilder<S, T, (::bon::private::Set<S>, __Second)>
        where
            __First: ::bon::private::IsUnset,
        {
            FooBonBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_named_members: (
                    ::bon::private::Set(value),
                    self.__private_named_members.1,
                ),
            }
        }
        ///Sets the value of `second`. See [`FooBon::builder()`] for more info.
        #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
        #[inline(always)]
        pub fn second(
            self,
            value: T,
        ) -> FooBonBuilder<S, T, (__First, ::bon::private::Set<T>)>
        where
            __Second: ::bon::private::IsUnset,
        {
            FooBonBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_named_members: (
                    self.__private_named_members.0,
                    ::bon::private::Set(value),
                ),
            }
        }
    }
    pub struct FooBuildstructor<S, T> {
        first: S,
        second: T,
    }
    impl<S, T> FooBuildstructor<S, T> {
        /**Create a new FooBuildstructor

 # Arguments

* `first`:
* `second`: */
        fn new(first: S, second: T) -> FooBuildstructor<S, T> {
            Self { first, second }
        }
    }
    impl<S, T> FooBuildstructor<S, T> {
        /**Create a new FooBuildstructor

 # Arguments

* `first`:
* `second`: */
        pub fn builder() -> NewFooBuildstructorBuilder<S, T> {
            __foobuildstructor_new_builder::new()
        }
    }
    ///Autogenerated by buildstructor
    #[allow(type_alias_bounds)]
    pub type NewFooBuildstructorBuilder<S, T> = __foobuildstructor_new_builder::__FooBuildstructorBuilder<
        (
            __foobuildstructor_new_builder::__Required<S>,
            __foobuildstructor_new_builder::__Required<T>,
        ),
        S,
        T,
    >;
    mod __foobuildstructor_new_builder {
        use super::*;
        #[inline(always)]
        pub fn new<S, T>() -> __FooBuildstructorBuilder<
            (
                __foobuildstructor_new_builder::__Required<S>,
                __foobuildstructor_new_builder::__Required<T>,
            ),
            S,
            T,
        > {
            __FooBuildstructorBuilder {
                fields: (__required(), __required()),
                _phantom: core::default::Default::default(),
            }
        }
        pub struct __Required<T> {
            _uninit: std::mem::MaybeUninit<T>,
        }
        pub struct __Optional<T> {
            lazy: Option<T>,
        }
        pub struct __Set<T> {
            value: T,
        }
        #[inline(always)]
        fn __set<T>(value: T) -> __Set<T> {
            __Set { value }
        }
        #[inline(always)]
        fn __required<T>() -> __Required<T> {
            __Required::<T> {
                _uninit: std::mem::MaybeUninit::uninit(),
            }
        }
        #[inline(always)]
        fn __optional<T>() -> __Optional<T> {
            __Optional::<T> { lazy: None }
        }
        impl<T: Default> From<__Optional<T>> for __Set<T> {
            #[inline(always)]
            fn from(o: __Optional<T>) -> Self {
                __Set {
                    value: o.lazy.unwrap_or_default(),
                }
            }
        }
        pub struct __FooBuildstructorBuilder<__P, S, T> {
            fields: __P,
            _phantom: core::marker::PhantomData<(S, T)>,
        }
        impl<__1, S, T> __FooBuildstructorBuilder<(__Required<S>, __1), S, T> {
            #[inline(always)]
            pub fn first(
                self,
                first: S,
            ) -> __FooBuildstructorBuilder<(__Set<S>, __1), S, T> {
                let first = first;
                __FooBuildstructorBuilder {
                    fields: (__set(first), self.fields.1),
                    _phantom: core::default::Default::default(),
                }
            }
        }
        impl<__0, S, T> __FooBuildstructorBuilder<(__0, __Required<T>), S, T> {
            #[inline(always)]
            pub fn second(
                self,
                second: T,
            ) -> __FooBuildstructorBuilder<(__0, __Set<T>), S, T> {
                let second = second;
                __FooBuildstructorBuilder {
                    fields: (self.fields.0, __set(second)),
                    _phantom: core::default::Default::default(),
                }
            }
        }
        impl<
            S,
            T,
            __P0: Into<__Set<S>>,
            __P1: Into<__Set<T>>,
        > __FooBuildstructorBuilder<(__P0, __P1), S, T> {
            #[inline(always)]
            pub fn build(self) -> FooBuildstructor<S, T> {
                FooBuildstructor::new(
                    self.fields.0.into().value,
                    self.fields.1.into().value,
                )
            }
        }
    }
    use const_typed_builder::Builder;
    pub struct FooConstTyped<S, T> {
        first: S,
        second: T,
    }
    impl<S, T> Builder for FooConstTyped<S, T> {
        type BuilderImpl = FooConstTypedBuilder<S, T, false, false>;
        ///Creates an instance of [`FooConstTypedBuilder`]
        fn builder() -> Self::BuilderImpl {
            Self::BuilderImpl::new()
        }
    }
    ///Builder for [`FooConstTyped`] derived using the `const_typed_builder` crate
    pub struct FooConstTypedBuilder<
        S,
        T,
        const __BUILDER_CONST0: bool,
        const __BUILDER_CONST1: bool,
    > {
        __foo_const_typed_data: FooConstTypedData<S, T>,
    }
    impl<S, T> FooConstTypedBuilder<S, T, false, false> {
        ///Creates a new [`FooConstTypedBuilder`] without any fields initialized
        pub fn new() -> FooConstTypedBuilder<S, T, false, false> {
            Self::default()
        }
    }
    impl<S, T> Default for FooConstTypedBuilder<S, T, false, false> {
        fn default() -> Self {
            FooConstTypedBuilder {
                __foo_const_typed_data: FooConstTypedData::default(),
            }
        }
    }
    impl<
        S,
        T,
        const __BUILDER_CONST1: bool,
    > FooConstTypedBuilder<S, T, false, __BUILDER_CONST1> {
        /**
Setter for the [`FooConstTyped::first`] field.

# Arguments

- `first`: field to be set

# Returns

`Self` with `first` initialized*/
        pub fn first(
            self,
            first: S,
        ) -> FooConstTypedBuilder<S, T, true, __BUILDER_CONST1> {
            let mut __foo_const_typed_data = self.__foo_const_typed_data;
            __foo_const_typed_data.first = Some(first);
            FooConstTypedBuilder {
                __foo_const_typed_data,
            }
        }
    }
    impl<
        S,
        T,
        const __BUILDER_CONST0: bool,
    > FooConstTypedBuilder<S, T, __BUILDER_CONST0, false> {
        /**
Setter for the [`FooConstTyped::second`] field.

# Arguments

- `second`: field to be set

# Returns

`Self` with `second` initialized*/
        pub fn second(
            self,
            second: T,
        ) -> FooConstTypedBuilder<S, T, __BUILDER_CONST0, true> {
            let mut __foo_const_typed_data = self.__foo_const_typed_data;
            __foo_const_typed_data.second = Some(second);
            FooConstTypedBuilder {
                __foo_const_typed_data,
            }
        }
    }
    impl<S, T> FooConstTypedBuilder<S, T, true, true> {
        ///Build an instance of [`FooConstTyped`], consuming the [`FooConstTypedBuilder`]
        pub fn build(self) -> FooConstTyped<S, T> {
            self.__foo_const_typed_data.into()
        }
    }
    #[doc(hidden)]
    pub struct FooConstTypedData<S, T> {
        pub first: Option<S>,
        pub second: Option<T>,
    }
    impl<S, T> From<FooConstTypedData<S, T>> for FooConstTyped<S, T> {
        #[doc(hidden)]
        fn from(data: FooConstTypedData<S, T>) -> FooConstTyped<S, T> {
            FooConstTyped {
                first: data.first.unwrap(),
                second: data.second.unwrap(),
            }
        }
    }
    impl<S, T> Default for FooConstTypedData<S, T> {
        #[doc(hidden)]
        fn default() -> Self {
            FooConstTypedData {
                first: None,
                second: None,
            }
        }
    }
    #[builder(pattern = "owned")]
    pub struct RuntimeFoo<S, T> {
        first: S,
        second: T,
    }
    #[allow(clippy::all)]
    /**Builder for [`RuntimeFoo`](struct.RuntimeFoo.html).
*/
    pub struct RuntimeFooBuilder<S, T> {
        first: ::derive_builder::export::core::option::Option<S>,
        second: ::derive_builder::export::core::option::Option<T>,
    }
    #[allow(clippy::all)]
    #[allow(dead_code)]
    impl<S, T> RuntimeFooBuilder<S, T> {
        #[allow(unused_mut)]
        pub fn first(self, value: S) -> Self {
            let mut new = self;
            new.first = ::derive_builder::export::core::option::Option::Some(value);
            new
        }
        #[allow(unused_mut)]
        pub fn second(self, value: T) -> Self {
            let mut new = self;
            new.second = ::derive_builder::export::core::option::Option::Some(value);
            new
        }
        /**Builds a new `RuntimeFoo`.

# Errors

If a required field has not been initialized.
*/
        pub fn build(
            self,
        ) -> ::derive_builder::export::core::result::Result<
            RuntimeFoo<S, T>,
            RuntimeFooBuilderError,
        > {
            Ok(RuntimeFoo {
                first: match self.first {
                    Some(value) => value,
                    None => {
                        return ::derive_builder::export::core::result::Result::Err(
                            ::derive_builder::export::core::convert::Into::into(
                                ::derive_builder::UninitializedFieldError::from("first"),
                            ),
                        );
                    }
                },
                second: match self.second {
                    Some(value) => value,
                    None => {
                        return ::derive_builder::export::core::result::Result::Err(
                            ::derive_builder::export::core::convert::Into::into(
                                ::derive_builder::UninitializedFieldError::from("second"),
                            ),
                        );
                    }
                },
            })
        }
        /// Create an empty builder, with all fields set to `None` or `PhantomData`.
        fn create_empty() -> Self {
            Self {
                first: ::derive_builder::export::core::default::Default::default(),
                second: ::derive_builder::export::core::default::Default::default(),
            }
        }
    }
    impl<S, T> ::derive_builder::export::core::default::Default
    for RuntimeFooBuilder<S, T> {
        fn default() -> Self {
            Self::create_empty()
        }
    }
    ///Error type for RuntimeFooBuilder
    #[non_exhaustive]
    pub enum RuntimeFooBuilderError {
        /// Uninitialized field
        UninitializedField(&'static str),
        /// Custom validation error
        ValidationError(::derive_builder::export::core::string::String),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RuntimeFooBuilderError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                RuntimeFooBuilderError::UninitializedField(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "UninitializedField",
                        &__self_0,
                    )
                }
                RuntimeFooBuilderError::ValidationError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ValidationError",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl ::derive_builder::export::core::convert::From<
        ::derive_builder::UninitializedFieldError,
    > for RuntimeFooBuilderError {
        fn from(s: ::derive_builder::UninitializedFieldError) -> Self {
            Self::UninitializedField(s.field_name())
        }
    }
    impl ::derive_builder::export::core::convert::From<
        ::derive_builder::export::core::string::String,
    > for RuntimeFooBuilderError {
        fn from(s: ::derive_builder::export::core::string::String) -> Self {
            Self::ValidationError(s)
        }
    }
    impl ::derive_builder::export::core::fmt::Display for RuntimeFooBuilderError {
        fn fmt(
            &self,
            f: &mut ::derive_builder::export::core::fmt::Formatter,
        ) -> ::derive_builder::export::core::fmt::Result {
            match self {
                Self::UninitializedField(ref field) => {
                    f.write_fmt(format_args!("`{0}` must be initialized", field))
                }
                Self::ValidationError(ref error) => {
                    f.write_fmt(format_args!("{0}", error))
                }
            }
        }
    }
    impl std::error::Error for RuntimeFooBuilderError {}
    pub struct TypeStateFoo<S, T> {
        first: S,
        second: T,
    }
    struct TypeStateFooBuilderFirstAdded<S>(S);
    struct TypeStateFooBuilderFirstEmpty;
    struct TypeStateFooBuilderSecondAdded<T>(T);
    struct TypeStateFooBuilderSecondEmpty;
    pub struct TypeStateFooBuilder<First, Second> {
        first: First,
        second: Second,
    }
    impl<S, T> TypeStateFoo<S, T> {
        pub fn builder() -> TypeStateFooBuilder<
            TypeStateFooBuilderFirstEmpty,
            TypeStateFooBuilderSecondEmpty,
        > {
            TypeStateFooBuilder {
                first: TypeStateFooBuilderFirstEmpty,
                second: TypeStateFooBuilderSecondEmpty,
            }
        }
    }
    impl<Second> TypeStateFooBuilder<TypeStateFooBuilderFirstEmpty, Second> {
        pub fn first<S>(
            self,
            first: S,
        ) -> TypeStateFooBuilder<TypeStateFooBuilderFirstAdded<S>, Second> {
            TypeStateFooBuilder {
                first: TypeStateFooBuilderFirstAdded(first),
                second: self.second,
            }
        }
    }
    impl<First> TypeStateFooBuilder<First, TypeStateFooBuilderSecondEmpty> {
        pub fn second<T>(
            self,
            second: T,
        ) -> TypeStateFooBuilder<First, TypeStateFooBuilderSecondAdded<T>> {
            TypeStateFooBuilder {
                first: self.first,
                second: TypeStateFooBuilderSecondAdded(second),
            }
        }
    }
    impl<
        S,
        T,
    > TypeStateFooBuilder<
        TypeStateFooBuilderFirstAdded<S>,
        TypeStateFooBuilderSecondAdded<T>,
    > {
        pub fn build(self) -> TypeStateFoo<S, T> {
            TypeStateFoo {
                first: self.first.0,
                second: self.second.0,
            }
        }
    }
    #[inline(always)]
    #[allow(clippy::inline_always, clippy::use_self, clippy::missing_const_for_fn)]
    pub fn bon_fn<S, T>() -> BonFnBuilder<S, T> {
        BonFnBuilder {
            __private_phantom: ::core::marker::PhantomData,
            __private_named_members: (
                ::bon::private::Unset(::bon::private::Required),
                ::bon::private::Unset(::bon::private::Required),
            ),
        }
    }
    #[doc(hidden)]
    pub type __BonFnBuilderInitialState = (
        ::bon::private::Unset<::bon::private::Required>,
        ::bon::private::Unset<::bon::private::Required>,
    );
    #[must_use = "the builder does nothing until you call `call()` on it to finish building"]
    ///Use builder syntax to set the required parameters and finish by calling the method [`Self::call()`].
    #[allow(unused_parens)]
    #[allow(clippy::struct_field_names, clippy::type_complexity)]
    pub struct BonFnBuilder<S, T, ___State = __BonFnBuilderInitialState> {
        /**Please don't touch this field. It's an implementation detail that is exempt from the API stability guarantees. This field couldn't be hidden using Rust's privacy syntax. The details about this are described in [the blog post](https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust).
        */
        __private_phantom: ::core::marker::PhantomData<
            (
                ::core::marker::PhantomData<S>,
                ::core::marker::PhantomData<T>,
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
    pub struct BonFnBuilder__first;
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub struct BonFnBuilder__second;
    #[allow(unused_parens)]
    #[automatically_derived]
    impl<S, T, __First, __Second> BonFnBuilder<S, T, (__First, __Second)> {
        ///Finishes building and performs the requested action.
        #[inline(always)]
        #[allow(clippy::inline_always, clippy::future_not_send)]
        pub fn call(self)
        where
            __First: ::bon::private::IntoSet<S, BonFnBuilder__first>,
            __Second: ::bon::private::IntoSet<T, BonFnBuilder__second>,
        {
            let first: S = ::bon::private::IntoSet::<
                S,
                BonFnBuilder__first,
            >::into_set(self.__private_named_members.0);
            let second: T = ::bon::private::IntoSet::<
                T,
                BonFnBuilder__second,
            >::into_set(self.__private_named_members.1);
            __orig_bon_fn::<S, T>(first, second)
        }
        ///Sets the value of `first`. See [`bon_fn()`] for more info.
        #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
        #[inline(always)]
        pub fn first(
            self,
            value: S,
        ) -> BonFnBuilder<S, T, (::bon::private::Set<S>, __Second)>
        where
            __First: ::bon::private::IsUnset,
        {
            BonFnBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_named_members: (
                    ::bon::private::Set(value),
                    self.__private_named_members.1,
                ),
            }
        }
        ///Sets the value of `second`. See [`bon_fn()`] for more info.
        #[allow(clippy::inline_always, clippy::impl_trait_in_params)]
        #[inline(always)]
        pub fn second(
            self,
            value: T,
        ) -> BonFnBuilder<S, T, (__First, ::bon::private::Set<T>)>
        where
            __Second: ::bon::private::IsUnset,
        {
            BonFnBuilder {
                __private_phantom: ::core::marker::PhantomData,
                __private_named_members: (
                    self.__private_named_members.0,
                    ::bon::private::Set(value),
                ),
            }
        }
    }
    /**Positional function equivalent of [`bon_fn()`].
See its docs for details.*/
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
    fn __orig_bon_fn<S, T>(first: S, second: T) {}
    #[allow(unused)]
    pub fn test_stemcells(cond: bool) {
        let stemcell = FooTypedBuilder::builder().first(1);
        if cond {
            let foo = stemcell.second(2).build();
        } else {}
        let stemcell = FooBon::builder().first(1f32);
        if cond {
            let foo = stemcell.second(1usize).build();
        } else {}
        let stemcell = FooBuildstructor::builder().first(1);
        if cond {
            let foo = stemcell.second(2).build();
        } else {}
        let stemcell = FooConstTyped::builder().first(1);
        if cond {
            let foo = stemcell.second(2).build();
        } else {}
        let stemcell = RuntimeFooBuilder::default().first(1);
        if cond {
            let foo = stemcell.second(2).build();
        } else {}
        let stemcell = bon_fn().first(1);
        if cond {
            let foo = stemcell.second(2).call();
        } else {}
        let stemcell = TypeStateFoo::builder().first(1);
        if cond {
            let foo = stemcell.second(2).build();
        } else {}
    }
}
