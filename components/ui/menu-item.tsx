interface MenuItemProps {
  title: string;
  right: React.ReactNode;
  onClick?: () => void;
}

export function MenuItem(props: MenuItemProps) {
  return (
    <div
      onClick={props.onClick}
      className="flex items-center justify-between rounded-lg bg-neutral-50 py-2 px-3 hover:bg-neutral-300 dark:bg-neutral-800 dark:active:bg-neutral-700"
    >
      <div className="font-medium opacity-90">{props.title}</div>
      {props.right}
    </div>
  );
}
