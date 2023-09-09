export function debounce<A = unknown, R = unknown>(
  fn: (...args: A[]) => R,
  delay: number,
) {
  let timeout: number | undefined;

  return (...args: A[]) => {
    if (timeout) {
      clearTimeout(timeout);
    }

    timeout = window.setTimeout(() => {
      fn(...args);
    }, delay);
  };
}
