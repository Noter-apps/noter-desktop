import { File } from "./file";
import { Id } from "./id";

export type Entry = {
  File: File | undefined;
  Directory: Directory | undefined;
};

export type Directory = {
  id: Id;
  name: string;
  created_at: Date;
  modified_at: Date;
  entries: Entry[];
};
