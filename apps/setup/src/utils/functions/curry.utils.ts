type Curried<F> = F extends (...args: infer A) => infer R
  ? A extends [infer First, ...infer Rest]
    ? (arg: First) => Curried<(...args: Rest) => R>
    : R
  : never;

function curry<F extends (...args: any[]) => any>(fn: F): Curried<F>;

function curry(fn: (...args: any[]) => any): any {
  return function curried(...args: any[]) {
    if (args.length >= fn.length) {
      return fn.apply(null, args);
    } else {
      return function (...args2: any[]) {
        return curried.apply(null, args.concat(args2));
      };
    }
  };
}

export { curry };
