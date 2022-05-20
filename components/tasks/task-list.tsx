import { useTasks } from '~/data';
import { useTasksStore } from './store';
import { TaskItem } from './task-item';

export function TaskList() {
  const { data: tasks } = useTasks();

  const { keyword } = useTasksStore().searchParams;

  const filteredTasks = useMemo(() => {
    if (!keyword || !tasks) return tasks || [];
    return tasks.filter((task) => task.name.includes(keyword));
  }, [keyword, tasks]);

  return (
    <ul className="overflow-auto p-2 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-neutral-300 dark:scrollbar-thumb-neutral-700">
      {filteredTasks.map((i) => (
        <li key={i.pid}>
          <TaskItem task={i} />
        </li>
      ))}
    </ul>
  );
}
