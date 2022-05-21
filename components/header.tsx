import { window } from '@tauri-apps/api';

export function Header() {
  return (
    <header data-tauri-drag-region className="flex items-center space-x-2 py-2 pl-5 pr-2">
      <nav
        className="pointer-events-none inline-block space-x-2 overflow-hidden font-bold"
        style={{ marginRight: 'auto' }}
      >
        <div className="overflow-hidden text-ellipsis whitespace-nowrap">Process Killer</div>
      </nav>

      <div className="flex flex-shrink-0 space-x-2">
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
