import { CommonValues } from "./common-values.type";

export type Command = CommonValues & {
  cmd: string,
};
