import {
  isObject,
  isArray,
  isEqual,
  isEqualWith,
  includes,
  omitBy,
} from 'lodash';
import { JsonData } from './common';

/**
 * Checks if the input object or array is empty.
 * An object is considered empty if all values are either undefined, empty objects, or arrays.
 * For example, `{ value: undefined, obj: {} }`.
 * An empty array this an array that is empty or only contains empty objects,
 * e.g. `[{ value: undefined}]`.
 */
const objectOrArrayIsEmpty = (obj: JsonData | undefined): boolean => {
  if (obj === undefined || obj === null) return true;
  if (typeof obj !== 'object') return false;

  // array
  if (Array.isArray(obj)) {
    if (obj.length === 0) return true;
    return obj.every(it => objectOrArrayIsEmpty(it));
  }

  // object
  if (Object.keys(obj).length === 0) return true;
  const allValuesEmpty = Object.values(obj).every(it => {
    if (typeof it === 'object') return objectOrArrayIsEmpty(it);
    return it === undefined;
  });
  if (allValuesEmpty) {
    return true;
  }
  return false;
};

/**
 * Recursively removes all empty data which has been added to the newData compared to the old
 * input object.
 *
 * For example, given:
 *
 * old: { some: "value" }
 * newData: { some: "value", obj: { add1: undefined }, array: [], add2: undefined}
 *
 * stripEmptyAdditions(old, newData) will return { some: "value" }.
 */
export const stripEmptyAdditions = (
  old: JsonData | undefined,
  newData: JsonData | undefined
): JsonData | undefined => {
  if (newData === undefined) return undefined;
  if (!isObject(newData)) return newData;

  if (isObject(newData) && !isArray(newData)) {
    const object: JsonData = {};
    const oldObj = !old || !isObject(old) || isArray(old) ? {} : old;

    for (const key of Object.keys(newData)) {
      let n = newData[key];
      if (isObject(n) && !isArray(n)) {
        const o = oldObj[key];
        n = stripEmptyAdditions(o, n);
        if (objectOrArrayIsEmpty(n)) {
          if (isEqual(o, {}) || isEqual(o, [])) {
            // keep existing empty object
            object[key] = o;
          }
          // ignore the empty addition
          continue;
        }
      }
      if (n !== undefined) object[key] = n;
    }
    if (Object.keys(object).length === 0) return undefined;
    return object;
  }

  return newData;
};

// https://stackoverflow.com/questions/57874879
const isEqualIgnoreUndefined = (
  a: JsonData | undefined,
  b: JsonData | undefined
) => {
  const comparisonFunc = (a: JsonData | undefined, b: JsonData | undefined) => {
    if (isArray(a) || isArray(b)) return;
    if (!isObject(a) || !isObject(b)) return;

    if (!includes(a, undefined) && !includes(b, undefined)) return;

    // Call recursively, after filtering all undefined properties
    return isEqualWith(
      omitBy(a, value => value === undefined),
      omitBy(b, value => value === undefined),
      comparisonFunc
    );
  };
  return isEqualWith(a, b, comparisonFunc);
};

export const isEqualIgnoreUndefinedAndEmpty = (
  old: JsonData | undefined,
  newData: JsonData | undefined
) => {
  const stripped = stripEmptyAdditions(old, newData);
  // ignore undefined (e.g. in array objects which haven't been stripped) when comparing
  return isEqualIgnoreUndefined(old, stripped);
};
