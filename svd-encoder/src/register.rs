use super::{new_node, Element, ElementMerge, Encode, EncodeChildren, EncodeError, XMLNode};

use crate::svd::{Field, Register, RegisterInfo};

impl Encode for Register {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        match self {
            Self::Single(info) => info.encode(),
            Self::Array(info, array_info) => {
                let mut base = Element::new("register");
                base.merge(&array_info.encode()?);
                base.merge(&info.encode()?);
                Ok(base)
            }
        }
    }
}

impl Encode for RegisterInfo {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        let mut elem = Element::new("register");
        elem.children.push(new_node("name", self.name.clone()));

        if let Some(v) = &self.display_name {
            elem.children.push(new_node("displayName", v.clone()));
        }

        if let Some(v) = &self.description {
            elem.children.push(new_node("description", v.clone()));
        }

        if let Some(v) = &self.alternate_group {
            elem.children
                .push(new_node("alternateGroup", v.to_string()));
        }

        if let Some(v) = &self.alternate_register {
            elem.children
                .push(new_node("alternateRegister", v.to_string()));
        }

        elem.children.push(new_node(
            "addressOffset",
            format!("0x{:X}", self.address_offset),
        ));

        elem.children.extend(self.properties.encode()?);

        if let Some(v) = &self.modified_write_values {
            elem.children.push(v.encode_node()?);
        }

        if let Some(v) = &self.write_constraint {
            elem.children.push(v.encode_node()?);
        }

        if let Some(v) = &self.read_action {
            elem.children.push(v.encode_node()?);
        }

        if let Some(v) = &self.fields {
            let children = v
                .iter()
                .map(Field::encode_node)
                .collect::<Result<Vec<_>, EncodeError>>()?;
            if !children.is_empty() {
                let mut fields = Element::new("fields");
                fields.children = children;
                elem.children.push(XMLNode::Element(fields));
            }
        }

        if let Some(v) = &self.derived_from {
            elem.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(elem)
    }
}
