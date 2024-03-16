export type Id = string;

export function getParentId(id: Id): Id {
  return id.split("/").slice(0, -1).join("/");
}
