import type { Anime, Episode } from "../interfaces";

export class AnimeEntity {

  anime: Anime;

  constructor(anime: Anime) {
    this.anime = anime;
  }

  getLastEpisodeSeen(): Episode | undefined {
    const lastSeenNum = this.anime.last_episode_number_seen ?? 1;

    if (!lastSeenNum || !this.anime.episodes?.length) {
      return undefined;
    }

    return this.anime.episodes.find((e) => e.number == lastSeenNum);
  }

  getNextEpisode(current: number): Episode | undefined {
    const next = current + 1;
    return this.anime.episodes?.find((e) => e.number === next);
  }

  hasNextEpisode(current: number): boolean {
    return this.getNextEpisode(current) !== undefined;
  }
}
