import { getType } from './get-type.utils';

const { hasOwnProperty } = Object.prototype;

export const isNumber = (value: unknown): value is number => {
  return getType(value) === 'Number';
};

export const isString = (value: unknown): value is string => {
  return getType(value) === 'String';
};

export const isFunction = (value: unknown): value is Function => {
  return getType(value) === 'Function';
};

export const isObject = (value: unknown): value is Record<any, any> => {
  return getType(value) === 'Object';
};

export const isPlainObject = (value: unknown): value is Record<any, any> => {
  return !!value && value.constructor === Object;
};

export const isArray = <T>(value: unknown | T[]): value is T[] => {
  return Array.isArray(value);
};

const Constructor = (async () => {}).constructor;

type AsyncFunction = typeof Constructor;

export const isAsyncFunction = (value: unknown): value is AsyncFunction => {
  return getType(value) === 'AsyncFunction';
};

export const isDate = (value: unknown): value is Date => {
  return value instanceof Date;
};

export const isUndefined = (value: unknown): value is undefined => {
  return value === undefined;
};

export const isNil = (value: unknown): value is undefined | null => {
  return value == null;
};

export const isNaN = (value: unknown): value is number => {
  return Number.isNaN(value);
};

export const isNull = (value: unknown): value is null => {
  return value === null;
};

export const isEmpty = (value: unknown, strait = true): boolean => {
  if (isNil(value) || value === '') return true;

  value = isObject(value) ? Object.values(value) : value;

  if (isArray(value)) {
    return strait ? !value.length : value.every(item => isEmpty(item, false));
  }

  return false;
};

export const isEqual = (a: any, b: any): boolean => {
  if (isNaN(a) && isNaN(b)) return true;
  if (isDate(a) && isDate(b)) return +a === +b;

  if (isArray(a) && isArray(b)) {
    if (a.length !== b.length) return false;

    for (let i = 0, len = a.length; i < len; i++) {
      if (!isEqual(a[i], b[i])) return false;
    }

    return true;
  }

  if (isObject(a) && isObject(b)) {
    if (Object.keys(a).length !== Object.keys(b).length) return false;

    for (const name in a) {
      if (!hasOwnProperty.call(a, name)) continue;
      if (!isEqual(a[name], b[name])) return false;
    }

    return true;
  }

  return a === b;
};
