import { os, shell } from '@tauri-apps/api';
import { memoize } from 'lodash';
import { useQuery } from 'react-query';
import { defaultFileIconData, defaultWindowsFileIconData } from './default-file-icon';
import { ITask } from './use-tasks';

const macGetIconByTask = memoize(
  async (task: ITask): Promise<string> => {
    const arg = JSON.stringify([{ appOrPID: String(task.pid), size: 64, encode: true }]);
    const command = shell.Command.sidecar('vendor/file-icon', arg);
    const result = await command.execute();
    if (result.stderr) return task.parent ? macGetIconByTask(task.parent) : defaultFileIconData;
    return result.stdout;
  },
  (task) => task.pid
);

const windowsGetIconByTask = memoize(
  async (task: ITask): Promise<string> => {
    if (!task.exe) return defaultWindowsFileIconData;
    const command = shell.Command.sidecar('vendor/file-icon', [task.exe]);
    const { stderr, stdout } = await command.execute();
    if (stderr || stdout === '') return task.parent ? macGetIconByTask(task.parent) : defaultWindowsFileIconData;
    return stdout;
  },
  (task) => task.exe
);

export const useFileIcon = (task: ITask) => {
  return useQuery(
    ['file-icon', task.pid],
    async () => {
      const type = await os.type();
      if (type === 'Windows_NT') return await windowsGetIconByTask(task);
      if (type === 'Darwin') return await macGetIconByTask(task);
      return null;
    },
    {
      staleTime: Infinity,
      cacheTime: Infinity,
    }
  );
};
