import { window } from '@tauri-apps/api';
import { useTranslation } from 'react-i18next';
import { useRouter } from 'next/router';
import create from 'zustand';

const useTitleStore = create(() => ({ title: '' }));

export const useLayoutTitle = (title: string) => {
  useEffect(() => {
    useTitleStore.setState({ title });
    return () => useTitleStore.setState({ title: '' });
  }, [title]);
};

export function Header() {
  const { t, i18n } = useTranslation('common');
  const router = useRouter();
  const { title } = useTitleStore();
  const isInHomePage = router.pathname === '/';
  return (
    <header data-tauri-drag-region className="flex items-center space-x-2 py-2 pl-4 pr-2">
      <div
        className="rounded p-1 hover:bg-neutral-200 dark:hover:bg-neutral-600"
        onClick={() => (isInHomePage ? router.push('/settings') : router.back())}
      >
        <div className={isInHomePage ? 'i-[mdi-menu]' : 'i-[mdi-arrow-left]'} />
      </div>
      <nav className="pointer-events-none items-center space-x-2 overflow-hidden font-bold">
        <div className="overflow-hidden text-ellipsis whitespace-nowrap">{title || t('app_name')}</div>
      </nav>

      <div className="flex flex-shrink-0 space-x-2" style={{ marginLeft: 'auto' }}>
        <div
          className="rounded p-1 hover:bg-neutral-200 dark:hover:bg-neutral-600"
          onClick={() => window.appWindow.minimize()}
        >
          <div className="i-[mdi-window-minimize]" />
        </div>
        <div
          className="rounded p-1 hover:bg-neutral-200 hover:text-red-500 dark:hover:bg-neutral-600"
          onClick={() => window.appWindow.close()}
        >
          <div className="i-[mdi-close]" />
        </div>
      </div>
    </header>
  );
}
