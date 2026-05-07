import { ParagraphJSON, BookmarkStartJSON, BookmarkEndJSON } from "./paragraph";
import { TableJSON } from "./table";
import { SectionPropertyJSON } from "./section-property";
import { CommentRangeStartJSON, CommentRangeEndJSON } from "..";
import { StructuredTagJSON } from "./structured-data-tag";

export type DocumentChildJSON =
  | ParagraphJSON
  | TableJSON
  | CommentRangeStartJSON
  | CommentRangeEndJSON
  | BookmarkStartJSON
  | BookmarkEndJSON
  | StructuredTagJSON
  | SectionJSON;

export type SectionJSON = {
  type: "section";
  data: {
    property: SectionPropertyJSON;
  };
};

export type DocumentJSON = {
  children: DocumentChildJSON[];
  sectionProperty: SectionPropertyJSON;
  hasNumbering: boolean;
};
