use docx_rs::*;

#[test]
fn sections_have_independent_footers_and_page_numbers() {
    let footer_one = Footer::new().add_paragraph(
        Paragraph::new().add_run(Run::new().add_text("Section 1 Footer")),
    );
    let footer_two = Footer::new().add_paragraph(
        Paragraph::new().add_run(Run::new().add_text("Section 2 Footer")),
    );

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
