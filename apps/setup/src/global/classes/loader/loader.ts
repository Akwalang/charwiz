import { CacheInterface } from "../cache";

export class Loader<T> {
  constructor(
    private cache: CacheInterface<string, T>,
    private transform: (data: Blob) => T | Promise<T> = (data) => data as T,
  ) {}

  public get(url: string): T | null {
    return this.cache.get(url) ?? null;
  }

  public async load(url: string): Promise<T> {
    let result = this.cache.get(url);

    if (!result) {
      const response = await fetch(url);

      if (!response.ok) {
        throw new Error(`Failed to load ${url}: ${response.statusText}`);
      }

      const blob = await response.blob();

      result = await this.transform(blob);

      this.cache.set(url, result);
    }

    return result;
  }
}
