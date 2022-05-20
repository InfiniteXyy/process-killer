import { Header } from './header';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="text-dark-500 dark:text-light-300 flex h-full flex-col overflow-hidden rounded-lg border border-neutral-200 bg-neutral-50 transition-colors dark:border-neutral-800 dark:bg-neutral-900">
      <Header />
      <div className="h-full overflow-hidden">{children}</div>
    </div>
  );
}
