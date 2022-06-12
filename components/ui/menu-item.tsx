interface MenuItemProps {
  title: string;
  right: React.ReactNode;
  onClick?: () => void;
}

export function MenuItem(props: MenuItemProps) {
  return (
    <div
      onClick={props.onClick}
      className="flex h-[40px] items-center justify-between rounded-lg bg-neutral-50 px-3 hover:bg-neutral-300 dark:bg-neutral-700 dark:active:bg-neutral-600"
    >
      <div className="font-medium opacity-90">{props.title}</div>
      {props.right}
    </div>
  );
}
