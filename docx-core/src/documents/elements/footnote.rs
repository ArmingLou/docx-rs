use serde::Serialize;
use std::io::Write;

use crate::documents::*;
use crate::types::VertAlignType;
use crate::xml_builder::*;
use footnote_id::generate_footnote_id;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Footnote {
    pub id: usize,
    pub content: Vec<Paragraph>,
}

impl Default for Footnote {
    fn default() -> Self {
        Footnote {
            id: 1,
            content: vec![],
        }
    }
}

impl Footnote {
    pub fn new() -> Self {
        Self {
            id: generate_footnote_id(),
            ..Default::default()
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn add_content(&mut self, p: Paragraph) -> Self {
        self.content.push(p);
        self.clone()
    }
}
impl From<&FootnoteReference> for Footnote {
    fn from(reference: &FootnoteReference) -> Self {
        Footnote {
            id: reference.id,
            content: reference.content.clone(),
        }
    }
}

impl BuildXML for Footnote {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        // To ensure docx compatible XML serialization for footnotes, we default to an empty paragraph.
        let mut footnote = self.clone();
        if self.content.is_empty() {
            footnote.add_content(Paragraph::new());
        }

        prepend_reference_marker(&mut footnote);

        XMLBuilder::from(stream)
            .open_footnote(&format!("{}", self.id))?
            .add_children(&footnote.content)?
            .close()?
            .into_inner()
    }
}

fn prepend_reference_marker(footnote: &mut Footnote) {
    if let Some(first) = footnote.content.first_mut() {
        // if let Some(run) = first.children.first() {
        //     if contains_reference_marker(run) {
        //         return;
        //     }
        // }

        let mut run = Run::new().style("FootnoteReference").set_property(
            RunProperty::new()
                .style("FootnoteReference")
                .vert_align(VertAlignType::SuperScript)
                // .size(32),
        );

        run.children.push(RunChild::FootnoteRef);

        // run.children
        //     .push(RunChild::Text(Text {
        //         text: format!("{}", footnote.id),
        //         preserve_space: true,
        //     }));

        first.children.insert(0, ParagraphChild::Run(Box::new(run)));
    }
}

fn contains_reference_marker(child: &ParagraphChild) -> bool {
    if let ParagraphChild::Run(run) = child {
        return run
            .children
            .iter()
            .any(|c| matches!(c, RunChild::FootnoteReference(_)));
    }
    false
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_footnote_build_default() {
        let b = Footnote::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:footnote w:id="1"><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr><w:vertAlign w:val="superscript" /><w:rStyle w:val="FootnoteReference" /></w:rPr><w:footnoteRef /></w:r></w:p></w:footnote>"#
        );
    }

    #[test]
    fn test_footnote_build_with_paragraph() {
        let b = Footnote::new()
            .add_content(Paragraph::new().add_run(Run::new().add_text("hello")))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:footnote w:id="1"><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr><w:vertAlign w:val="superscript" /><w:rStyle w:val="FootnoteReference" /></w:rPr><w:footnoteRef /></w:r><w:r><w:rPr /><w:t xml:space="preserve">hello</w:t></w:r></w:p></w:footnote>"#
        );
    }
}
