import { useVirtual } from 'react-virtual';
import { useComputedTasks } from '~/data';
import { openKillConfirm } from './modal-kill-confirm';
import { useTasksStore } from './store';
import { TaskItem } from './task-item';

export function TaskList() {
  const parentRef = useRef<HTMLDivElement | null>(null);
  const tasks = useComputedTasks();
  const { keyword } = useTasksStore();
  const deferredKeyword = useDeferredValue(keyword);

  const filteredTasks = useMemo(() => {
    if (deferredKeyword.startsWith(':')) {
      const k = deferredKeyword.slice(1);
      return tasks.filter((task) => task.ports?.some((port) => port.local_port.includes(k)));
    } else {
      const k = deferredKeyword.toLowerCase();
      return tasks.filter((task) => task.name.toLowerCase().includes(k) || String(task.pid).includes(k));
    }
  }, [deferredKeyword, tasks]);

  const rowVirtualizer = useVirtual({
    size: filteredTasks.length,
    parentRef,
    estimateSize: useCallback(() => 50, []),
  });
  const { scrollToIndex } = rowVirtualizer;

  useEffect(() => {
    const onKeydownEvent = (e: KeyboardEvent) => {
      if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
        e.preventDefault();
        useTasksStore.setState(({ activeIndex }) => {
          const offset = e.key === 'ArrowUp' ? -1 : 1;
          const nextIndex = (activeIndex + offset + filteredTasks.length) % filteredTasks.length;
          scrollToIndex(nextIndex);
          return { activeIndex: nextIndex };
        });
      }
      if (e.key === 'Enter') {
        if (e.target instanceof HTMLButtonElement) return;
        e.preventDefault();
        const task = filteredTasks[useTasksStore.getState().activeIndex];
        task && openKillConfirm({ task });
      }
    };
    document.addEventListener('keydown', onKeydownEvent);
    return () => document.removeEventListener('keydown', onKeydownEvent);
  }, [filteredTasks, scrollToIndex]);

  return (
    <div
      ref={parentRef}
      className="overflow-auto px-3 py-2 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-neutral-300 dark:scrollbar-thumb-neutral-700"
    >
      <div className="relative" style={{ height: `${rowVirtualizer.totalSize}px` }}>
        {rowVirtualizer.virtualItems.map((virtualRow) => {
          const task = filteredTasks[virtualRow.index];
          return (
            <div
              ref={virtualRow.measureRef}
              className="absolute inset-0"
              key={virtualRow.index}
              style={{ height: `${virtualRow.size}px`, transform: `translateY(${virtualRow.start}px)` }}
            >
              <TaskItem task={task} index={virtualRow.index} />
            </div>
          );
        })}
      </div>
    </div>
  );
}
