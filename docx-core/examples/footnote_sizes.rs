use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./footnote_sizes.docx");
    let file = std::fs::File::create(path).unwrap();
    
    // 创建不同大小的脚注
    let footnote_small = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("这是小号脚注 (6pt)")));
    
    let footnote_default = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("这是默认脚注 (8pt)")));
    
    let footnote_medium = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("这是中号脚注 (10pt)")));
    
    let footnote_large = Footnote::new()
        .add_content(Paragraph::new().add_run(Run::new().add_text("这是大号脚注 (12pt)")));
    
    // 创建文档，展示不同大小的脚注引用
    Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("脚注大小对比示例："))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("小号脚注 (6pt = 12 half-points)"))
                .add_run(Run::new().add_footnote_reference_with_size(footnote_small, 12))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("默认脚注 (8pt = 16 half-points)"))
                .add_run(Run::new().add_footnote_reference(footnote_default))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("中号脚注 (10pt = 20 half-points)"))
                .add_run(Run::new().add_footnote_reference_with_size(footnote_medium, 20))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("大号脚注 (12pt = 24 half-points)"))
                .add_run(Run::new().add_footnote_reference_with_size(footnote_large, 24))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text(""))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("字体大小说明："))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("• Word文档中字体大小以半点(half-points)为单位"))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("• 6pt = 12 half-points (较小，可能不够清晰)"))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("• 8pt = 16 half-points (默认，推荐大小)"))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("• 10pt = 20 half-points (中等，清晰可见)"))
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("• 12pt = 24 half-points (较大，非常清晰)"))
        )
        .build()
        .pack(file)?;
    
    println!("已创建脚注大小对比文档: footnote_sizes.docx");
    println!();
    println!("脚注引用大小设置说明：");
    println!("1. 默认大小：8pt (16 half-points) - 使用 add_footnote_reference()");
    println!("2. 自定义大小：使用 add_footnote_reference_with_size(footnote, size)");
    println!("   - size 参数是半点数，例如：");
    println!("   - 12 = 6pt (较小)");
    println!("   - 16 = 8pt (默认，推荐)");
    println!("   - 20 = 10pt (中等)");
    println!("   - 24 = 12pt (较大)");
    println!();
    println!("如果您觉得当前的8pt太小，建议使用10pt (20 half-points)");
    
    Ok(())
}
