import { type Comparator, isComparator, DEFAULT_COMPARATOR } from './common';

export const intersection = function intersection<T>(comparator: Comparator<T> | T[], ...arrays: T[][]) {
  const items = isComparator(comparator) ? arrays : [comparator, ...arrays] as T[][];
  const compare = isComparator(comparator) ? comparator : DEFAULT_COMPARATOR;

  return items[0].filter((a: T) => {
    for (let i = 1, len = items.length; i < len; i++) {
      const item = items[i].find(b => compare(a, b));

      if (!item) return false;
    }

    return true;
  });
};
