import { Typography } from "antd";
import Carousel from "../../../components/carousel/component";
import useSearchAnimesByFilters from "../../../../app/hooks/use-search-animes-by-filters";
import type { NotificationInstance } from "antd/es/notification/interface";
import AnimeCard from "../../../components/anime-card/component";
import type { Filters } from "../../../../domain/interfaces";
import { useMemo } from "react";
import useObserver from "../../../../app/hooks/use-observer";

type Props = Filters & {
  notification: NotificationInstance;
};

export default function FilteredSection({
  types,
  genres,
  statuses,
  notification,
}: Props) {

  const filters = useMemo(() => ({
    types,
    genres,
    statuses,
  }), [types, genres, statuses]);

  const { page, setPage, result, loading } = useSearchAnimesByFilters({
    notification,
    filters,
    order: "rating",
  });

  const ref = useObserver(() => {
    if (result?.hasNextPage) {
      setPage(prev => prev + 1);
    }
  }, [result, setPage]);

  return (
    <>
      <Typography.Title level={2}>{genres?.[0]}</Typography.Title>

      <Carousel>
        {result?.media.map((a) => (
          <AnimeCard key={a.slug} anime={a} />
        ))}
        <div style={{ opacity: 0 }} ref={ref}>-----</div>
      </Carousel>
    </>
  );
}
