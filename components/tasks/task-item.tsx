import Tippy from '@tippyjs/react';
import clsx from 'clsx';
import { isEmpty, uniqBy } from 'lodash';
import { memo } from 'react';
import 'tippy.js/dist/tippy.css';
import { IComputedTask } from '~/data';
import { openKillConfirm } from './modal-kill-confirm';

export const TaskItem = memo(function TaskItem(props: { task: IComputedTask }) {
  const { task } = props;
  const [isExpandChildren, setIsExpandChildren] = useState(false);

  const cpuUsage = Math.floor(task.cpu_usage * 100) / 100;

  // Listened TCP, or UDP connections
  const listenedPorts = useMemo(() => {
    return uniqBy(
      task.ports?.filter((i) => i.tcp_state === 'LISTEN' || !i.tcp_state),
      (i) => i.local_port
    );
  }, [task.ports]);

  return (
    <>
      <div
        onClick={() => openKillConfirm({ task })}
        className={clsx(
          isExpandChildren ? 'bg-stone-200' : 'hover:bg-stone-200 dark:hover:bg-stone-900',
          'mr-2 flex items-center justify-between gap-2 rounded-lg p-2 transition'
        )}
      >
        <div className="flex items-center space-x-2 overflow-hidden">
          <div className="grid h-6 w-6 flex-shrink-0 place-items-center rounded-lg bg-neutral-300 text-xs font-medium text-neutral-500 dark:text-neutral-700">
            {task.name[0].toUpperCase()}
          </div>
          <span className="font-medium">{task.name}</span>
          <small className="ml-2 text-xs opacity-30">pid: {task.pid}</small>
          {!isEmpty(task.children) && (
            <button
              className="h-4 w-4 rounded text-neutral-400 hover:bg-neutral-300"
              onClick={(e) => {
                e.stopPropagation();
                setIsExpandChildren(!isExpandChildren);
              }}
            >
              <div className={clsx('i-[mdi-chevron-down] transition-transform', isExpandChildren && 'rotate-180')} />
            </button>
          )}
        </div>
        <div className="flex items-center space-x-2 overflow-hidden text-sm text-neutral-400">
          {listenedPorts?.slice(0, 4)?.map((port) => (
            <div key={port.local_addr}>:{port.local_port}</div>
          ))}
          {listenedPorts && listenedPorts.length > 4 && (
            <Tippy
              content={listenedPorts
                .slice(4)
                .map((i) => i.local_port)
                .join(' ')}
            >
              <div className="rounded bg-slate-600 px-1 font-bold text-white">+{listenedPorts.length - 4}</div>
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
      {!isEmpty(task.children) && isExpandChildren && (
        <div className="pl-4">
          {task.children?.map((child) => (
            <TaskItem task={child} key={child.pid} />
          ))}
        </div>
      )}
    </>
  );
});
