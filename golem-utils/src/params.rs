use golem_rust::value_and_type::{FromValueAndType, IntoValue, TypeNodeBuilder};
use golem_rust::wasm_rpc::{NodeBuilder, WitValueExtractor};

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

#[derive(Debug, Clone, PartialEq)]
pub struct Param<A>(pub A);

impl<A> Param<A> {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce(A) -> R,
    {
        func(self.0)
    }
}

impl<A: IntoValue> IntoValue for Param<A> {
    fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = self.0.add_to_builder(builder.item());
        builder.finish()
    }

    fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = A::add_to_type_builder(builder.item());
        builder.finish()
    }
}

impl<A: FromValueAndType> FromValueAndType for Param<A> {
    fn from_extractor<'a, 'b>(
        extractor: &'a impl WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        let a = A::from_extractor(
            &extractor
                .tuple_element(0)
                .ok_or_else(|| "Expected 1-tuple".to_string())?,
        )?;
        Ok(Param(a))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param2<A, B>(pub A, pub B);

impl<A, B> Param2<A, B> {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce(A, B) -> R,
    {
        func(self.0, self.1)
    }
}

impl<A: IntoValue, B: IntoValue> IntoValue for Param2<A, B> {
    fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = self.0.add_to_builder(builder.item());
        builder = self.1.add_to_builder(builder.item());
        builder.finish()
    }

    fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = A::add_to_type_builder(builder.item());
        builder = B::add_to_type_builder(builder.item());
        builder.finish()
    }
}

impl<A: FromValueAndType, B: FromValueAndType> FromValueAndType for Param2<A, B> {
    fn from_extractor<'a, 'b>(
        extractor: &'a impl WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        let a = A::from_extractor(
            &extractor
                .tuple_element(0)
                .ok_or_else(|| "Expected 2-tuple".to_string())?,
        )?;
        let b = B::from_extractor(
            &extractor
                .tuple_element(1)
                .ok_or_else(|| "Expected 2-tuple".to_string())?,
        )?;
        Ok(Param2(a, b))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param3<A, B, C>(pub A, pub B, pub C);

impl<A, B, C> Param3<A, B, C> {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce(A, B, C) -> R,
    {
        func(self.0, self.1, self.2)
    }
}

impl<A: IntoValue, B: IntoValue, C: IntoValue> IntoValue for Param3<A, B, C> {
    fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = self.0.add_to_builder(builder.item());
        builder = self.1.add_to_builder(builder.item());
        builder = self.2.add_to_builder(builder.item());
        builder.finish()
    }

    fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = A::add_to_type_builder(builder.item());
        builder = B::add_to_type_builder(builder.item());
        builder = C::add_to_type_builder(builder.item());
        builder.finish()
    }
}

impl<A: FromValueAndType, B: FromValueAndType, C: FromValueAndType> FromValueAndType
    for Param3<A, B, C>
{
    fn from_extractor<'a, 'b>(
        extractor: &'a impl WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        let a = A::from_extractor(
            &extractor
                .tuple_element(0)
                .ok_or_else(|| "Expected 3-tuple".to_string())?,
        )?;
        let b = B::from_extractor(
            &extractor
                .tuple_element(1)
                .ok_or_else(|| "Expected 3-tuple".to_string())?,
        )?;
        let c = C::from_extractor(
            &extractor
                .tuple_element(2)
                .ok_or_else(|| "Expected 3-tuple".to_string())?,
        )?;
        Ok(Param3(a, b, c))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param4<A, B, C, D>(pub A, pub B, pub C, pub D);

impl<A, B, C, D> Param4<A, B, C, D> {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce(A, B, C, D) -> R,
    {
        func(self.0, self.1, self.2, self.3)
    }
}

impl<A: IntoValue, B: IntoValue, C: IntoValue, D: IntoValue> IntoValue for Param4<A, B, C, D> {
    fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = self.0.add_to_builder(builder.item());
        builder = self.1.add_to_builder(builder.item());
        builder = self.2.add_to_builder(builder.item());
        builder = self.3.add_to_builder(builder.item());
        builder.finish()
    }

    fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = A::add_to_type_builder(builder.item());
        builder = B::add_to_type_builder(builder.item());
        builder = C::add_to_type_builder(builder.item());
        builder = D::add_to_type_builder(builder.item());
        builder.finish()
    }
}

impl<A: FromValueAndType, B: FromValueAndType, C: FromValueAndType, D: FromValueAndType>
    FromValueAndType for Param4<A, B, C, D>
{
    fn from_extractor<'a, 'b>(
        extractor: &'a impl WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        let a = A::from_extractor(
            &extractor
                .tuple_element(0)
                .ok_or_else(|| "Expected 4-tuple".to_string())?,
        )?;
        let b = B::from_extractor(
            &extractor
                .tuple_element(1)
                .ok_or_else(|| "Expected 4-tuple".to_string())?,
        )?;
        let c = C::from_extractor(
            &extractor
                .tuple_element(2)
                .ok_or_else(|| "Expected 4-tuple".to_string())?,
        )?;
        let d = D::from_extractor(
            &extractor
                .tuple_element(3)
                .ok_or_else(|| "Expected 4-tuple".to_string())?,
        )?;
        Ok(Param4(a, b, c, d))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param5<A, B, C, D, E>(pub A, pub B, pub C, pub D, pub E);

impl<A, B, C, D, E> Param5<A, B, C, D, E> {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce(A, B, C, D, E) -> R,
    {
        func(self.0, self.1, self.2, self.3, self.4)
    }
}

impl<A: IntoValue, B: IntoValue, C: IntoValue, D: IntoValue, E: IntoValue> IntoValue
    for Param5<A, B, C, D, E>
{
    fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = self.0.add_to_builder(builder.item());
        builder = self.1.add_to_builder(builder.item());
        builder = self.2.add_to_builder(builder.item());
        builder = self.3.add_to_builder(builder.item());
        builder = self.4.add_to_builder(builder.item());
        builder.finish()
    }

    fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = A::add_to_type_builder(builder.item());
        builder = B::add_to_type_builder(builder.item());
        builder = C::add_to_type_builder(builder.item());
        builder = D::add_to_type_builder(builder.item());
        builder = E::add_to_type_builder(builder.item());
        builder.finish()
    }
}

impl<
        A: FromValueAndType,
        B: FromValueAndType,
        C: FromValueAndType,
        D: FromValueAndType,
        E: FromValueAndType,
    > FromValueAndType for Param5<A, B, C, D, E>
{
    fn from_extractor<'a, 'b>(
        extractor: &'a impl WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        let a = A::from_extractor(
            &extractor
                .tuple_element(0)
                .ok_or_else(|| "Expected 5-tuple".to_string())?,
        )?;
        let b = B::from_extractor(
            &extractor
                .tuple_element(1)
                .ok_or_else(|| "Expected 5-tuple".to_string())?,
        )?;
        let c = C::from_extractor(
            &extractor
                .tuple_element(2)
                .ok_or_else(|| "Expected 5-tuple".to_string())?,
        )?;
        let d = D::from_extractor(
            &extractor
                .tuple_element(3)
                .ok_or_else(|| "Expected 5-tuple".to_string())?,
        )?;
        let e = E::from_extractor(
            &extractor
                .tuple_element(4)
                .ok_or_else(|| "Expected 5-tuple".to_string())?,
        )?;
        Ok(Param5(a, b, c, d, e))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param6<A, B, C, D, E, F>(pub A, pub B, pub C, pub D, pub E, pub F);

impl<A, B, C, D, E, F> Param6<A, B, C, D, E, F> {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce(A, B, C, D, E, F) -> R,
    {
        func(self.0, self.1, self.2, self.3, self.4, self.5)
    }
}

impl<A: IntoValue, B: IntoValue, C: IntoValue, D: IntoValue, E: IntoValue, F: IntoValue> IntoValue
    for Param6<A, B, C, D, E, F>
{
    fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = self.0.add_to_builder(builder.item());
        builder = self.1.add_to_builder(builder.item());
        builder = self.2.add_to_builder(builder.item());
        builder = self.3.add_to_builder(builder.item());
        builder = self.4.add_to_builder(builder.item());
        builder = self.5.add_to_builder(builder.item());
        builder.finish()
    }

    fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = A::add_to_type_builder(builder.item());
        builder = B::add_to_type_builder(builder.item());
        builder = C::add_to_type_builder(builder.item());
        builder = D::add_to_type_builder(builder.item());
        builder = E::add_to_type_builder(builder.item());
        builder = F::add_to_type_builder(builder.item());
        builder.finish()
    }
}

impl<
        A: FromValueAndType,
        B: FromValueAndType,
        C: FromValueAndType,
        D: FromValueAndType,
        E: FromValueAndType,
        F: FromValueAndType,
    > FromValueAndType for Param6<A, B, C, D, E, F>
{
    fn from_extractor<'a, 'b>(
        extractor: &'a impl WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        let a = A::from_extractor(
            &extractor
                .tuple_element(0)
                .ok_or_else(|| "Expected 6-tuple".to_string())?,
        )?;
        let b = B::from_extractor(
            &extractor
                .tuple_element(1)
                .ok_or_else(|| "Expected 6-tuple".to_string())?,
        )?;
        let c = C::from_extractor(
            &extractor
                .tuple_element(2)
                .ok_or_else(|| "Expected 6-tuple".to_string())?,
        )?;
        let d = D::from_extractor(
            &extractor
                .tuple_element(3)
                .ok_or_else(|| "Expected 6-tuple".to_string())?,
        )?;
        let e = E::from_extractor(
            &extractor
                .tuple_element(4)
                .ok_or_else(|| "Expected 6-tuple".to_string())?,
        )?;
        let f = F::from_extractor(
            &extractor
                .tuple_element(5)
                .ok_or_else(|| "Expected 6-tuple".to_string())?,
        )?;
        Ok(Param6(a, b, c, d, e, f))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param7<A, B, C, D, E, F, G>(pub A, pub B, pub C, pub D, pub E, pub F, pub G);

impl<A, B, C, D, E, F, G> Param7<A, B, C, D, E, F, G> {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce(A, B, C, D, E, F, G) -> R,
    {
        func(self.0, self.1, self.2, self.3, self.4, self.5, self.6)
    }
}

impl<
        A: IntoValue,
        B: IntoValue,
        C: IntoValue,
        D: IntoValue,
        E: IntoValue,
        F: IntoValue,
        G: IntoValue,
    > IntoValue for Param7<A, B, C, D, E, F, G>
{
    fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = self.0.add_to_builder(builder.item());
        builder = self.1.add_to_builder(builder.item());
        builder = self.2.add_to_builder(builder.item());
        builder = self.3.add_to_builder(builder.item());
        builder = self.4.add_to_builder(builder.item());
        builder = self.5.add_to_builder(builder.item());
        builder = self.6.add_to_builder(builder.item());
        builder.finish()
    }

    fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = A::add_to_type_builder(builder.item());
        builder = B::add_to_type_builder(builder.item());
        builder = C::add_to_type_builder(builder.item());
        builder = D::add_to_type_builder(builder.item());
        builder = E::add_to_type_builder(builder.item());
        builder = F::add_to_type_builder(builder.item());
        builder = G::add_to_type_builder(builder.item());
        builder.finish()
    }
}

impl<
        A: FromValueAndType,
        B: FromValueAndType,
        C: FromValueAndType,
        D: FromValueAndType,
        E: FromValueAndType,
        F: FromValueAndType,
        G: FromValueAndType,
    > FromValueAndType for Param7<A, B, C, D, E, F, G>
{
    fn from_extractor<'a, 'b>(
        extractor: &'a impl WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        let a = A::from_extractor(
            &extractor
                .tuple_element(0)
                .ok_or_else(|| "Expected 7-tuple".to_string())?,
        )?;
        let b = B::from_extractor(
            &extractor
                .tuple_element(1)
                .ok_or_else(|| "Expected 7-tuple".to_string())?,
        )?;
        let c = C::from_extractor(
            &extractor
                .tuple_element(2)
                .ok_or_else(|| "Expected 7-tuple".to_string())?,
        )?;
        let d = D::from_extractor(
            &extractor
                .tuple_element(3)
                .ok_or_else(|| "Expected 7-tuple".to_string())?,
        )?;
        let e = E::from_extractor(
            &extractor
                .tuple_element(4)
                .ok_or_else(|| "Expected 7-tuple".to_string())?,
        )?;
        let f = F::from_extractor(
            &extractor
                .tuple_element(5)
                .ok_or_else(|| "Expected 7-tuple".to_string())?,
        )?;
        let g = G::from_extractor(
            &extractor
                .tuple_element(6)
                .ok_or_else(|| "Expected 7-tuple".to_string())?,
        )?;
        Ok(Param7(a, b, c, d, e, f, g))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param8<A, B, C, D, E, F, G, H>(pub A, pub B, pub C, pub D, pub E, pub F, pub G, pub H);

impl<A, B, C, D, E, F, G, H> Param8<A, B, C, D, E, F, G, H> {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce(A, B, C, D, E, F, G, H) -> R,
    {
        func(
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7,
        )
    }
}

impl<
        A: IntoValue,
        B: IntoValue,
        C: IntoValue,
        D: IntoValue,
        E: IntoValue,
        F: IntoValue,
        G: IntoValue,
        H: IntoValue,
    > IntoValue for Param8<A, B, C, D, E, F, G, H>
{
    fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = self.0.add_to_builder(builder.item());
        builder = self.1.add_to_builder(builder.item());
        builder = self.2.add_to_builder(builder.item());
        builder = self.3.add_to_builder(builder.item());
        builder = self.4.add_to_builder(builder.item());
        builder = self.5.add_to_builder(builder.item());
        builder = self.6.add_to_builder(builder.item());
        builder = self.7.add_to_builder(builder.item());
        builder.finish()
    }

    fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = A::add_to_type_builder(builder.item());
        builder = B::add_to_type_builder(builder.item());
        builder = C::add_to_type_builder(builder.item());
        builder = D::add_to_type_builder(builder.item());
        builder = E::add_to_type_builder(builder.item());
        builder = F::add_to_type_builder(builder.item());
        builder = G::add_to_type_builder(builder.item());
        builder = H::add_to_type_builder(builder.item());
        builder.finish()
    }
}

impl<
        A: FromValueAndType,
        B: FromValueAndType,
        C: FromValueAndType,
        D: FromValueAndType,
        E: FromValueAndType,
        F: FromValueAndType,
        G: FromValueAndType,
        H: FromValueAndType,
    > FromValueAndType for Param8<A, B, C, D, E, F, G, H>
{
    fn from_extractor<'a, 'b>(
        extractor: &'a impl WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        let a = A::from_extractor(
            &extractor
                .tuple_element(0)
                .ok_or_else(|| "Expected 8-tuple".to_string())?,
        )?;
        let b = B::from_extractor(
            &extractor
                .tuple_element(1)
                .ok_or_else(|| "Expected 8-tuple".to_string())?,
        )?;
        let c = C::from_extractor(
            &extractor
                .tuple_element(2)
                .ok_or_else(|| "Expected 8-tuple".to_string())?,
        )?;
        let d = D::from_extractor(
            &extractor
                .tuple_element(3)
                .ok_or_else(|| "Expected 8-tuple".to_string())?,
        )?;
        let e = E::from_extractor(
            &extractor
                .tuple_element(4)
                .ok_or_else(|| "Expected 8-tuple".to_string())?,
        )?;
        let f = F::from_extractor(
            &extractor
                .tuple_element(5)
                .ok_or_else(|| "Expected 8-tuple".to_string())?,
        )?;
        let g = G::from_extractor(
            &extractor
                .tuple_element(6)
                .ok_or_else(|| "Expected 8-tuple".to_string())?,
        )?;
        let h = H::from_extractor(
            &extractor
                .tuple_element(7)
                .ok_or_else(|| "Expected 8-tuple".to_string())?,
        )?;
        Ok(Param8(a, b, c, d, e, f, g, h))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param9<A, B, C, D, E, F, G, H, I>(
    pub A,
    pub B,
    pub C,
    pub D,
    pub E,
    pub F,
    pub G,
    pub H,
    pub I,
);

impl<A, B, C, D, E, F, G, H, I> Param9<A, B, C, D, E, F, G, H, I> {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce(A, B, C, D, E, F, G, H, I) -> R,
    {
        func(
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8,
        )
    }
}

impl<
        A: IntoValue,
        B: IntoValue,
        C: IntoValue,
        D: IntoValue,
        E: IntoValue,
        F: IntoValue,
        G: IntoValue,
        H: IntoValue,
        I: IntoValue,
    > IntoValue for Param9<A, B, C, D, E, F, G, H, I>
{
    fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = self.0.add_to_builder(builder.item());
        builder = self.1.add_to_builder(builder.item());
        builder = self.2.add_to_builder(builder.item());
        builder = self.3.add_to_builder(builder.item());
        builder = self.4.add_to_builder(builder.item());
        builder = self.5.add_to_builder(builder.item());
        builder = self.6.add_to_builder(builder.item());
        builder = self.7.add_to_builder(builder.item());
        builder = self.8.add_to_builder(builder.item());
        builder.finish()
    }

    fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = A::add_to_type_builder(builder.item());
        builder = B::add_to_type_builder(builder.item());
        builder = C::add_to_type_builder(builder.item());
        builder = D::add_to_type_builder(builder.item());
        builder = E::add_to_type_builder(builder.item());
        builder = F::add_to_type_builder(builder.item());
        builder = G::add_to_type_builder(builder.item());
        builder = H::add_to_type_builder(builder.item());
        builder = I::add_to_type_builder(builder.item());
        builder.finish()
    }
}

impl<
        A: FromValueAndType,
        B: FromValueAndType,
        C: FromValueAndType,
        D: FromValueAndType,
        E: FromValueAndType,
        F: FromValueAndType,
        G: FromValueAndType,
        H: FromValueAndType,
        I: FromValueAndType,
    > FromValueAndType for Param9<A, B, C, D, E, F, G, H, I>
{
    fn from_extractor<'a, 'b>(
        extractor: &'a impl WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        let a = A::from_extractor(
            &extractor
                .tuple_element(0)
                .ok_or_else(|| "Expected 9-tuple".to_string())?,
        )?;
        let b = B::from_extractor(
            &extractor
                .tuple_element(1)
                .ok_or_else(|| "Expected 9-tuple".to_string())?,
        )?;
        let c = C::from_extractor(
            &extractor
                .tuple_element(2)
                .ok_or_else(|| "Expected 9-tuple".to_string())?,
        )?;
        let d = D::from_extractor(
            &extractor
                .tuple_element(3)
                .ok_or_else(|| "Expected 9-tuple".to_string())?,
        )?;
        let e = E::from_extractor(
            &extractor
                .tuple_element(4)
                .ok_or_else(|| "Expected 9-tuple".to_string())?,
        )?;
        let f = F::from_extractor(
            &extractor
                .tuple_element(5)
                .ok_or_else(|| "Expected 9-tuple".to_string())?,
        )?;
        let g = G::from_extractor(
            &extractor
                .tuple_element(6)
                .ok_or_else(|| "Expected 9-tuple".to_string())?,
        )?;
        let h = H::from_extractor(
            &extractor
                .tuple_element(7)
                .ok_or_else(|| "Expected 9-tuple".to_string())?,
        )?;
        let i = I::from_extractor(
            &extractor
                .tuple_element(8)
                .ok_or_else(|| "Expected 9-tuple".to_string())?,
        )?;
        Ok(Param9(a, b, c, d, e, f, g, h, i))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param10<A, B, C, D, E, F, G, H, I, J>(
    pub A,
    pub B,
    pub C,
    pub D,
    pub E,
    pub F,
    pub G,
    pub H,
    pub I,
    pub J,
);

impl<A, B, C, D, E, F, G, H, I, J> Param10<A, B, C, D, E, F, G, H, I, J> {
    pub fn invoke<Func, R>(self, func: Func) -> R
    where
        Func: FnOnce(A, B, C, D, E, F, G, H, I, J) -> R,
    {
        func(
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
        )
    }
}

impl<
        A: IntoValue,
        B: IntoValue,
        C: IntoValue,
        D: IntoValue,
        E: IntoValue,
        F: IntoValue,
        G: IntoValue,
        H: IntoValue,
        I: IntoValue,
        J: IntoValue,
    > IntoValue for Param10<A, B, C, D, E, F, G, H, I, J>
{
    fn add_to_builder<T: NodeBuilder>(self, builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = self.0.add_to_builder(builder.item());
        builder = self.1.add_to_builder(builder.item());
        builder = self.2.add_to_builder(builder.item());
        builder = self.3.add_to_builder(builder.item());
        builder = self.4.add_to_builder(builder.item());
        builder = self.5.add_to_builder(builder.item());
        builder = self.6.add_to_builder(builder.item());
        builder = self.7.add_to_builder(builder.item());
        builder = self.8.add_to_builder(builder.item());
        builder = self.9.add_to_builder(builder.item());
        builder.finish()
    }

    fn add_to_type_builder<T: TypeNodeBuilder>(builder: T) -> T::Result {
        let mut builder = builder.tuple();
        builder = A::add_to_type_builder(builder.item());
        builder = B::add_to_type_builder(builder.item());
        builder = C::add_to_type_builder(builder.item());
        builder = D::add_to_type_builder(builder.item());
        builder = E::add_to_type_builder(builder.item());
        builder = F::add_to_type_builder(builder.item());
        builder = G::add_to_type_builder(builder.item());
        builder = H::add_to_type_builder(builder.item());
        builder = I::add_to_type_builder(builder.item());
        builder = J::add_to_type_builder(builder.item());
        builder.finish()
    }
}

impl<
        A: FromValueAndType,
        B: FromValueAndType,
        C: FromValueAndType,
        D: FromValueAndType,
        E: FromValueAndType,
        F: FromValueAndType,
        G: FromValueAndType,
        H: FromValueAndType,
        I: FromValueAndType,
        J: FromValueAndType,
    > FromValueAndType for Param10<A, B, C, D, E, F, G, H, I, J>
{
    fn from_extractor<'a, 'b>(
        extractor: &'a impl WitValueExtractor<'a, 'b>,
    ) -> Result<Self, String> {
        let a = A::from_extractor(
            &extractor
                .tuple_element(0)
                .ok_or_else(|| "Expected 10-tuple".to_string())?,
        )?;
        let b = B::from_extractor(
            &extractor
                .tuple_element(1)
                .ok_or_else(|| "Expected 10-tuple".to_string())?,
        )?;
        let c = C::from_extractor(
            &extractor
                .tuple_element(2)
                .ok_or_else(|| "Expected 10-tuple".to_string())?,
        )?;
        let d = D::from_extractor(
            &extractor
                .tuple_element(3)
                .ok_or_else(|| "Expected 10-tuple".to_string())?,
        )?;
        let e = E::from_extractor(
            &extractor
                .tuple_element(4)
                .ok_or_else(|| "Expected 10-tuple".to_string())?,
        )?;
        let f = F::from_extractor(
            &extractor
                .tuple_element(5)
                .ok_or_else(|| "Expected 10-tuple".to_string())?,
        )?;
        let g = G::from_extractor(
            &extractor
                .tuple_element(6)
                .ok_or_else(|| "Expected 10-tuple".to_string())?,
        )?;
        let h = H::from_extractor(
            &extractor
                .tuple_element(7)
                .ok_or_else(|| "Expected 10-tuple".to_string())?,
        )?;
        let i = I::from_extractor(
            &extractor
                .tuple_element(8)
                .ok_or_else(|| "Expected 10-tuple".to_string())?,
        )?;
        let j = J::from_extractor(
            &extractor
                .tuple_element(9)
                .ok_or_else(|| "Expected 10-tuple".to_string())?,
        )?;
        Ok(Param10(a, b, c, d, e, f, g, h, i, j))
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
        roundtrip_assert(Param(1));
    }

    #[test]
    fn roundtrip_test_param2() {
        roundtrip_assert(Param2(1, 2));
    }

    #[test]
    fn roundtrip_test_param3() {
        roundtrip_assert(Param3(1, 2, 3));
    }

    #[test]
    fn roundtrip_test_param4() {
        roundtrip_assert(Param4(1, 2, 3, 4));
    }

    #[test]
    fn roundtrip_test_param5() {
        roundtrip_assert(Param5(1, 2, 3, 4, 5));
    }

    #[test]
    fn roundtrip_test_param6() {
        roundtrip_assert(Param6(1, 2, 3, 4, 5, 6));
    }

    #[test]
    fn roundtrip_test_param7() {
        roundtrip_assert(Param7(1, 2, 3, 4, 5, 6, 7));
    }

    #[test]
    fn roundtrip_test_param8() {
        roundtrip_assert(Param8(1, 2, 3, 4, 5, 6, 7, 8));
    }

    #[test]
    fn roundtrip_test_param9() {
        roundtrip_assert(Param9(1, 2, 3, 4, 5, 6, 7, 8, 9));
    }

    #[test]
    fn roundtrip_test_param10() {
        roundtrip_assert(Param10(1, 2, 3, 4, 5, 6, 7, 8, 9, 10));
    }
}
