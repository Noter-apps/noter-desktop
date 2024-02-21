import {
    InitialConfigType,
    LexicalComposer,
} from "@lexical/react/LexicalComposer";
import { $convertFromMarkdownString } from "@lexical/markdown";
import { RichTextPlugin } from "@lexical/react/LexicalRichTextPlugin";
import LexicalErrorBoundary from "@lexical/react/LexicalErrorBoundary";
import { ContentEditable } from "@lexical/react/LexicalContentEditable";
import { theme } from "./theme";
import { Note } from "@/types/note";
import { HeadingNode, QuoteNode } from "@lexical/rich-text";
import { ListNode, ListItemNode } from "@lexical/list";
import { CodeNode, CodeHighlightNode } from "@lexical/code";
import { AutoLinkNode, LinkNode } from "@lexical/link";
import { EditorState } from "lexical";
import { useRef } from "react";
import { OnChangePlugin } from "@lexical/react/LexicalOnChangePlugin";
import putFile from "@/types/commands/putFile";


function AutoSavePlugin({ onSave }: { onSave: (editorState: EditorState) => void }) {
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

export default function Editor({ note, onSave }: { note: Note, onSave: (editorState:EditorState) => void }) {
    const initialConfig = {
        namespace: "editor",
        editable: true,
        onError(error) {
            console.log("Error", error);
        },
        editorState: (editorState) => {
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
        ],
    } satisfies InitialConfigType;

    return (
        <LexicalComposer initialConfig={initialConfig}>
            <RichTextPlugin
                placeholder={<div>"Start typing..."</div>}
                contentEditable={
                    <ContentEditable className="border-none outline-none block p-2" />
                }
                ErrorBoundary={LexicalErrorBoundary}
            ></RichTextPlugin>
            <AutoSavePlugin onSave={onSave}/>
        </LexicalComposer>
    );
}
