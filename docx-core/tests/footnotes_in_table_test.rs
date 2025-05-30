use docx_rs::*;
use std::str;

#[test]
fn test_footnotes_in_table_are_collected() {
    // 创建脚注
    let footnote1 = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("表格脚注1")));

    let footnote2 = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("表格脚注2")));

    // 创建包含脚注的表格
    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new().add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text("文本1"))
                    .add_run(Run::new().add_footnote_reference(footnote1))
            ),
            TableCell::new().add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text("文本2"))
                    .add_run(Run::new().add_footnote_reference(footnote2))
            ),
        ])
    ]);

    // 创建普通段落中的脚注
    let normal_footnote = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("普通脚注")));

    // 创建文档
    let mut docx = Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("普通段落"))
                .add_run(Run::new().add_footnote_reference(normal_footnote))
        )
        .add_table(table);

    // 收集脚注
    let has_footnotes = docx.collect_footnotes();

    // 验证脚注被收集
    assert!(has_footnotes, "应该检测到脚注");

    // 通过生成XML来验证脚注是否被正确收集
    let footnotes_xml = docx.footnotes.build();
    let footnotes_str = str::from_utf8(&footnotes_xml).unwrap();

    // 验证XML中包含我们期望的脚注内容
    assert!(footnotes_str.contains("普通脚注"), "脚注XML应该包含普通脚注内容");
    assert!(footnotes_str.contains("表格脚注1"), "脚注XML应该包含表格脚注1内容");
    assert!(footnotes_str.contains("表格脚注2"), "脚注XML应该包含表格脚注2内容");

    // 验证XML结构正确
    assert!(footnotes_str.contains("<w:footnotes"), "应该包含footnotes根元素");
    assert!(footnotes_str.contains("<w:footnote"), "应该包含footnote元素");

    println!("生成的脚注XML:");
    println!("{}", footnotes_str);
}
