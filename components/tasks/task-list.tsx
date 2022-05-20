import { useTasks } from '~/data';
import { useTasksStore } from './store';
import { TaskItem } from './task-item';

export function TaskList() {
  const { data: tasks } = useTasks();
  const { keyword } = useTasksStore().searchParams;

  const filteredTasks = useMemo(() => {
    const pidSet = new Set(tasks?.map((i) => i.pid));
    return (
      tasks
        ?.filter((task) => task.name.includes(keyword) || task.pid.includes(keyword))
        .filter((i) => !!i.parent_pid && !pidSet.has(i.parent_pid))
        .sort((a, b) => (a.name > b.name ? -1 : 1)) ?? []
    );
  }, [keyword, tasks]);

  console.log(filteredTasks);

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
