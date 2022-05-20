import { ITask, useKillTask } from '~/data';

export function TaskItem(props: { task: ITask }) {
  const { task } = props;

  const { mutateAsync: killTask } = useKillTask();
  return (
    <div
      onClick={() => {
        if (confirm('sure to kill')) {
          killTask(task.pid);
        }
      }}
      className="mr-2 flex items-center justify-between gap-2 rounded-lg p-2 hover:bg-neutral-200 dark:hover:bg-neutral-900"
    >
      <div className="flex space-x-2 overflow-hidden font-medium">
        <div className="flex h-6 w-6 flex-shrink-0 items-center justify-center rounded-lg bg-gray-300 text-xs text-neutral-500 dark:text-neutral-700">
          {task.name[0].toUpperCase()}
        </div>
        <div className="overflow-hidden text-ellipsis whitespace-nowrap">
          <span>{task.name}</span>
          <small className="ml-2 text-xs opacity-30">{task.pid}</small>
          <small className="ml-2 text-xs opacity-30">{task.parent_pid}</small>
        </div>
      </div>
      <div className="text-sm text-neutral-400">{Math.floor(task.cpu_usage * 100) / 100}%</div>
    </div>
  );
}
