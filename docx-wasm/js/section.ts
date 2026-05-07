import { Header } from "./header";
import { Footer } from "./footer";
import { SectionProperty } from "./section-property";

export class Section {
  property: SectionProperty = new SectionProperty();
  hasNumberings = false;

  sectionProperty(property: SectionProperty) {
    this.property = property;
    this.syncNumberings();
    return this;
  }

  header(header: Header) {
    this.property.header(header);
    this.syncNumberings();
    return this;
  }

  firstHeader(header: Header) {
    this.property.firstHeader(header);
    this.syncNumberings();
    return this;
  }

  evenHeader(header: Header) {
    this.property.evenHeader(header);
    this.syncNumberings();
    return this;
  }

  footer(footer: Footer) {
    this.property.footer(footer);
    this.syncNumberings();
    return this;
  }

  firstFooter(footer: Footer) {
    this.property.firstFooter(footer);
    this.syncNumberings();
    return this;
  }

  evenFooter(footer: Footer) {
    this.property.evenFooter(footer);
    this.syncNumberings();
    return this;
  }

  titlePg() {
    this.property.titlePg();
    return this;
  }

  pageSize(w: number, h: number) {
    this.property.pageSize(w, h);
    return this;
  }

  pageMargin(margin: Parameters<SectionProperty["pageMargin"]>[0]) {
    this.property.pageMargin(margin);
    return this;
  }

  pageOrientation(orient: Parameters<SectionProperty["pageOrientation"]>[0]) {
    this.property.pageOrientation(orient);
    return this;
  }

  docGrid(
    type: Parameters<SectionProperty["docGrid"]>[0],
    linePitch?: number,
    charSpace?: number
  ) {
    this.property.docGrid(type, linePitch, charSpace);
    return this;
  }

  sectionType(type: Parameters<SectionProperty["sectionType"]>[0]) {
    this.property.sectionType(type);
    return this;
  }

  pageTypeNum(options: Parameters<SectionProperty["pageTypeNum"]>[0]) {
    this.property.pageTypeNum(options);
    return this;
  }

  private syncNumberings() {
    this.hasNumberings = Boolean(
      this.property._header?.hasNumberings ||
        this.property._firstHeader?.hasNumberings ||
        this.property._evenHeader?.hasNumberings ||
        this.property._footer?.hasNumberings ||
        this.property._firstFooter?.hasNumberings ||
        this.property._evenFooter?.hasNumberings
    );
  }
}
