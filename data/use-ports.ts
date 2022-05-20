import { invoke } from '@tauri-apps/api';
import { useQuery } from 'react-query';

export type IPort = {
  local_addr: string;
  local_port: string;
  pids: number[];
  tcp_state?: string;
};

export const usePorts = () => {
  return useQuery(
    'ports',
    async () => {
      const ports = await invoke<IPort[]>('get_port_list');
      return ports;
    },
    { refetchInterval: 10000 }
  );
};
