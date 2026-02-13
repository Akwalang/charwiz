import { clsx, type ClassValue } from 'clsx';

import { twMerge } from 'tailwind-merge';

export const WHITESPACE = "\u00a0";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
