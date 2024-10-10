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

#[derive(typed_builder::TypedBuilder)]
pub struct FooTypedBuilder<S, T> {
    first: S,
    second: T,
}

#[derive(bon::Builder)]
pub struct FooBon<S, T> {
    first: S,
    second: T,
}

#[derive(buildstructor::Builder)]
pub struct FooBuildstructor<S, T> {
    first: S,
    second: T,
}

use const_typed_builder::Builder;
#[derive(const_typed_builder::Builder)]
pub struct FooConstTyped<S, T> {
    first: S,
    second: T,
}

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct RuntimeFoo<S, T> {
    first: S,
    second: T,
}

#[derive(typestate_builder::TypestateBuilder)]
pub struct TypeStateFoo<S, T> {
    first: S,
    second: T,
}

#[bon::builder]
pub fn bon_fn<S, T>(first: S, second: T) {}

#[allow(unused)]
pub fn test_stemcells(cond: bool) {
    // let builder = Foo::builder();

    // typed_builder crate: does not work!
    let stemcell = FooTypedBuilder::builder().first(1);
    if cond {
        let foo = stemcell.second(2).build();
    } else {
        // let foo = stemcell.second(3f32);
    }

    // bon crate: does not work!
    let stemcell = FooBon::builder().first(1f32);
    if cond {
        let foo = stemcell.second(1usize).build();
    } else {
        // let foo = stemcell.second("hi").build();
    }

    // buildstructor crate: does not work!
    let stemcell = FooBuildstructor::builder().first(1);
    if cond {
        let foo = stemcell.second(2).build();
    } else {
        // let foo = stemcell.second(3f32).build();
    }

    let stemcell = FooConstTypedBuilder::new().first(1);
    if cond {
        let foo = stemcell.second(2).build();
    } else {
        // let foo = stemcell.second(3.).build();
    }

    let stemcell = RuntimeFooBuilder::default().first(1);
    if cond {
        let foo = stemcell.second(2).build();
    } else {
        // let foo = stemcell.second(3.).build();
    }

    let stemcell = bon_fn().first(1);
    if cond {
        let foo = stemcell.second(2).call();
    } else {
        // let foo = stemcell.second(3.).call();
    }

    // this does not work with generics
    // let stemcell = TypeStateFoo::builder().first(1).second(2);

    // but this does, but it's not the intended use of the typestate builder
    let stemcell =
        TypeStateFooBuilder::<TypeStateFooBuilderFirstEmpty, TypeStateFooBuilderSecondEmpty> {
            first: TypeStateFooBuilderFirstEmpty {},
            second: TypeStateFooBuilderSecondEmpty {},
        }
        .first(1);

    if cond {
        let foo = stemcell.second(2).build();
    } else {
        let foo = stemcell.second(3.).build();
    }
}
