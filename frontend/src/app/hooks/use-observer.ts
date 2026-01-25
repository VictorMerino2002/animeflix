import { useEffect, useRef } from "react";

export default function useObserver(onIntersect: () => void, deps: any[] = []) {
  const loadMoreRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const element = loadMoreRef.current;
    if (!element) return;

    const observer = new IntersectionObserver((entries) => {
      if (entries[0].isIntersecting) {
        onIntersect();
      }
    });

    observer.observe(element);

    return () => {
      observer.unobserve(element);
    };
  }, deps);

  return loadMoreRef;
}
