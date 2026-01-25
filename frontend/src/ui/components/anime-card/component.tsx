import { useState, useRef, useEffect } from "react";
import type { Anime } from "../../../domain/interfaces";
import "./style.css";
import { Button, Flex, Typography } from "antd";
import { IoIosPlay } from "react-icons/io";
import { FaAngleDown } from "react-icons/fa6";
import useNotification from "antd/es/notification/useNotification";
import { GetAnimeBySlugUseCase } from "../../../app/use-cases/get-anime-by-slug-use-case";
import { useNavigate } from "react-router-dom";
import { AnimeEntity } from "../../../domain/entity/anime-entity";

export default function AnimeCard({ anime }: { anime: Anime }) {
  const [hover, setHover] = useState(false);
  const timeoutRef = useRef<number | null>(null);
  const [notification, context] = useNotification();
  const [animeInfo, setAnimeInfo] = useState<Anime | null>(null);
  const navigate = useNavigate();

  const onEnterHover = () => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    timeoutRef.current = window.setTimeout(async () => {
      if (!anime.slug) return;
      const useCase = new GetAnimeBySlugUseCase(notification);
      const result = await useCase.execute(anime.slug);
      setAnimeInfo(result);
      setHover(true);
    }, 300);
  };

  const onLeaveHover = () => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    timeoutRef.current = window.setTimeout(() => {
      setHover(false);
    }, 100);
  };

  const playLastSeen = (anime: Anime | null) => {
    if (anime === null) return;
    const entity = new AnimeEntity(anime);
    const episode = entity.getLastEpisodeSeen();
    if (episode === undefined) {
      return;
    }
    navigate(`/anime/${anime.slug}/play/${episode?.slug}`);
  }

  return (
    <div
      className="AnimeCard"
      onMouseEnter={onEnterHover}
      onMouseLeave={onLeaveHover}
    >
      <img className="AnimeCard-cover" src={anime.cover} alt={anime.title} />

      {hover && (
        <Flex vertical className="AnimeCard-hover">
          <div className="AnimeCard-cover-container">
            <img className="AnimeCard-cover" src={anime.cover} alt={anime.title} />
            <div className="AnimeCard-cover-gradient"></div>
            <Typography.Title className="AnimeCard-hover-title" level={3}>{anime.title}</Typography.Title>
          </div>

          <Flex className="AnimeCard-hover-controls" justify="space-between">
            <Button onClick={() => playLastSeen(animeInfo)} className="AnimeCard-hover-controls-btn" type="primary" icon={<IoIosPlay size={25} />} />
            <Button className="AnimeCard-hover-controls-btn" icon={<FaAngleDown size={25} />} />
          </Flex>
        </Flex>
      )}
    </div>
  );
}
