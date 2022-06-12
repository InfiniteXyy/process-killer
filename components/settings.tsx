import { useTranslation } from 'react-i18next';
import { useLayoutTitle } from '~/components/header';
import { MenuItem, Select } from '~/components/ui';
import { useGlobalStore } from '~/hooks';

export default function Settings() {
  const { i18n, t } = useTranslation('common');
  const { darkMode, revalidateInterval } = useGlobalStore();

  useLayoutTitle(t('setting'));

  return (
    <div className="mx-auto my-10 w-[400px] space-y-2">
      <MenuItem
        title={t('language')}
        onClick={() => useGlobalStore.setState({ locale: i18n.language === 'zh' ? 'en' : 'zh' })}
        right={<div className="text-sm text-neutral-500">{i18n.language === 'zh' ? '中文' : 'English'}</div>}
      />
      <MenuItem
        title={t('dark_mode')}
        right={
          <Select
            value={darkMode}
            onChange={(v) => useGlobalStore.setState({ darkMode: v })}
            options={[
              { label: t('dark_mode_follow_sys'), value: 'follow-system' },
              { label: t('on'), value: 'on' },
              { label: t('off'), value: 'off' },
            ]}
          />
        }
      />
      <MenuItem
        title={t('revalidate_time')}
        right={
          <Select
            value={revalidateInterval}
            onChange={(v) => useGlobalStore.setState({ revalidateInterval: v })}
            options={[
              { label: '1s', value: 1000 },
              { label: '5s', value: 5000 },
              { label: '10s', value: 10000 },
              { label: '20s', value: 20000 },
            ]}
          />
        }
      />
      <MenuItem title={t('version')} right={<div className="text-sm text-neutral-500">0.0.2</div>} />
      <div className="pt-3 text-center text-xs opacity-50">Developed by Xyy</div>
    </div>
  );
}
