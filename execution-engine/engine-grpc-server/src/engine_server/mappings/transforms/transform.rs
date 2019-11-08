use std::convert::{TryFrom, TryInto};

use contract_ffi::value::Value;
use engine_shared::transform::{Error as TransformError, Transform};

use crate::engine_server::{
    mappings::{state::NamedKeyMap, ParsingError},
    state::NamedKey as ProtobufNamedKey,
    transforms::{
        Transform as ProtobufTransform, Transform_oneof_transform_instance as ProtobufTransformEnum,
    },
};

impl From<Transform> for ProtobufTransform {
    fn from(transform: Transform) -> Self {
        let mut pb_transform = ProtobufTransform::new();
        match transform {
            Transform::Identity => {
                pb_transform.set_identity(Default::default());
            }
            Transform::AddInt32(i) => {
                pb_transform.mut_add_i32().set_value(i);
            }
            Transform::AddUInt64(u) => {
                pb_transform.mut_add_u64().set_value(u);
            }
            Transform::Write(value) => {
                pb_transform.mut_write().set_value(value.into());
            }
            Transform::AddKeys(keys_map) => {
                let pb_named_keys: Vec<ProtobufNamedKey> = NamedKeyMap(keys_map).into();
                pb_transform.mut_add_keys().set_value(pb_named_keys.into());
            }
            Transform::Failure(transform_error) => pb_transform.set_failure(transform_error.into()),
            Transform::AddUInt128(uint128) => {
                pb_transform.mut_add_big_int().set_value(uint128.into());
            }
            Transform::AddUInt256(uint256) => {
                pb_transform.mut_add_big_int().set_value(uint256.into());
            }
            Transform::AddUInt512(uint512) => {
                pb_transform.mut_add_big_int().set_value(uint512.into());
            }
        };
        pb_transform
    }
}

impl TryFrom<ProtobufTransform> for Transform {
    type Error = ParsingError;

    fn try_from(pb_transform: ProtobufTransform) -> Result<Self, Self::Error> {
        let pb_transform = pb_transform
            .transform_instance
            .ok_or_else(|| ParsingError::from("Unable to parse Protobuf Transform"))?;
        let transform = match pb_transform {
            ProtobufTransformEnum::identity(_) => Transform::Identity,
            ProtobufTransformEnum::add_keys(pb_add_keys) => {
                let named_keys_map: NamedKeyMap = pb_add_keys.value.into_vec().try_into()?;
                named_keys_map.0.into()
            }
            ProtobufTransformEnum::add_i32(pb_add_int32) => pb_add_int32.value.into(),
            ProtobufTransformEnum::add_u64(pb_add_u64) => pb_add_u64.value.into(),
            ProtobufTransformEnum::add_big_int(mut pb_big_int) => {
                let value = pb_big_int.take_value().try_into()?;
                match value {
                    Value::UInt128(uint128) => uint128.into(),
                    Value::UInt256(uint256) => uint256.into(),
                    Value::UInt512(uint512) => uint512.into(),
                    other => {
                        return Err(ParsingError(format!(
                            "Protobuf BigInt was turned into a non-uint Value type: {:?}",
                            other
                        )));
                    }
                }
            }
            ProtobufTransformEnum::write(mut pb_write) => {
                let value = Value::try_from(pb_write.take_value())?;
                Transform::Write(value)
            }
            ProtobufTransformEnum::failure(pb_failure) => {
                let error = TransformError::try_from(pb_failure)?;
                Transform::Failure(error)
            }
        };
        Ok(transform)
    }
}

#[cfg(test)]
mod tests {
    use proptest::proptest;

    use engine_shared::transform::gens;

    use super::*;
    use crate::engine_server::mappings::test_utils;

    proptest! {
        #[test]
        fn round_trip(transform in gens::transform_arb()) {
            test_utils::protobuf_round_trip::<Transform, ProtobufTransform>(transform);
        }
    }
}
