import create from 'zustand';
import { persist } from 'zustand/middleware';

export const useGlobalStore = create(persist(() => ({ locale: 'zh' }), { name: 'global-store', version: 0 }));
