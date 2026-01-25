import { useEffect, useState } from "react";
import type { Anime } from "../../domain/interfaces";
import { GetOnAirAnimesUseCase } from "../use-cases/get-on-air-animes-use-case";
import { GetAnimeBySlugUseCase } from "../use-cases/get-anime-by-slug-use-case";
import type { NotificationInstance } from "antd/es/notification/interface";

export default function useOnAirAnimes({
  notification,
}: {
  notification: NotificationInstance;
}) {
  const [onAir, setOnAir] = useState<Anime[]>([]);
  const [animes, setAnimes] = useState<Anime[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const useCase = new GetOnAirAnimesUseCase(notification);

    useCase.execute().then(result => {
      if (!result) return;
      setOnAir(result);
    });
  }, [notification]);

  useEffect(() => {
    if (onAir.length === 0) return;

    let cancelled = false;
    const useCase = new GetAnimeBySlugUseCase(notification);

    const loadAnimesInBatches = async () => {
      setLoading(true);
      setAnimes([]);

      const validAnimes = onAir.filter(anime => anime.slug);

      const handler = async (slug: string) => {
        const res = await useCase.execute(slug);
        if (res === null) return null;
        res.slug = slug;
        console.log(res);
        return res;
      }
      for (let i = 0; i < validAnimes.length; i += 10) {
        if (cancelled) break;

        const batch = validAnimes.slice(i, i + 10);


        const results = await Promise.all(
          batch.map(anime => handler(anime.slug!))
        );

        if (cancelled) break;

        setAnimes(prev =>
          prev.concat(results.filter(Boolean) as Anime[])
        );
      }

      setLoading(false);
    };

    loadAnimesInBatches();

    return () => {
      cancelled = true;
    };
  }, [onAir, notification]);

  return {
    animes,
    loading,
  };
}
