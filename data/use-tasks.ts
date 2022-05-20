import { invoke } from '@tauri-apps/api';
import { useQuery } from 'react-query';

export type ITask = { pid: number; name: string; cpu_usage: number };

export const useTasks = () => {
  return useQuery('tasks', async () => {
    const rawData = await invoke<ITask[]>('get_process_list');
    return rawData;
  });
};
