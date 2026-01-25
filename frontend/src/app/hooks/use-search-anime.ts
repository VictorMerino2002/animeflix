import { useEffect, useState } from "react";
import { SearchAnimeUseCase } from "../use-cases/search-anime-use-case";
import type { NotificationInstance } from "antd/es/notification/interface";
import type { Anime, Pagination } from "../../domain/interfaces";

interface parampProps {
  notification: NotificationInstance
}

export default function useSearchAnime({ notification }: parampProps) {
  const [searchValue, setSearchValue] = useState("");
  const [page, setPage] = useState(1);
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<Pagination<Anime> | null>(null);

  useEffect(() => {
    if (!searchValue) {
      setResult(null);
      setPage(1);
      return;
    }
    setPage(1);
  }, [searchValue]);

  useEffect(() => {
    if (!searchValue) return;

    const timeoutId = setTimeout(async () => {
      setLoading(true);

      const useCase = new SearchAnimeUseCase(notification);
      const response = await useCase.execute(searchValue, page);

      if (!response) {
        setLoading(false);
        return;
      }

      setResult(prev => {
        if (!prev || page === 1) return response;

        return {
          ...response,
          media: [...prev.media, ...response.media]
        };
      });

      setLoading(false);
    }, 500);

    return () => clearTimeout(timeoutId);
  }, [searchValue, page]);

  return { searchValue, setSearchValue, page, setPage, result, loading };
}
