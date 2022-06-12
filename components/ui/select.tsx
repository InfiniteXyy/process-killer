import { Listbox, Transition } from '@headlessui/react';
import { Fragment } from 'react';

export function Select<T extends string | number>(props: {
  options: { value: T; label: string }[];
  value: T;
  onChange: (value: T) => void;
}) {
  const { onChange, value, options } = props;

  return (
    <Listbox value={value} onChange={onChange}>
      <div className="flex h-full flex-1 justify-end">
        <Listbox.Button className="h-full text-right flex-1 text-sm text-neutral-500">
          <span className="block truncate">{options.find((i) => i.value === value)?.label || '--'}</span>
        </Listbox.Button>
        <Transition as={Fragment} leave="transition ease-in duration-100" leaveFrom="opacity-100" leaveTo="opacity-0">
          <Listbox.Options className="absolute z-10 mt-1 max-h-60 overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none dark:bg-neutral-800 sm:text-sm">
            {options.map((option) => (
              <Listbox.Option
                key={option.value}
                value={option.value}
                className={({ active }) =>
                  `relative cursor-default select-none p-2 px-4 ${active ? 'bg-neutral-100 dark:bg-neutral-700' : ''}`
                }
              >
                {({ selected }) => (
                  <span className={`block truncate ${selected ? 'font-medium' : 'font-normal'}`}>{option.label}</span>
                )}
              </Listbox.Option>
            ))}
          </Listbox.Options>
        </Transition>
      </div>
    </Listbox>
  );
}
