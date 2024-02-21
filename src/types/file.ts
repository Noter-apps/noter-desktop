import { Id } from "./id";
import { Image } from "./image";
import { Note } from "./note";
import { Table } from "./table";
import { TodoList } from "./todo-list";

export type FileType = "Note" | "TodoList" | "Image" | "Table";

export type FileContent = {
    Note: Note | undefined;
    TodoList: TodoList | undefined;
    Image: Image | undefined;
    Table: Table | undefined;
};

export type File = {
    id: Id;
    name: string;
    created_at: Date;
    modified_at: Date;
    content: FileContent;
};
