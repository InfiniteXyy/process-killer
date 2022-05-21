import { useTasksStore } from './store';

export function TaskSearch() {
  const { keyword } = useTasksStore();
  return (
    <div>
      <input
        value={keyword}
        onChange={(e) => useTasksStore.setState({ keyword: e.target.value })}
        className="w-full bg-transparent px-5 pt-1 pb-2 outline-none"
        placeholder="Keywords / Pid / Ports by starting with ':'"
        autoComplete="off"
        autoCorrect="off"
        autoCapitalize="off"
        spellCheck={false}
      />
      <div className="h-[1px] w-full bg-neutral-200 transition dark:bg-neutral-800" />
    </div>
  );
}
