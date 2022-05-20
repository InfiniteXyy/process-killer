import { useComputedTasks } from '~/data';
import { useTasksStore } from './store';
import { TaskItem } from './task-item';

export function TaskList() {
  const tasks = useComputedTasks();
  const { keyword } = useTasksStore().searchParams;

  const filteredTasks = useMemo(() => {
    if (keyword.startsWith(':')) {
      const port = keyword.slice(1);
      return tasks.filter((task) => task.ports?.some((i) => i.local_port.includes(port)));
    } else {
      return tasks.filter((task) => task.name.includes(keyword) || String(task.pid).includes(keyword));
    }
  }, [keyword, tasks]);

  return (
    <div className="space-y-2 overflow-auto p-3 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-neutral-300 dark:scrollbar-thumb-neutral-700">
      {filteredTasks.map((i) => (
        <TaskItem key={i.pid} task={i} />
      ))}
    </div>
  );
}
