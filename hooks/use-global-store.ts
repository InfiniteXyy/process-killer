import create from 'zustand';
import { persist } from 'zustand/middleware';

export const useGlobalStore = create(
  persist(
    () => ({
      locale: 'zh',
      revalidateInterval: 5000,
      darkMode: 'follow-system' as 'on' | 'off' | 'follow-system',
    }),
    {
      name: 'global-store',
      version: 1,
    }
  )
);
