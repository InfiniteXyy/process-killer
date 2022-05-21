import { appWithTranslation, useTranslation } from 'next-i18next';
import { ThemeProvider } from 'next-themes';
import type { AppProps } from 'next/app';
import dynamic from 'next/dynamic';
import { QueryClient, QueryClientProvider } from 'react-query';
import { GLobalPortal } from '~/components/ui';
import { useGlobalStore, usePreventContextMenu } from '~/hooks';
import '../styles/globals.css';

const Layout = dynamic(() => import('~/components/layout'), { ssr: false });

function App({ Component, pageProps }: AppProps) {
  const [client] = useState(() => new QueryClient({}));
  const { i18n } = useTranslation();
  const { locale } = useGlobalStore();
  usePreventContextMenu();
  
  useEffect(() => {
    if (locale) {
      i18n.changeLanguage(locale);
    }
  }, [i18n, locale]);

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

export default appWithTranslation(App);
