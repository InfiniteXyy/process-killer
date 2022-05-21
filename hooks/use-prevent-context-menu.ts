export const usePreventContextMenu = () => {
  useEffect(() => {
    if (process.env.NODE_ENV === 'development') return;
    const onContextMenuClick = (e: MouseEvent) => e.preventDefault();
    document.addEventListener('contextmenu', onContextMenuClick);
    return () => document.removeEventListener('contextmenu', onContextMenuClick);
  }, []);
};
