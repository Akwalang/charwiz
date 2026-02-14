import { KeyEnum } from "../../enums/key.enum";
import { KeyInsert } from "./key-insert.type";

export type KeyItem = {
  key: KeyEnum,
  insert: KeyInsert,
};
