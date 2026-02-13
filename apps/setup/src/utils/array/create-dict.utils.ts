import { isFunction } from "../is";

type Item = Record<string, any>;

function createDict<T extends Item, K extends keyof T>(
  array: T[],
  key: K
): Map<T[K], T>;

function createDict<T extends Item, V>(
  array: T[],
  getter: (item: T) => V
): Map<V, T>;

function createDict<T extends Item, K extends keyof T, V>(
  array: T[],
  keyOrGetter: K | ((item: T) => V)
): Map<K extends keyof T ? T[K] : V, T> {
  const result = new Map<K extends keyof T ? T[K] : V, T>();

  const getter = isFunction(keyOrGetter)
    ? keyOrGetter as ((item: T) => V)
    : ((item: T) => item[keyOrGetter as T[keyof T]]);

  for (const item of array) {
    const key = (getter as Function)(item);

    !!key && result.set(key, item);
  }

  return result;
}

export { createDict };
