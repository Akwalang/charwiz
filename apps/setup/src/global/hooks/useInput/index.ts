import { useCallback } from 'react';

export const useInput = (callback: (value: string) => void) => {
  return useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = event.target;

    callback(value);
  }, [callback]);
}
