import { TaskList, TaskSearch } from './tasks';

export default function App() {
  return (
    <div className="flex h-full flex-col overflow-hidden">
      <TaskSearch />
      <div className="flex h-full flex-col overflow-hidden">
        <TaskList />
      </div>
    </div>
  );
}
