import { os, shell } from '@tauri-apps/api';
import { memoize } from 'lodash';
import { useQuery } from 'react-query';
import { ITask } from './use-tasks';

const getIconByPid = memoize(
  async (task: ITask): Promise<string | null> => {
    const arg = JSON.stringify([{ appOrPID: String(task.pid), size: 64, encode: true }]);
    const command = shell.Command.sidecar('vendor/file-icon', arg);
    const result = await command.execute();
    if (result.stderr) return task.parent ? getIconByPid(task.parent) : null;
    return result.stdout;
  },
  (task) => task.pid
);

export const useFileIcon = (task: ITask) => {
  return useQuery(
    ['file-icon', task.pid],
    async () => {
      const type = await os.type();
      if (type === 'Windows_NT') return;
      return await getIconByPid(task);
    },
    {
      staleTime: Infinity,
      cacheTime: Infinity,
    }
  );
};
