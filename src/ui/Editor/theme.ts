import { EditorThemeClasses } from "lexical";

export const theme = {
  heading: {
    h1: "mt-6 scroll-m-20 text-3xl border-b font-extrabold tracking-tight lg:text-4xl",
    h2: "mt-6 scroll-m-20 text-3xl font-bold tracking-tight first:mt-0",
    h3: "mt-6 scroll-m-20 text-2xl font-bold tracking-tight",
    h4: "mt-6 scroll-m-20 text-xl font-semibold tracking-tight",
    h5: "mt-6 scroll-m-20 text-lg font-bold tracking-tight",
    h6: "mt-6 scroll-m-20 text-lg font-semibold tracking-tight",
  },
  text: {
    italic: "text-italic",
    bold: "font-bold",
    code: "relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold",
    base: "text-base",
    highlight: "bg-primary text-primary-foreground",
    subscript: "text-xs align-sub",
    underline: "underline",
    superscript: "text-xs align-super",
    strikethrough: "line-through",
    underlineStrikethrough: "line-through underline",
  },
  link: "text-primary",
  list: {
    listitem: "ml-6",
    ol: "list-decimal ",
    ul: "list-disc ",
  },
  quote: "border-l-2 pl-6 italic",
  indent: "pl-6",
} satisfies EditorThemeClasses;
