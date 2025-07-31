use golem_rust::value_and_type::{FromValueAndType, IntoValue, TypeNodeBuilder};
use golem_rust::wasm_rpc::{NodeBuilder, WitValueExtractor};

use paste::paste;

macro_rules! define_params {
    ($count:literal, $name:ident<$($typ:ty),*>) => {

        paste! {
            #[derive(Debug, Clone, PartialEq)]
            pub struct [<$name>]<$($typ),*>{
                $(pub [<$typ:lower>]: $typ),*
            }

            impl<$($typ),*> [<$name>]<$([<$typ>]),*> {
                pub fn new($([<$typ:lower>]: $typ),*) -> Self {
                    Self {
                        $([<$typ:lower>]),*
                    }
                }

                pub fn invoke<Func, R>(self, func: Func) -> R
                where
                    Func: FnOnce($($typ),*) -> R,
                {
                    func($(self.[<$typ:lower>]),*)
                }
            }

            impl<$($typ: IntoValue),*> IntoValue for [<$name>]<$([<$typ>]),*> {
                fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
                    let mut builder = builder.tuple();

                    $(
                        builder = self.[<$typ:lower>].add_to_builder(builder.item());
                    )*
                    builder.finish()
                }

                fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
                    let mut builder = builder.tuple();
                    $(
                        builder = $typ::add_to_type_builder(builder.item());
                    )*
                    builder.finish()
                }
            }

            impl<$($typ: FromValueAndType),*> FromValueAndType for [<$name>]<$($typ),*> {
                fn from_extractor<'a, 'b>(
                    extractor: &'a impl WitValueExtractor<'a, 'b>,
                ) -> Result<Self, String> {
                    let mut _index = 0;
                    $(
                        let [<$typ:lower>] = $typ::from_extractor(
                            &extractor
                                .tuple_element(_index)
                                .ok_or_else(|| format!("Expected {} tuple", $count))?,
                        )?;
                        _index += 1;
                    )*
                    Ok([<$name>]::new($([<$typ:lower>]),*))
                }
            }
        }
    };
}

define_params!(1, Param<P1>);
define_params!(2, Param2<P1, P2>);
define_params!(3, Param3<P1, P2, P3>);
define_params!(4, Param4<P1, P2, P3, P4>);
define_params!(5, Param5<P1, P2, P3, P4, P5>);
define_params!(6, Param6<P1, P2, P3, P4, P5, P6>);
define_params!(7, Param7<P1, P2, P3, P4, P5, P6, P7>);
define_params!(8, Param8<P1, P2, P3, P4, P5, P6, P7, P8>);
define_params!(9, Param9<P1, P2, P3, P4, P5, P6, P7, P8, P9>);
define_params!(10, Param10<P1, P2, P3, P4, P5, P6, P7, P8, P9, P10>);

#[derive(Debug, Clone, PartialEq, golem_rust::IntoValue, golem_rust::FromValueAndType)]
pub struct NoParam;

impl NoParam {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce() -> R,
    {
        func()
    }
}

mod tests {
    use super::*;
    use crate::assert::roundtrip_assert;

    #[test]
    fn roundtrip_test_noparam() {
        roundtrip_assert(NoParam);
    }

    #[test]
    fn roundtrip_test_param() {
        roundtrip_assert(Param::new(1));
    }

    #[test]
    fn roundtrip_test_param2() {
        roundtrip_assert(Param2::new(1, 2));
    }

    #[test]
    fn roundtrip_test_param3() {
        roundtrip_assert(Param3::new(1, 2, 3));
    }

    #[test]
    fn roundtrip_test_param4() {
        roundtrip_assert(Param4::new(1, 2, 3, 4));
    }

    #[test]
    fn roundtrip_test_param5() {
        roundtrip_assert(Param5::new(1, 2, 3, 4, 5));
    }

    #[test]
    fn roundtrip_test_param6() {
        roundtrip_assert(Param6::new(1, 2, 3, 4, 5, 6));
    }

    #[test]
    fn roundtrip_test_param7() {
        roundtrip_assert(Param7::new(1, 2, 3, 4, 5, 6, 7));
    }

    #[test]
    fn roundtrip_test_param8() {
        roundtrip_assert(Param8::new(1, 2, 3, 4, 5, 6, 7, 8));
    }

    #[test]
    fn roundtrip_test_param9() {
        roundtrip_assert(Param9::new(1, 2, 3, 4, 5, 6, 7, 8, 9));
    }

    #[test]
    fn roundtrip_test_param10() {
        roundtrip_assert(Param10::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10));
    }
}
