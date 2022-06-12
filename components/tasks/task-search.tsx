import { useRouter } from 'next/router';
import { useTranslation } from 'react-i18next';
import { useTasksStore } from './store';

export function TaskSearch() {
  const { keyword } = useTasksStore();
  const router = useRouter();
  const { t } = useTranslation('common');

  return (
    <div className="m-3 mb-1 flex items-center rounded bg-white pl-3 shadow-sm transition-all focus-within:shadow-md dark:bg-neutral-900">
      <div className="text-md i-[carbon-search] mr-2 text-neutral-400 dark:text-neutral-500" />
      <input
        value={keyword}
        onChange={(e) => useTasksStore.setState({ keyword: e.target.value })}
        className="w-full bg-transparent py-2 text-sm outline-none"
        placeholder={t('keyword_placeholder')}
        autoComplete="off"
        autoCorrect="off"
        autoCapitalize="off"
        autoFocus={true}
        spellCheck={false}
      />
      <div
        onClick={() => router.push('/settings')}
        className="mr-3 cursor-pointer rounded p-1 text-neutral-600 hover:bg-neutral-100 dark:text-neutral-300 dark:hover:bg-neutral-700"
      >
        <div className="i-[mdi-menu] rounded" />
      </div>
    </div>
  );
}
