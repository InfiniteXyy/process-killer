import { useTasksStore } from './store';

export function TaskSearch() {
  const { keyword } = useTasksStore().searchParams;
  return (
    <div>
      <input
        value={keyword}
        onChange={(e) => useTasksStore.setState({ searchParams: { keyword: e.target.value } })}
        className="w-full bg-transparent px-4 pb-2 outline-none"
        placeholder="Please type in something"
      />
      <div className="h-[1px] w-full bg-neutral-200 transition dark:bg-neutral-800" />
    </div>
  );
}
