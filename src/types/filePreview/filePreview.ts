import { FileType } from "../files/file";
import { Id } from "../id";

export type FilePreview = {
  id: Id;
  name: string;
  fileType: FileType;
  created_at: Date;
  modified_at: Date;
};
