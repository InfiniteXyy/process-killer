import create from 'zustand';

export const useTasksStore = create(() => ({
  keyword: '',
  activeIndex: 0,
}));
