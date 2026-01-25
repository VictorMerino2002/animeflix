import { useEffect, useState } from "react";
import { GetAnimeHistoryUseCase } from "../use-cases/get-anime-history-use-case";
import type { NotificationInstance } from "antd/es/notification/interface";
import type { Anime } from "../../domain/interfaces";

export default function useAnimeHistory({ notification }: { notification: NotificationInstance }) {
  const [animes, setAnimes] = useState<Anime[] | null>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const useCase = new GetAnimeHistoryUseCase(notification);
    const handler = async () => {
      setLoading(true);
      const result = await useCase.execute();
      setAnimes(result);
      setLoading(false)
    }
    handler();
  }, []);

  return { animes, loading };
}
