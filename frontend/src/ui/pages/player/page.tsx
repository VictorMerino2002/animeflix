import { useParams } from "react-router-dom";
import "./style.css";
import useEpisode from "../../../app/hooks/use-episode";
import useNotification from "antd/es/notification/useNotification";
import useAnime from "../../../app/hooks/use-anime";

export default function PlayerPage() {
  const { animeSlug, episodeSlug } = useParams();
  const [notification, context] = useNotification();
  const { episode } = useEpisode({ animeSlug, episodeSlug, notification });
  const { anime } = useAnime({ slug: animeSlug, notification });

  return (
    <div className="PlayerPage">
      <iframe src={episode?.servers[0].embed} />
    </div>
  );
}
