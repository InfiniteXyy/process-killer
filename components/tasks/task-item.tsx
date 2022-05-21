import Tippy from '@tippyjs/react';
import clsx from 'clsx';
import { uniqBy } from 'lodash';
import { memo } from 'react';
import 'tippy.js/dist/tippy.css';
import { IComputedTask, useFileIcon } from '~/data';
import { openKillConfirm } from './modal-kill-confirm';
import { useTasksStore } from './store';

interface TaskItemProps {
  task: IComputedTask;
  index: number;
}

export const TaskItem = memo(function TaskItem(props: TaskItemProps) {
  const { task, index } = props;
  const { activeIndex } = useTasksStore();
  const cpuUsage = Math.floor(task.cpu_usage * 100) / 100;
  const { data: icon } = useFileIcon(task);

  // Listened TCP, or UDP connections
  const listenedPorts = useMemo(() => {
    return uniqBy(task.ports, (i) => i.local_port);
  }, [task.ports]);

  return (
    <div
      onMouseEnter={() => useTasksStore.setState({ activeIndex: index })}
      onClick={() => openKillConfirm({ task })}
      className={clsx(
        activeIndex === index && 'bg-gray-200 active:bg-gray-300 dark:bg-gray-900 dark:active:bg-gray-800',
        'my-1 flex w-full items-center justify-between gap-2 rounded-lg p-2 outline-none'
      )}
    >
      <div className="flex items-center space-x-2 overflow-hidden">
        {icon ? (
          <div style={{ backgroundImage: `url(data:image/png;base64,${icon})` }} className="h-6 w-6 bg-contain" />
        ) : (
          <div className="grid h-6 w-6 flex-shrink-0 place-items-center rounded-lg bg-neutral-200 text-xs font-medium text-neutral-400 dark:text-neutral-700">
            {task.name[0].toUpperCase()}
          </div>
        )}

        <span>{task.name}</span>
        <small className="ml-2 text-xs opacity-30">pid: {task.pid}</small>
      </div>
      <div className="flex items-center space-x-2 overflow-hidden text-sm text-neutral-400">
        {listenedPorts?.slice(0, 4)?.map((port) => (
          <div key={port.local_addr + port.local_port} className="text-xs">
            :{port.local_port}
          </div>
        ))}
        {listenedPorts && listenedPorts.length > 4 && (
          <Tippy
            content={listenedPorts
              .slice(4)
              .map((i) => `:${i.local_port}`)
              .join(' ')}
          >
            <div className="rounded bg-slate-600 px-1 text-xs font-medium text-white">+{listenedPorts.length - 4}</div>
          </Tippy>
        )}
        {cpuUsage > 0.5 && (
          <div>
            {cpuUsage > 20 ? '🚆' : '🐏'}
            {cpuUsage}%
          </div>
        )}
      </div>
    </div>
  );
});
