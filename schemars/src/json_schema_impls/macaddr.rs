use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use macaddr::MacAddr;

impl JsonSchema for MacAddr {
    no_ref_schema!();

    fn schema_name() -> String {
        "MacAddr".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}
