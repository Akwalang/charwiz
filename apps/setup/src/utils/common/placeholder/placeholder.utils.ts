import { isFunction } from '@/utils/is';
import { getter } from '@/utils/object';

export enum Types {
  Colon = 'colon',
  Curly = 'curly',
  Percent = 'percent',
  TagPercent = 'tag_percent',
}

const exps: Record<Types, () => RegExp> = {
  [Types.Colon]: () => /:([a-z0-9_.-]+)/ig,
  [Types.Curly]: () => /\{\{([a-z0-9_.-]+)}}/ig,
  [Types.Percent]: () => /%([a-z0-9_.-]+)%/ig,
  [Types.TagPercent]: () => /<%=\s*([a-z0-9_.-]+)\s*%>/ig,
};

export type ExtractResult = {
  original: string;
  value: string;
};

export const extract = (str: string, type: Types): ExtractResult[] => {
  const result: ExtractResult[] = [];
  const reg = exps[type]();

  for (const match of str.matchAll(reg)) {
    const [original, value] = match;

    result.push({ original, value });
  }

  return result;
};

export const replace = (
  str: string,
  type: Types,
  data: Record<string, any>,
): string => {
  const reg = exps[type]();

  return str.replace(reg, (match, key) => {
    const value = getter(data, key);

    return isFunction(value) ? value() : value ?? match;
  });
};
