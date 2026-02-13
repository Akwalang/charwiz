export interface CacheInterface<Key, Value> {
  set(key: Key, value: Value): Value;
  get(key: Key): Value | null;
}
