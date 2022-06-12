import { Header } from './header';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex h-full flex-col overflow-hidden bg-neutral-100 transition-colors dark:bg-neutral-800">
      <Header />
      <div className="h-full overflow-hidden">{children}</div>
    </div>
  );
}
