import { clone, merge } from "@/utils/object";

import { DeepPartial } from "../types";

export const mutateListItem = <T extends Record<any, any>>(
  list: T[],
  idx: number,
  mutator: (item: T) => DeepPartial<T>,
): T[] => {
  const result = [...list];

  result[idx] = merge(
    clone(result[idx]),
    mutator(result[idx]),
  );

  return result;
};
