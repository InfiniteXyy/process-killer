import { entries, omit } from 'lodash';
import { FC, Fragment } from 'react';
import create from 'zustand';

type PortalElements = Record<string, React.ReactNode>;
type UseGlobalPortalState = {
  elements: PortalElements;
};
const useGlobalPortalState = create<UseGlobalPortalState>(() => ({ elements: {} }));

export const portalSubject = {
  next: (node: React.ReactNode, key = '_default') => {
    useGlobalPortalState.setState((v) => ({ elements: { ...v.elements, [key]: node } }));
  },
  remove: (key = '_default') => {
    useGlobalPortalState.setState((v) => ({ elements: omit(v.elements, key) }));
  },
};

export const GLobalPortal: FC = () => {
  const { elements } = useGlobalPortalState();

  return (
    <>
      {entries(elements).map(([key, element]) => (
        <Fragment key={key}>{element}</Fragment>
      ))}
    </>
  );
};
