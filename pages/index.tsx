import Head from 'next/head';
import dynamic from 'next/dynamic';

const App = dynamic(() => import('~/components/app'), { ssr: false });

export default function IndexPage() {
  return (
    <>
      <Head>
        <title>Process Killer</title>
      </Head>
      <App />
    </>
  );
}
