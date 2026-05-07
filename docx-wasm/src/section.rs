use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Section(docx_rs::Section);

impl Section {
    pub fn take(self) -> docx_rs::Section {
        self.0
    }
}

#[wasm_bindgen(js_name = createSection)]
pub fn create_section() -> Section {
    Section(docx_rs::Section::new())
}

#[wasm_bindgen]
impl Section {
    pub fn section_property(mut self, property: SectionProperty) -> Self {
        self.0 = self.0.with_property(property.take());
        self
    }

    pub fn header(mut self, header: Header) -> Self {
        self.0 = self.0.header(header.take());
        self
    }

    pub fn first_header(mut self, header: Header) -> Self {
        self.0 = self.0.first_header(header.take());
        self
    }

    pub fn even_header(mut self, header: Header) -> Self {
        self.0 = self.0.even_header(header.take());
        self
    }

    pub fn footer(mut self, footer: Footer) -> Self {
        self.0 = self.0.footer(footer.take());
        self
    }

    pub fn first_footer(mut self, footer: Footer) -> Self {
        self.0 = self.0.first_footer(footer.take());
        self
    }

    pub fn even_footer(mut self, footer: Footer) -> Self {
        self.0 = self.0.even_footer(footer.take());
        self
    }
}
