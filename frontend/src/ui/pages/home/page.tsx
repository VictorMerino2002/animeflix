import { Flex } from "antd";
import useNotification from "antd/es/notification/useNotification";
import Grid from "../../components/grid/component";
import AnimeCard from "../../components/anime-card/component";
import NavBar from "../../components/navbar/component";
import "./style.css";
import useSearchAnime from "../../../app/hooks/use-search-anime";
import useObserver from "../../../app/hooks/use-observer";
import KeepWatchingSection from "./sections/keep-watching";
import OnAirSection from "./sections/on-air";
import FilteredSection from "./sections/filtered";
import { GENRES } from "../../../domain/interfaces/index.ts";
import { useMemo, useState, useEffect } from "react";

export default function HomePage() {
  const [notification, context] = useNotification();

  const { searchValue, setSearchValue, setPage, result, loading } = useSearchAnime({ notification });

  const cardWidth = "210px";

  const [genrePage, setGenrePage] = useState(1);
  const genresPerPage = 5;

  const [loadingGenres, setLoadingGenres] = useState(false);

  const visibleGenres = useMemo(() => {
    return GENRES.slice(0, genrePage * genresPerPage);
  }, [genrePage]);

  useEffect(() => {
    setLoadingGenres(false);
  }, [genrePage]);

  const genreLoadMoreRef = useObserver(() => {
    if (loadingGenres) return;

    if (genrePage * genresPerPage < GENRES.length) {
      setLoadingGenres(true);
      setGenrePage(prev => prev + 1);
    }
  }, [genrePage, loadingGenres]);

  const searcLoadMoreRef = useObserver(() => {
    if (result?.hasNextPage && !loading) {
      setPage(prev => prev + 1);
    }
  }, [result]);

  return (
    <>
      <Flex className="HomePage" vertical>
        <NavBar searchValue={searchValue} setSearchValue={setSearchValue} />
        {context}

        {result !== null ? (
          <Flex vertical className="HomePage-search-results">
            <Grid wrap width={cardWidth} gap={10}>
              {result.media.map((a) => <AnimeCard anime={a} key={a.title} />)}
            </Grid>

            <div ref={searcLoadMoreRef} style={{ height: 1 }} />
          </Flex>
        ) : (
          <Flex vertical className="HomePage-sections">
            <KeepWatchingSection notification={notification} />
            <OnAirSection notification={notification} />

            {visibleGenres.map((genre) => (
              <FilteredSection
                genres={[genre]}
                key={genre}
                notification={notification}
              />
            ))}

            <div ref={genreLoadMoreRef} style={{ height: 1 }} />
          </Flex>
        )}
      </Flex>
    </>
  );
}
