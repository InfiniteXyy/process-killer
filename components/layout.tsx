import { Header } from './header';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex h-full flex-col overflow-hidden rounded-lg border border-neutral-200 bg-gray-100 transition-colors dark:border-neutral-800 dark:bg-neutral-800">
      <Header />
      <div className="h-full overflow-hidden">{children}</div>
    </div>
  );
}
