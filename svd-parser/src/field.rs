use super::*;
use crate::svd::{
    Access, BitRange, EnumeratedValues, Field, FieldInfo, ModifiedWriteValues, ReadAction,
    WriteConstraint,
};

impl Parse for Field {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        parse_array("field", tree, config)
    }
}

impl Parse for FieldInfo {
    type Object = Self;
    type Error = SVDErrorAt;
    type Config = Config;

    fn parse(tree: &Node, config: &Self::Config) -> Result<Self, Self::Error> {
        if !tree.has_tag_name("field") {
            return Err(SVDError::NotExpectedTag("field".to_string()).at(tree.id()));
        }

        let bit_range = BitRange::parse(tree, config)?;
        FieldInfo::builder()
            .name(tree.get_child_text("name")?)
            .description(tree.get_child_text_opt("description")?)
            .bit_range(bit_range)
            .access(optional::<Access>("access", tree, config)?)
            .modified_write_values(optional::<ModifiedWriteValues>(
                "modifiedWriteValues",
                tree,
                config,
            )?)
            .write_constraint(if !config.ignore_enums {
                optional::<WriteConstraint>("writeConstraint", tree, config)?
            } else {
                None
            })
            .read_action(optional::<ReadAction>("readAction", tree, config)?)
            .enumerated_values(if !config.ignore_enums {
                let values: Result<Vec<_>, _> = tree
                    .children()
                    .filter(|t| t.is_element() && t.has_tag_name("enumeratedValues"))
                    .map(|t| EnumeratedValues::parse(&t, config))
                    .collect();
                values?
            } else {
                Vec::new()
            })
            .derived_from(tree.attribute("derivedFrom").map(|s| s.to_owned()))
            .build(config.validate_level)
            .map_err(|e| SVDError::from(e).at(tree.id()))
    }
}
