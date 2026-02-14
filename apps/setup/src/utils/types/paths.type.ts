export type FullPaths<T> = {
  [K in keyof T & string]:
    T[K] extends Array<any>
      ? never
      : T[K] extends object
        ? `${K}.${FullPaths<T[K]>}`
        : K
}[keyof T & string];

export type AllPaths<T> = {
  [K in keyof T & string]:
    T[K] extends Array<any>
    ? never
    : T[K] extends object
      ? K | `${K}.${AllPaths<T[K]>}`
      : K
}[keyof T & string];

export type PathValue<T, P extends string> =
  P extends `${infer K}.${infer Rest}`
    ? K extends keyof T
      ? PathValue<T[K], Rest>
      : never
    : P extends keyof T
      ? T[P]
      : never;
