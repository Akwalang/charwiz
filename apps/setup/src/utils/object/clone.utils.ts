import { isDate, isArray, isPlainObject } from '../is';

const { hasOwnProperty } = Object.prototype;

const TYPES = [Number, String, Boolean];

export const clone = function clone<T>(entity: T): T {
  // null, undefined, NaN values check
  if (!entity) return entity;

  // normalizing primitives if someone did new String('aaa'), or new Number('444');
  for (let i = 0, len = TYPES.length; i < len; i++) {
    if (entity instanceof TYPES[i]) return TYPES[i](entity) as unknown as T;
  }

  if (isArray(entity)) return entity.map(value => clone(value)) as unknown as T;

  if (isDate(entity)) return new Date(+entity) as unknown as T;

  if (isPlainObject(entity)) {
    const result: Record<any, any> = {};

    for (const name in entity) {
      if (!hasOwnProperty.call(entity, name)) continue;

      result[name] = clone(entity[name]);
    }

    return result as T;
  }

  return entity;
};
