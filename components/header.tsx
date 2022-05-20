import { window } from '@tauri-apps/api';

export function Header() {
  return (
    <header data-tauri-drag-region className="flex items-center justify-between space-x-2 pl-4 pr-2 py-2">
      <nav className="pointer-events-none inline-block space-x-2 overflow-hidden font-bold">
        <div className="overflow-hidden text-ellipsis whitespace-nowrap">Process Killer</div>
      </nav>

      <div className="flex flex-shrink-0 space-x-2">
        <button className="rounded p-1 hover:bg-neutral-200" onClick={() => window.appWindow.minimize()}>
          <div className="i-[mdi-window-minimize]" />
        </button>
        <button className="rounded p-1 hover:bg-neutral-200" onClick={() => window.appWindow.toggleMaximize()}>
          <div className="i-[mdi-window-maximize]" />
        </button>
        <button className="rounded p-1 hover:bg-neutral-200 hover:text-red-500" onClick={() => window.appWindow.close()}>
          <div className="i-[mdi-close]" />
        </button>
      </div>
    </header>
  );
}
