import { invoke } from '@tauri-apps/api';
import { useQuery } from 'react-query';
import { IPort, usePorts } from './use-ports';

export interface ITask {
  pid: number;
  name: string;
  cpu_usage: number;
  parent_pid?: number;
  exe: string;
  children?: ITask[];
}

interface WithPort extends ITask {
  ports?: IPort[];
}

export interface IComputedTask extends WithPort {}

export const useTasks = () => {
  return useQuery(
    'tasks',
    async () => {
      const tasks = await invoke<ITask[]>('get_process_list');
      return tasks.sort((a, b) => {
        if (a.name === b.name) return a.pid < b.pid ? -1 : 1;
        return a.name > b.name ? -1 : 1;
      });
    },
    { refetchInterval: 10000 }
  );
};

export const useComputedTasks = () => {
  const { data: ports } = usePorts();
  const { data: tasks = [] } = useTasks();
  return useMemo(() => {
    const result: IComputedTask[] = tasks.map((task) => ({
      ...task,
      ports: ports?.filter((port) => port.pids.includes(task.pid)),
    }));
    return result;
  }, [ports, tasks]);
};
