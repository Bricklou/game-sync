export type Entries<T> = {
  [K in keyof T]: [K, T[K]];
}[keyof T][];

export const objectEntries = <T extends object>(obj: T): Entries<T> => {
  return Object.entries(obj) as Entries<T>;
};

export const objectKeys = <T extends object>(obj: T): (keyof T)[] => {
  return Object.keys(obj) as (keyof T)[];
};
