import { EditorThemeClasses } from "lexical";

export const theme = {
  ltr: "leading-7 [&:not(:first-child)]:pt-2 w-full",
  heading: {
    h1: "scroll-m-20 text-3xl border-b font-extrabold tracking-tight lg:text-4xl",
    h2: "scroll-m-20 text-3xl font-bold tracking-tight first:mt-0",
    h3: "scroll-m-20 text-2xl font-bold tracking-tight",
    h4: "scroll-m-20 text-xl font-semibold tracking-tight",
    h5: "scroll-m-20 text-lg font-bold tracking-tight",
    h6: "scroll-m-20 text-lg font-semibold tracking-tight",
  },
  code: "relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold",
  link: "text-primary",
  list: {
    listitem: "ml-6",
    ol: "list-decimal ",
    ul: "list-disc ",
  },
  quote: "border-l-2 pl-6 italic",
  indent: "pl-6",
} satisfies EditorThemeClasses;
