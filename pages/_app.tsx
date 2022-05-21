import i18n from 'i18next';
import { ThemeProvider } from 'next-themes';
import type { AppProps } from 'next/app';
import dynamic from 'next/dynamic';
import { initReactI18next, useTranslation, withTranslation } from 'react-i18next';
import { QueryClient, QueryClientProvider } from 'react-query';
import { GLobalPortal } from '~/components/ui';
import { useGlobalStore, usePreventContextMenu } from '~/hooks';
import TranslationEn from '~/public/locales/en/common.json';
import TranslationZh from '~/public/locales/zh/common.json';
import '../styles/globals.css';

i18n.use(initReactI18next).init({
  resources: { en: { common: TranslationEn }, zh: { common: TranslationZh } },
  lng: useGlobalStore.getState().locale || 'zh',
  interpolation: {
    escapeValue: false,
  },
});

const Layout = dynamic(() => import('~/components/layout'), { ssr: false });

function App({ Component, pageProps }: AppProps) {
  const [client] = useState(() => new QueryClient({}));
  const { i18n } = useTranslation();
  const { locale } = useGlobalStore();
  usePreventContextMenu();

  useEffect(() => {
    if (locale) i18n.changeLanguage(locale);
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

export default withTranslation()(App);
