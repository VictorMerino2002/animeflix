import { Typography } from "antd";
import type { NotificationInstance } from "antd/es/notification/interface";
import Carousel from "../../../components/carousel/component";
import AnimeCard from "../../../components/anime-card/component";
import useOnAirAnimes from "../../../../app/hooks/use-on-air-animes";

export default function OnAirSection({ notification }: { notification: NotificationInstance }) {
  const { animes, loading } = useOnAirAnimes({ notification });
  return (
    <>
      <Typography.Title level={3}>En Emision</Typography.Title>
      <Carousel>
        {animes.map((a) => <AnimeCard anime={a} key={a.title} />)}
      </Carousel>

    </>
  )
}
