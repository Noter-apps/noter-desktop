import { HistoryPlugin } from "@lexical/react/LexicalHistoryPlugin";
import { HorizontalRulePlugin } from "@lexical/react/LexicalHorizontalRulePlugin";
import { ListPlugin } from "@lexical/react/LexicalListPlugin";
import { TabIndentationPlugin } from "@lexical/react/LexicalTabIndentationPlugin";
import InternalLinkPlugin from "./InternalLinkPlugin";
import { MarkdownShortcutsPlugin } from "./Markdown/MarkdownShortcuts";

export default function PluginList() {
  return (
    <>
      <HistoryPlugin />
      <HorizontalRulePlugin />
      <ListPlugin />
      <TabIndentationPlugin />
      <MarkdownShortcutsPlugin />
      <InternalLinkPlugin />
    </>
  );
}
