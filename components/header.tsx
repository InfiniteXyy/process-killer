import { window } from '@tauri-apps/api';
import clsx from 'clsx';
import { useTheme } from 'next-themes';

export function Header() {
  const { resolvedTheme, setTheme } = useTheme();
  return (
    <header data-tauri-drag-region className="flex items-center space-x-2 py-2 pl-5 pr-2">
      <nav className="pointer-events-none inline-block space-x-2 overflow-hidden font-bold">
        <div className="overflow-hidden text-ellipsis whitespace-nowrap">Process Killer</div>
      </nav>
      <div
        style={{ marginRight: 'auto' }}
        className={clsx(resolvedTheme === 'dark' ? 'i-[carbon-sun]' : 'i-[carbon-moon]')}
        onClick={() => setTheme(resolvedTheme === 'light' ? 'dark' : 'light')}
      />

      <div className="flex flex-shrink-0 space-x-2">
        <button
          className="rounded p-1 hover:bg-neutral-200 dark:hover:bg-neutral-600"
          onClick={() => window.appWindow.minimize()}
        >
          <div className="i-[mdi-window-minimize]" />
        </button>
        <button
          className="rounded p-1 hover:bg-neutral-200 hover:text-red-500 dark:hover:bg-neutral-600"
          onClick={() => window.appWindow.close()}
        >
          <div className="i-[mdi-close]" />
        </button>
      </div>
    </header>
  );
}
