import { useEffect, useState } from "react";
import { GetAnimeBySlugUseCase } from "../use-cases/get-anime-by-slug-use-case";
import type { NotificationInstance } from "antd/es/notification/interface";
import type { Anime } from "../../domain/interfaces";

export default function useAnime({ slug, notification }: { slug?: string, notification: NotificationInstance }) {
  const [anime, setAnime] = useState<Anime | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!slug) return;
    const handler = async () => {
      const useCase = new GetAnimeBySlugUseCase(notification);
      setLoading(true);
      const result = await useCase.execute(slug);
      setLoading(false);
      setAnime(result);
    }
    handler();
  }, []);

  return { anime, loading };
}
