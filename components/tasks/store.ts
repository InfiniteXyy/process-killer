import create from 'zustand';

export const useTasksStore = create(() => ({
  searchParams: {
    keyword: '',
  },
}));
