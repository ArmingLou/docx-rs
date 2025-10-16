import { Comment } from "./comment";
import { CommentEnd } from "./comment-end";
import { Paragraph } from "./paragraph";
import { Table } from "./table";
import { Run } from "./run";
import { Insert } from "./insert";
import { Delete } from "./delete";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { Hyperlink, convertHyperlinkType } from "./hyperlink";
import { setParagraphProperty } from "./paragraph-property";
import { SectionProperty as SectionPropertyClass } from "./section-property";
import { Section } from "./section";
import { Header } from "./header";
import { Footer } from "./footer";

import * as wasm from "./pkg";
import { PageNum } from "./page-num";
import { NumPages } from "./num-pages";

type Child = Paragraph | Table | Comment | Hyperlink | Section;

function buildHeaderContent(header: Header) {
  let wasmHeader = wasm.createHeader();
  header.children.forEach((child) => {
    if (child instanceof Paragraph) {
      wasmHeader = wasmHeader.add_paragraph(build(child));
    } else if (child instanceof Table) {
      wasmHeader = wasmHeader.add_table(child.build());
    }
  });
  return wasmHeader;
}

function buildFooterContent(footer: Footer) {
  let wasmFooter = wasm.createFooter();
  footer.children.forEach((child) => {
    if (child instanceof Paragraph) {
      wasmFooter = wasmFooter.add_paragraph(build(child));
    } else if (child instanceof Table) {
      wasmFooter = wasmFooter.add_table(child.build());
    }
  });
  return wasmFooter;
}

function buildSectionProperty(section: SectionPropertyClass) {
  let property = wasm.createSectionProperty();

  if (section._pageMargin) {
    const { top, left, bottom, right, header, footer, gutter } =
      section._pageMargin;
    let margin = wasm.createPageMargin();
    margin = margin.top(top).left(left).bottom(bottom).right(right);
    margin = margin.header(header).footer(footer).gutter(gutter);
    property = property.page_margin(margin);
  }

  if (section._pageSize) {
    const { w, h, orient } = section._pageSize;
    property = property.page_size(w, h);
    if (orient) {
      if (orient === "landscape") {
        property = property.page_orient(wasm.PageOrientationType.Landscape);
      } else if (orient === "portrait") {
        property = property.page_orient(wasm.PageOrientationType.Portrait);
      }
    }
  }

  if (section._titlePg) {
    property = property.title_pg();
  }

  if (section._pageTypeNum) {
    const { start, chapStyle } = section._pageTypeNum;
    const pageNumType = wasm.createPageNumType(start, chapStyle);
    property = property.page_num_type(pageNumType);
  }

  if (section._docGrid) {
    const { gridType, charSpace, linePitch } = section._docGrid;
    let type = wasm.DocGridType.Default;
    switch (gridType) {
      case "lines":
        type = wasm.DocGridType.Lines;
        break;
      case "linesAndChars":
        type = wasm.DocGridType.LinesAndChars;
        break;
      case "snapToChars":
        type = wasm.DocGridType.SnapToChars;
        break;
      default:
        break;
    }
    property = property.doc_grid(type, linePitch, charSpace);
  }

  if (section._sectionType) {
    switch (section._sectionType) {
      case "nextPage":
        property = property.section_type(wasm.SectionType.NextPage);
        break;
      case "nextColumn":
        property = property.section_type(wasm.SectionType.NextColumn);
        break;
      case "continuous":
        property = property.section_type(wasm.SectionType.Continuous);
        break;
      case "evenPage":
        property = property.section_type(wasm.SectionType.EvenPage);
        break;
      case "oddPage":
        property = property.section_type(wasm.SectionType.OddPage);
        break;
    }
  }

  return property;
}

function buildHyperlink(child: Hyperlink) {
  let hyperlink = wasm.createHyperlink(child.v, convertHyperlinkType(child));

  child.children.forEach((child) => {
    if (child instanceof Run) {
      const run = child.build();
      hyperlink = hyperlink.add_run(run);
    } else if (child instanceof Insert) {
      const insert = child.build();
      hyperlink = hyperlink.add_insert(insert);
    } else if (child instanceof Delete) {
      const del = child.build();
      hyperlink = hyperlink.add_delete(del);
    } else if (child instanceof BookmarkStart) {
      hyperlink = hyperlink.add_bookmark_start(child.id, child.name);
    } else if (child instanceof BookmarkEnd) {
      hyperlink = hyperlink.add_bookmark_end(child.id);
    } else if (child instanceof Comment) {
      hyperlink = hyperlink.add_comment_start(build(child));
    } else if (child instanceof CommentEnd) {
      hyperlink = hyperlink.add_comment_end(child.id);
    }
  });

  return hyperlink;
}

function buildSection(section: Section) {
  let wasmSection = wasm.createSection();
  const property = buildSectionProperty(section.property);
  wasmSection = wasmSection.section_property(property);

  if (section.property._header) {
    const header = buildHeaderContent(section.property._header);
    wasmSection = wasmSection.header(header);
  }

  if (section.property._firstHeader) {
    const header = buildHeaderContent(section.property._firstHeader);
    wasmSection = wasmSection.first_header(header);
  }

  if (section.property._evenHeader) {
    const header = buildHeaderContent(section.property._evenHeader);
    wasmSection = wasmSection.even_header(header);
  }

  if (section.property._footer) {
    const footer = buildFooterContent(section.property._footer);
    wasmSection = wasmSection.footer(footer);
  }

  if (section.property._firstFooter) {
    const footer = buildFooterContent(section.property._firstFooter);
    wasmSection = wasmSection.first_footer(footer);
  }

  if (section.property._evenFooter) {
    const footer = buildFooterContent(section.property._evenFooter);
    wasmSection = wasmSection.even_footer(footer);
  }

  return wasmSection;
}

function buildParagraph(child: Paragraph) {
  let paragraph = wasm.createParagraph();
  child.children.forEach((child) => {
    if (child instanceof Run) {
      const run = child.build();
      paragraph = paragraph.add_run(run);
    } else if (child instanceof Insert) {
      const insert = child.build();
      paragraph = paragraph.add_insert(insert);
    } else if (child instanceof Delete) {
      const del = child.build();
      paragraph = paragraph.add_delete(del);
    } else if (child instanceof Hyperlink) {
      paragraph = paragraph.add_hyperlink(build(child));
    } else if (child instanceof BookmarkStart) {
      paragraph = paragraph.add_bookmark_start(child.id, child.name);
    } else if (child instanceof BookmarkEnd) {
      paragraph = paragraph.add_bookmark_end(child.id);
    } else if (child instanceof Comment) {
      const comment = build(child);
      paragraph = paragraph.add_comment_start(comment as wasm.Comment);
    } else if (child instanceof CommentEnd) {
      paragraph = paragraph.add_comment_end(child.id);
    } else if (child instanceof PageNum) {
      paragraph = paragraph.add_page_num(wasm.createPageNum());
    } else if (child instanceof NumPages) {
      paragraph = paragraph.add_num_pages(wasm.createNumPages());
    }
  });

  paragraph = setParagraphProperty(paragraph, child.property);

  if (child.property.sectionProperty) {
    const sectionProp = buildSectionProperty(child.property.sectionProperty);
    paragraph = paragraph.section_property(sectionProp);
  }

  if (typeof child.property.styleId !== "undefined") {
    paragraph = paragraph.style(child.property.styleId);
  }

  if (child.property.runProperty._del) {
    paragraph = paragraph.delete(
      child.property.runProperty._del.author,
      child.property.runProperty._del.date
    );
  }

  if (child.property.runProperty._ins) {
    paragraph = paragraph.insert(
      child.property.runProperty._ins.author,
      child.property.runProperty._ins.date
    );
  }

  if (child.property.runProperty._characterSpacing != null) {
    paragraph = paragraph.character_spacing(
      child.property.runProperty._characterSpacing
    );
  }

  if (child.property.paragraphPropertyChange) {
    let change = wasm.createParagraphPropertyChange();
    change = change
      .author(child.property.paragraphPropertyChange._author)
      .date(child.property.paragraphPropertyChange._date);

    if (child.property.paragraphPropertyChange._property.numbering) {
      change = change.numbering(
        child.property.paragraphPropertyChange._property.numbering.id,
        child.property.paragraphPropertyChange._property.numbering.level
      );
    }
    // TODO: add style, indent, alignment
    paragraph = paragraph.paragraph_property_change(change);
  }

  return paragraph;
}

function buildComment(child: Comment) {
  let comment = wasm.createComment(child.id);
  child.children.forEach((c) => {
    if (c instanceof Paragraph) {
      comment = comment.add_paragraph(buildParagraph(c));
    } else if (child instanceof Table) {
      // TODO: Support later
    }
  });
  if (child._author) {
    comment = comment.author(child._author);
  }
  if (child._date) {
    comment = comment.date(child._date);
  }
  if (child._parentCommentId) {
    comment = comment.parent_comment_id(child._parentCommentId);
  }
  return comment;
}

export function build<T>(child: Child) {
  if (child instanceof Comment) {
    return buildComment(child) as T;
  } else if (child instanceof Paragraph) {
    return buildParagraph(child) as T;
  } else if (child instanceof Hyperlink) {
    return buildHyperlink(child) as T;
  } else if (child instanceof Section) {
    return buildSection(child) as T;
  }
  throw new Error(`not found builder for child: ${child}`);
}
