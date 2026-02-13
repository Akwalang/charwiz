import { isFunction } from "../is";

function deduplicate<T extends {}>(
  array: T[],
  key?: keyof T,
): T[];

function deduplicate<T extends {}>(
  array: T[],
  getter?: (v: T) => {},
): T[];

function deduplicate<T extends {}>(
  array: T[],
  keyOrGetter: keyof T | ((v: T) => {}) = (v: T) => v,
): T[] {
  const seen = new Set();
  const result: T[] = [];

  const getter = isFunction(keyOrGetter) ? keyOrGetter : (v: T) => v[keyOrGetter];

  for (const item of array) {
    const value = getter(item);

    if (seen.has(value)) continue;

    seen.add(value);
    result.push(item);
  }

  return result;
};

export { deduplicate };
