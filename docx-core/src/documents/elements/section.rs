use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;
use crate::{
    DocGrid, Footer, Header, PageMargin, PageNumType, PageOrientationType, PageSize, SectionType,
};
use std::io::Write;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    property: SectionProperty,
}

impl Section {
    pub fn new() -> Section {
        Default::default()
    }

    pub fn with_property(mut self, property: SectionProperty) -> Self {
        self.property = property;
        self
    }

    pub fn title_pg(mut self) -> Self {
        self.property = self.property.title_pg();
        self
    }

    pub fn page_size(mut self, size: PageSize) -> Self {
        self.property = self.property.page_size(size);
        self
    }

    pub fn page_margin(mut self, margin: PageMargin) -> Self {
        self.property = self.property.page_margin(margin);
        self
    }

    pub fn page_orient(mut self, orientation: PageOrientationType) -> Self {
        self.property = self.property.page_orient(orientation);
        self
    }

    pub fn doc_grid(mut self, doc_grid: DocGrid) -> Self {
        self.property = self.property.doc_grid(doc_grid);
        self
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.property.columns = columns;
        self
    }

    pub fn space(mut self, space: usize) -> Self {
        self.property.space = space;
        self
    }

    pub fn text_direction(mut self, direction: impl Into<String>) -> Self {
        self.property = self.property.text_direction(direction.into());
        self
    }

    pub fn section_type(mut self, section_type: SectionType) -> Self {
        self.property = self.property.section_type(section_type);
        self
    }

    pub fn page_num_type(mut self, page_num_type: PageNumType) -> Self {
        self.property = self.property.page_num_type(page_num_type);
        self
    }

    pub fn header(mut self, header: Header) -> Self {
        self.property.header = Some(header);
        self.property
            .header_reference
            .replace(HeaderReference::new("default", ""));
        self
    }

    pub fn first_header(mut self, header: Header) -> Self {
        self.property.first_header = Some(header);
        self.property
            .first_header_reference
            .replace(HeaderReference::new("first", ""));
        self.property.title_pg = true;
        self
    }

    pub fn even_header(mut self, header: Header) -> Self {
        self.property.even_header = Some(header);
        self.property
            .even_header_reference
            .replace(HeaderReference::new("even", ""));
        self
    }

    pub fn footer(mut self, footer: Footer) -> Self {
        self.property.footer = Some(footer);
        self.property
            .footer_reference
            .replace(FooterReference::new("default", ""));
        self
    }

    pub fn first_footer(mut self, footer: Footer) -> Self {
        self.property.first_footer = Some(footer);
        self.property
            .first_footer_reference
            .replace(FooterReference::new("first", ""));
        self.property.title_pg = true;
        self
    }

    pub fn even_footer(mut self, footer: Footer) -> Self {
        self.property.even_footer = Some(footer);
        self.property
            .even_footer_reference
            .replace(FooterReference::new("even", ""));
        self
    }

    pub(crate) fn property(&self) -> &SectionProperty {
        &self.property
    }

    pub(crate) fn property_mut(&mut self) -> &mut SectionProperty {
        &mut self.property
    }

    pub(crate) fn into_property(self) -> SectionProperty {
        self.property
    }
}

impl Default for Section {
    fn default() -> Self {
        Self {
            property: SectionProperty::new(),
        }
    }
}

impl BuildXML for Section {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        let id = crate::generate_para_id();
        XMLBuilder::from(stream)
            .open_paragraph(&id)?
            .open_paragraph_property()?
            .add_child(&self.property)?
            .close()?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_section_property_default() {
        let c = Section::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p w14:paraId="12345678"><w:pPr><w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" w:num="1" /></w:sectPr></w:pPr></w:p>"#
        );
    }
}
