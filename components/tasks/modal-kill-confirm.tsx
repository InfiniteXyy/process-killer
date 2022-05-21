import { Trans, useTranslation } from 'react-i18next';
import { ITask, useKillTask } from '~/data';
import { Modal, portalSubject } from '../ui';

function KillConfirmModal(props: { task: ITask }) {
  const { task } = props;
  const { t } = useTranslation('common');
  const [visible, setVisible] = useState(true);
  const { mutateAsync: killTask } = useKillTask();

  const initialFocusRef = useRef<HTMLButtonElement | null>(null);

  return (
    <Modal
      afterClose={() => portalSubject.remove('kill-confirm-' + task.pid)}
      visible={visible}
      onClose={() => setVisible(false)}
      title={t('sure_to_kill_process_title')}
      initialFocusRef={initialFocusRef}
    >
      <div className="mt-2">
        <p className="text-sm text-gray-500">
          <Trans
            i18nKey="sure_to_kill_process"
            values={{ name: task.name }}
            components={{ b: <strong className="font-bold text-gray-700" /> }}
          />
        </p>
        <p className="mt-2 break-all text-sm italic text-gray-400">{task.exe}</p>
      </div>

      <div className="mt-4 flex justify-end space-x-4">
        <button
          type="button"
          className="inline-flex justify-center rounded-md border border-transparent bg-gray-100 px-4 py-1 text-sm font-medium text-gray-900 hover:bg-gray-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-gray-500 focus-visible:ring-offset-2"
          onClick={() => setVisible(false)}
        >
          {t('cancel')}
        </button>
        <button
          ref={initialFocusRef}
          type="button"
          className="inline-flex justify-center rounded-md border border-transparent bg-red-100 px-4 py-1 text-sm font-medium text-red-900 hover:bg-red-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-red-500 focus-visible:ring-offset-2"
          onClick={() => {
            killTask(task.pid);
            setVisible(false);
          }}
        >
          {t('confirm')}
        </button>
      </div>
    </Modal>
  );
}

export function openKillConfirm(props: { task: ITask }) {
  portalSubject.next(<KillConfirmModal {...props} />, 'kill-confirm-' + props.task.pid);
}
