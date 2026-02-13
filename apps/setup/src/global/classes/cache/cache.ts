import { type CacheInterface } from './interface';

export class Cache<Key, Value> implements CacheInterface<Key, Value> {
  private queue: Key[] = new Array();
  private store: Map<Key, Value> = new Map();

  constructor(
    private size: number = 10,
    private comparator: (a: Key, b: Key) => boolean = (a, b) => a === b,
  ) {}

  public set(key: Key, value: Value): Value {
    this.queue = this.queue.filter((k) => this.comparator(k, key)).concat(key);

    if (this.store.has(key)) return value;

    this.store.set(key, value);

    if (this.queue.length > this.size) {
      this.store.delete(this.queue.shift()!);
    }

    return value;
  }

  public get(key: Key): Value | null {
    return this.store.get(key) ?? null;
  }
}
