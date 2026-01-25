import { useEffect, useState } from "react";
import type { Episode } from "../../domain/interfaces";
import { GetEpisodeBySlugUseCase } from "../use-cases/get-episode-by-slug-use-case";
import type { NotificationInstance } from "antd/es/notification/interface";

export default function useEpisode({ animeSlug, episodeSlug, notification }: { animeSlug?: string, episodeSlug?: string, notification: NotificationInstance }) {
  const [episode, setEspisode] = useState<Episode | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!episodeSlug || !animeSlug) return;
    const handler = async () => {
      const useCase = new GetEpisodeBySlugUseCase(notification);
      setLoading(true);
      const result = await useCase.execute(animeSlug, episodeSlug);
      setEspisode(result);
      setLoading(false);
    }
    handler();
  }, [animeSlug, episodeSlug]);

  return { episode, loading };
}
