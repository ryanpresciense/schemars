use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use macaddr::*;

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

impl JsonSchema for MacAddr6 {
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


impl JsonSchema for MacAddr8 {
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
