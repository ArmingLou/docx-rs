use docx_rs::*;

#[test]
fn sections_have_independent_footers_and_page_numbers() {
    let footer_one = Footer::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Section 1 Footer")));
    let footer_two = Footer::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Section 2 Footer")));

    let section_two = Section::new()
        .footer(footer_two.clone())
        .page_num_type(PageNumType::new().start(1));

    let docx = Docx::new()
        .footer(footer_one.clone())
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("First section")))
        .add_section(section_two)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Second section")))
        .build();

    let document_xml = String::from_utf8(docx.document.clone()).unwrap();
    assert!(document_xml.contains(r#"r:id="rIdFooter1""#));
    assert!(document_xml.contains(r#"r:id="rIdFooter2""#));
    assert!(document_xml.contains(r#"<w:pgNumType w:start="1""#));

    assert_eq!(docx.footers.len(), 2);
    let footer_texts: Vec<String> = docx
        .footers
        .iter()
        .map(|bytes| String::from_utf8(bytes.clone()).unwrap())
        .collect();
    assert!(footer_texts[0].contains("Section 1 Footer"));
    assert!(footer_texts[1].contains("Section 2 Footer"));

    let rels = String::from_utf8(docx.document_rels.clone()).unwrap();
    assert!(rels.contains(r#"Id="rIdFooter1""#));
    assert!(rels.contains(r#"Id="rIdFooter2""#));
}

#[test]
fn first_section_next_page_does_not_insert_leading_break() {
    let docx = Docx::new()
        .add_section(Section::new().section_type(SectionType::NextPage))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("First section content")))
        .add_section(Section::new())
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Second section content")))
        .build();

    let xml = String::from_utf8(docx.document.clone()).unwrap();
    let first_section_pos = xml
        .find("First section content")
        .expect("first section text to exist");

    assert!(
        !xml[..first_section_pos].contains("<w:pPr><w:sectPr"),
        "unexpected section break before first section content",
    );

    let break_pos = xml
        .find("<w:pPr><w:sectPr")
        .expect("section break to be present after first section content");
    assert!(break_pos > first_section_pos);

    assert!(
        xml.contains(r#"<w:type w:val="nextPage""#),
        "expected next page section break type",
    );
}
