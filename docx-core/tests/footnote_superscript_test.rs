use docx_rs::*;
use std::str;

#[test]
fn test_footnote_reference_has_superscript_style() {
    // 创建脚注
    let footnote = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("脚注内容")));
    
    // 创建包含脚注引用的Run
    let run = Run::new().add_footnote_reference(footnote);
    
    // 生成XML
    let xml = run.build();
    let xml_str = str::from_utf8(&xml).unwrap();
    
    println!("生成的Run XML:");
    println!("{}", xml_str);
    
    // 验证包含FootnoteReference样式
    assert!(xml_str.contains(r#"<w:rStyle w:val="FootnoteReference""#), 
           "应该包含FootnoteReference样式");
    
    // 验证包含上标设置
    assert!(xml_str.contains(r#"<w:vertAlign w:val="superscript""#), 
           "应该包含superscript垂直对齐设置");
    
    // 验证包含脚注引用元素
    assert!(xml_str.contains(r#"<w:footnoteReference"#), 
           "应该包含footnoteReference元素");
}

#[test]
fn test_footnote_in_paragraph_has_superscript() {
    // 创建脚注
    let footnote = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("脚注内容")));
    
    // 创建包含脚注的段落
    let paragraph = Paragraph::new()
        .add_run(Run::new().add_text("这是正文"))
        .add_run(Run::new().add_footnote_reference(footnote));
    
    // 生成XML
    let xml = paragraph.build();
    let xml_str = str::from_utf8(&xml).unwrap();
    
    println!("生成的段落XML:");
    println!("{}", xml_str);
    
    // 验证脚注引用具有正确的样式
    assert!(xml_str.contains(r#"<w:rStyle w:val="FootnoteReference""#), 
           "脚注引用应该有FootnoteReference样式");
    assert!(xml_str.contains(r#"<w:vertAlign w:val="superscript""#), 
           "脚注引用应该是上标样式");
}

#[test]
fn test_footnote_in_table_has_superscript() {
    // 创建脚注
    let footnote = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("表格脚注")));
    
    // 创建包含脚注的表格
    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new().add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text("表格内容"))
                    .add_run(Run::new().add_footnote_reference(footnote))
            )
        ])
    ]);
    
    // 生成XML
    let xml = table.build();
    let xml_str = str::from_utf8(&xml).unwrap();
    
    println!("生成的表格XML:");
    println!("{}", xml_str);
    
    // 验证表格中的脚注引用也具有正确的样式
    assert!(xml_str.contains(r#"<w:rStyle w:val="FootnoteReference""#), 
           "表格中的脚注引用应该有FootnoteReference样式");
    assert!(xml_str.contains(r#"<w:vertAlign w:val="superscript""#), 
           "表格中的脚注引用应该是上标样式");
}
