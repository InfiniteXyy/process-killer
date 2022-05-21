import { ThemeProvider } from 'next-themes';
import type { AppProps } from 'next/app';
import dynamic from 'next/dynamic';
import { QueryClient, QueryClientProvider } from 'react-query';
import { GLobalPortal } from '~/components/ui';
import { useIsServer } from '~/hooks';
import '../styles/globals.css';

const Layout = dynamic(() => import('~/components/layout'), { ssr: false });

export default function App({ Component, pageProps }: AppProps) {
  const isServer = useIsServer();
  const [client] = useState(() => new QueryClient({}));

  useEffect(() => {
    if (process.env.NODE_ENV === 'development') return;
    const onContextMenuClick = (e: MouseEvent) => e.preventDefault();
    document.addEventListener('contextmenu', onContextMenuClick);
    return () => document.removeEventListener('contextmenu', onContextMenuClick);
  }, []);

  if (isServer) return null;
  return (
    <ThemeProvider enableSystem={true} attribute="class">
      <QueryClientProvider client={client}>
        <Layout>
          <GLobalPortal />
          <Component {...pageProps} />
        </Layout>
      </QueryClientProvider>
    </ThemeProvider>
  );
}
