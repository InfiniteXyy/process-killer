import { GetStaticProps } from 'next';
import { serverSideTranslations } from 'next-i18next/serverSideTranslations';
import dynamic from 'next/dynamic';

export const getStaticProps: GetStaticProps = async ({ locale = 'zh' }) => {

  return {
    props: {
      ...(await serverSideTranslations(locale, ['common'])),
    },
  };
};

const Settings = dynamic(() => import('~/components/settings'), { ssr: false });

export default function SettingsPage() {
  return <Settings />;
}
