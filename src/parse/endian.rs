use super::Element;

use crate::elementext::ElementExt;
use crate::error::*;
use crate::svd::Endian;
use crate::types::Parse;

impl Parse for Endian {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let text = tree.get_text()?;

        match &text[..] {
            "little" => Ok(Endian::Little),
            "big" => Ok(Endian::Big),
            "selectable" => Ok(Endian::Selectable),
            "other" => Ok(Endian::Other),
            s => Err(SVDError::UnknownEndian(s.into()).into()),
        }
    }
}
