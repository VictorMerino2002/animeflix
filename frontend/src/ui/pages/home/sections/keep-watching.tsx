import { Typography } from "antd";
import Carousel from "../../../components/carousel/component";
import AnimeCard from "../../../components/anime-card/component";
import useAnimeHistory from "../../../../app/hooks/use-anime-history";
import type { NotificationInstance } from "antd/es/notification/interface";

export default function KeepWatchingSection({ notification }: { notification: NotificationInstance }) {

  const { animes, loading } = useAnimeHistory({ notification });
  return (
    <>
      <Typography.Title level={3}>Seguir Viendo</Typography.Title>
      <Carousel>
        {animes?.map((a) => <AnimeCard anime={a} key={a.title} />)}
      </Carousel>
    </>

  )
}
