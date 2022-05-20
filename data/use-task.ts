import { invoke } from '@tauri-apps/api';
import { useMutation, useQueryClient } from 'react-query';

export const useKillTask = () => {
  const client = useQueryClient();
  return useMutation(
    async (pid: number) => {
      await invoke('kill_process', { pid });
    },
    {
      onSuccess: () => client.invalidateQueries('tasks'),
    }
  );
};
