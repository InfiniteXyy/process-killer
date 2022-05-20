import { ITask, useKillTask } from '~/data';
import { Modal, portalSubject } from '../ui';

function KillConfirmModal(props: { task: ITask }) {
  const { task } = props;
  const [visible, setVisible] = useState(true);
  const { mutateAsync: killTask } = useKillTask();

  return (
    <Modal
      afterClose={() => portalSubject.remove('kill-confirm-' + task.pid)}
      visible={visible}
      onClose={() => setVisible(false)}
      title="Confirm"
    >
      <div className="mt-2">
        <p className="text-sm text-stone-500">
          Are you sure to kill <span className="font-bold text-stone-700">{task.name}</span> ?
        </p>
        <p className="mt-2 text-sm italic text-stone-400">{task.exe}</p>
      </div>

      <div className="mt-4 flex justify-end">
        <button
          type="button"
          className="inline-flex justify-center rounded-md border border-transparent bg-red-100 px-4 py-2 text-sm font-medium text-red-900 hover:bg-red-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-red-500 focus-visible:ring-offset-2"
          onClick={() => {
            killTask(task.pid);
            setVisible(false);
          }}
        >
          Confirm
        </button>
      </div>
    </Modal>
  );
}

export function openKillConfirm(props: { task: ITask }) {
  portalSubject.next(<KillConfirmModal {...props} />, 'kill-confirm-' + props.task.pid);
}
