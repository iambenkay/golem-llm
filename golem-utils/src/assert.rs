use golem_rust::value_and_type::{FromValueAndType, IntoValueAndType};

pub fn roundtrip_assert<
    T: std::fmt::Debug + Clone + PartialEq + IntoValueAndType + FromValueAndType,
>(
    value: T,
) {
    let vnt = value.clone().into_value_and_type();
    println!("{vnt:?}");
    let extracted = T::from_value_and_type(vnt).unwrap();
    assert_eq!(value, extracted);
}
