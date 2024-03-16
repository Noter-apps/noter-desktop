import {
  InitialConfigType,
  LexicalComposer,
} from "@lexical/react/LexicalComposer";
import { $convertFromMarkdownString } from "@lexical/markdown";
import { RichTextPlugin } from "@lexical/react/LexicalRichTextPlugin";
import LexicalErrorBoundary from "@lexical/react/LexicalErrorBoundary";
import { ContentEditable } from "@lexical/react/LexicalContentEditable";
import { theme } from "./theme";
import { HeadingNode, QuoteNode } from "@lexical/rich-text";
import { ListNode, ListItemNode } from "@lexical/list";
import { CodeNode, CodeHighlightNode } from "@lexical/code";
import { AutoLinkNode, LinkNode } from "@lexical/link";
import { EditorState } from "lexical";
import { useEffect, useRef, useState } from "react";
import { OnChangePlugin } from "@lexical/react/LexicalOnChangePlugin";
import PluginList from "./plugins";
import { InternalLinkNode } from "./nodes/LinkNode";
import { HorizontalRuleNode } from "@lexical/react/LexicalHorizontalRuleNode";
import { Note } from "@/types/files/note";

function AutoSavePlugin({
  onSave,
}: {
  onSave: (editorState: EditorState) => void;
}) {
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  function onChange(editorState: EditorState) {
    if (timeoutRef.current) {
      clearInterval(timeoutRef.current);
    }

    timeoutRef.current = setTimeout(() => {
      onSave(editorState);
    }, 1000);
  }

  return <OnChangePlugin ignoreSelectionChange={true} onChange={onChange} />;
}

export default function Editor({
  note,
  onSave,
}: {
  note: Note;
  onSave: (editorState: EditorState) => void;
}) {
  const initialConfig = {
    namespace: "editor",
    editable: true,
    onError(error) {
      console.log("Error", error);
    },
    editorState: (_editorState) => {
      $convertFromMarkdownString(note.body);
    },
    theme,
    nodes: [
      HeadingNode,
      ListNode,
      ListItemNode,
      QuoteNode,
      CodeNode,
      CodeHighlightNode,
      AutoLinkNode,
      LinkNode,
      InternalLinkNode,
      HorizontalRuleNode,
    ],
  } satisfies InitialConfigType;

  const [floatingAnchorElem, setFloatingAnchorElem] =
    useState<HTMLDivElement | null>(null);
  const [isSmallWidthViewport, setIsSmallWidthViewport] =
    useState<boolean>(false);
  const onRef = (_floatingAnchorElem: HTMLDivElement) => {
    if (_floatingAnchorElem !== null) {
      setFloatingAnchorElem(_floatingAnchorElem);
    }
  };

  useEffect(() => {
    const updateViewPortWidth = () => {
      const isNextSmallWidthViewport = window.matchMedia(
        "(max-width: 1025px)",
      ).matches;

      if (isNextSmallWidthViewport !== isSmallWidthViewport) {
        setIsSmallWidthViewport(isNextSmallWidthViewport);
      }
    };
    updateViewPortWidth();
    window.addEventListener("resize", updateViewPortWidth);

    return () => {
      window.removeEventListener("resize", updateViewPortWidth);
    };
  }, [isSmallWidthViewport]);

  return (
    <LexicalComposer initialConfig={initialConfig}>
      <RichTextPlugin
        placeholder={<div>"Start typing..."</div>}
        contentEditable={
          <div
            className={`w-full h-full mx-auto ${isSmallWidthViewport ? "w-full" : "lg:max-w-2xl"}`}
          >
            <div ref={onRef} className="w-full h-full">
              <ContentEditable className="border-none outline-none p-2 w-full h-full" />
            </div>
          </div>
        }
        ErrorBoundary={LexicalErrorBoundary}
      ></RichTextPlugin>

      <PluginList />
      <AutoSavePlugin onSave={onSave} />
    </LexicalComposer>
  );
}
