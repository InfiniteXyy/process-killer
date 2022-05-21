import { useTranslation } from 'next-i18next';
import { useLayoutTitle } from '~/components/header';
import { MenuItem } from '~/components/ui';
import { useGlobalStore } from '~/hooks';

export default function Settings() {
  const { i18n, t } = useTranslation('common');

  useLayoutTitle(t('setting'));

  return (
    <div className="mx-auto my-10 w-[400px] space-y-2">
      <MenuItem
        title={t('language')}
        onClick={() => useGlobalStore.setState({ locale: i18n.language === 'zh' ? 'en' : 'zh' })}
        right={<div className="text-sm text-gray-500">{i18n.language === 'zh' ? '中文' : 'English'}</div>}
      />
      <MenuItem title={t('version')} right={<div className="text-sm text-gray-500">0.0.1</div>} />
      <div className="pt-3 text-center text-xs opacity-50">Developed by Xyy</div>
    </div>
  );
}
