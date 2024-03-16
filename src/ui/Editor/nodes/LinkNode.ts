import { Id } from "@/types/id";
import {
  EditorConfig,
  LexicalEditor,
  NodeKey,
  TextNode,
  createCommand,
} from "lexical";

export class InternalLinkNode extends TextNode {
  __fileId: Id;

  constructor(text: string, fileId: Id, key?: NodeKey) {
    super(text, key);
    this.__fileId = fileId;
  }

  setFileId(fileId: Id) {
    const self = this.getWritable();
    self.__fileId = fileId;
  }

  getFileId() {
    const self = this.getLatest();
    return self.__fileId;
  }

  createDOM(
    config: EditorConfig,
    _editor?: LexicalEditor | undefined,
  ): HTMLElement {
    const dom = super.createDOM(config);
    dom.classList.add("bg-primary underline");
    return dom;
  }

  updateDOM(
    prevNode: TextNode,
    dom: HTMLElement,
    config: EditorConfig,
  ): boolean {
    return super.updateDOM(prevNode, dom, config);
  }

  static getType() {
    return "internal-link";
  }

  static clone(node: InternalLinkNode) {
    return new InternalLinkNode(node.__text, node.__fileId, node.__key);
  }
}

export function $isInternalLinkNode(node: TextNode): node is InternalLinkNode {
  return node instanceof InternalLinkNode;
}

export function $createInternalLinkNode(text: string, fileId: Id) {
  return new InternalLinkNode(text, fileId);
}

export const INSERT_INTERNAL_LINK_NODE = createCommand("insertInternalLink");
