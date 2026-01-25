import { useEffect, useMemo, useState } from "react";
import type { Anime, AnimeOrder, Filters, Pagination } from "../../domain/interfaces";
import { SearchAnimesByFilters } from "../use-cases/search-animes-by-filter-use-case";
import type { NotificationInstance } from "antd/es/notification/interface";

interface ParamProps {
  filters?: Filters;
  order: AnimeOrder;
  notification: NotificationInstance;
}

export default function useSearchAnimesByFilters({
  filters,
  order,
  notification,
}: ParamProps) {
  const [result, setResult] = useState<Pagination<Anime> | null>(null);
  const [loading, setLoading] = useState(false);
  const [page, setPage] = useState(1);

  const memoFilters = useMemo(() => filters, [JSON.stringify(filters)]);

  useEffect(() => {
    let mounted = true;

    const handler = async () => {
      setLoading(true);

      const useCase = new SearchAnimesByFilters(notification);
      const response = await useCase.execute(memoFilters, order, page);

      if (!mounted) return;

      if (response === null) {
        setLoading(false);
        return;
      }

      setResult((prev) => {
        if (!prev || page === 1) return response;

        return {
          ...response,
          media: [...prev.media, ...response.media],
        };
      });

      setLoading(false);
    };

    handler();

    return () => {
      mounted = false;
    };
  }, [memoFilters, page, order, notification]);

  return { result, page, setPage, loading };
}
