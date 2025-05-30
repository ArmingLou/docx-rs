use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./footnotes_in_table.docx");
    let file = std::fs::File::create(path).unwrap();

    // 创建脚注
    let footnote1 = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("这是表格中的第一个脚注内容")));

    let footnote2 = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("这是表格中的第二个脚注内容")));

    // 第一行：普通文本
    let row1 = TableRow::new(vec![
        TableCell::new().add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("普通文本"))
        ),
        TableCell::new().add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("另一个单元格"))
        ),
    ]);

    // 第二行：包含脚注的文本
    let row2 = TableRow::new(vec![
        TableCell::new().add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("带脚注的文本"))
                .add_run(Run::new().add_footnote_reference(footnote1))
        ),
        TableCell::new().add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("另一个带脚注的文本"))
                .add_run(Run::new().add_footnote_reference(footnote2))
        ),
    ]);

    // 创建表格并设置网格
    let table = Table::new(vec![row1, row2])
        .set_grid(vec![2000, 2000]);

    // 创建文档，包含普通段落中的脚注和表格中的脚注
    let normal_footnote = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("这是普通段落中的脚注内容")));

    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("这是普通段落中的文本"))
                .add_run(Run::new().add_footnote_reference(normal_footnote))
        )
        .add_table(table)
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("表格后的段落"))
        )
        .build()
        .pack(file)?;

    println!("已创建包含表格脚注的文档: footnotes_in_table.docx");
    Ok(())
}
