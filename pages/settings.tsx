import dynamic from 'next/dynamic';
import Head from 'next/head';

const Settings = dynamic(() => import('~/components/settings'), { ssr: false });

export default function SettingsPage() {
  return (
    <>
      <Head>
        <title>Process Killer - Settings</title>
      </Head>
      <Settings />
    </>
  );
}
