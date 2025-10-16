use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct SectionProperty(docx_rs::SectionProperty);

#[wasm_bindgen(js_name = createSectionProperty)]
pub fn create_section_property() -> SectionProperty {
    SectionProperty(docx_rs::SectionProperty::new())
}

#[wasm_bindgen]
impl SectionProperty {
    pub fn columns(mut self, columns: usize) -> Self {
        self.0.columns = columns;
        self
    }

    pub fn space(mut self, space: usize) -> Self {
        self.0.space = space;
        self
    }

    pub fn title_pg(mut self) -> Self {
        self.0 = self.0.title_pg();
        self
    }

    pub fn page_size(mut self, w: u32, h: u32) -> Self {
        self.0 = self.0.page_size(docx_rs::PageSize::new().size(w, h));
        self
    }

    pub fn page_margin(mut self, margin: PageMargin) -> Self {
        self.0 = self.0.page_margin(margin.take());
        self
    }

    pub fn page_orient(mut self, orientation: docx_rs::PageOrientationType) -> Self {
        self.0 = self.0.page_orient(orientation);
        self
    }

    pub fn doc_grid(
        mut self,
        grid_type: docx_rs::DocGridType,
        line_pitch: Option<usize>,
        char_space: Option<isize>,
    ) -> Self {
        let mut doc_grid = docx_rs::DocGrid::with_empty().grid_type(grid_type);
        if let Some(line_pitch) = line_pitch {
            doc_grid = doc_grid.line_pitch(line_pitch);
        }
        if let Some(char_space) = char_space {
            doc_grid = doc_grid.char_space(char_space);
        }
        self.0 = self.0.doc_grid(doc_grid);
        self
    }

    pub fn text_direction(mut self, direction: &str) -> Self {
        self.0 = self.0.text_direction(direction.to_string());
        self
    }

    pub fn section_type(mut self, section_type: docx_rs::SectionType) -> Self {
        self.0 = self.0.section_type(section_type);
        self
    }

    pub fn page_num_type(mut self, page_num_type: PageNumType) -> Self {
        self.0 = self.0.page_num_type(page_num_type.take());
        self
    }
}

impl SectionProperty {
    pub fn take(self) -> docx_rs::SectionProperty {
        self.0
    }
}
