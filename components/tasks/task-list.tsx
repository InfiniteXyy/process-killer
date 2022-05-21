import { useVirtual } from 'react-virtual';
import { useComputedTasks } from '~/data';
import { useTasksStore } from './store';
import { TaskItem } from './task-item';

export function TaskList() {
  const parentRef = useRef<HTMLDivElement | null>(null);
  const tasks = useComputedTasks();
  const { keyword } = useTasksStore();
  const deferredKeyword = useDeferredValue(keyword);

  const filteredTasks = useMemo(() => {
    if (deferredKeyword.startsWith(':')) {
      const port = deferredKeyword.slice(1);
      return tasks.filter((task) => task.ports?.some((i) => i.local_port.includes(port)));
    } else {
      return tasks.filter((task) => task.name.includes(deferredKeyword) || String(task.pid).includes(deferredKeyword));
    }
  }, [deferredKeyword, tasks]);

  const rowVirtualizer = useVirtual({
    size: filteredTasks.length,
    parentRef,
    estimateSize: useCallback(() => 40, []),
  });
  const { scrollToIndex } = rowVirtualizer;

  useEffect(() => {
    const onKeydownEvent = (e: KeyboardEvent) => {
      if (e.key === 'ArrowUp') {
        e.preventDefault();
        useTasksStore.setState((s) => {
          const nextIndex = Math.max(s.activeIndex - 1, 0);
          scrollToIndex(nextIndex);
          return { ...s, activeIndex: nextIndex };
        });
      }
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        useTasksStore.setState((s) => {
          const nextIndex = Math.min(s.activeIndex + 1, filteredTasks.length - 1);
          scrollToIndex(nextIndex);
          return { ...s, activeIndex: nextIndex };
        });
      }
    };
    document.addEventListener('keydown', onKeydownEvent);
    return () => document.removeEventListener('keydown', onKeydownEvent);
  }, [filteredTasks.length, scrollToIndex]);

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
