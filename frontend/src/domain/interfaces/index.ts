export interface ApiResponse<T> {
  success: boolean,
  payload: T,
  message?: string
}

export interface Pagination<T> {
  currentPage: number;
  foundPages: number;
  hasNextPage: boolean;
  media: T[];
}

export interface Anime {
  slug?: string | null;
  title: string;
  type: string;
  url: string;
  alternative_titles: string[];
  cover: string;
  genres: string[];
  rating: string;
  status?: string;
  synopsis: string;
  next_airing_episode?: string | null;
  related: AnimeRelation[];
  episodes: Episode[];
  last_episode_number_seen?: number | null;
}

export interface AnimeRelation {
  slug: string;
  title: string;
  url: string;
  relation: string;
}

export interface Server {
  name: string;
  download?: string;
  embed?: string;
}

export interface Episode {
  title?: string | null;
  slug?: string | null;
  number: number;
  servers: Server[];
  seen: boolean;
}

export const GENRES = [
  "accion",
  "artes-marciales",
  "aventura",
  "carreras",
  "ciencia-ficcion",
  "comedia",
  "demencia",
  "demonios",
  "deportes",
  "drama",
  "ecchi",
  "escolares",
  "espacial",
  "fantasia",
  "harem",
  "historico",
  "infantil",
  "josei",
  "juegos",
  "magia",
  "mecha",
  "militar",
  "misterio",
  "musica",
  "parodia",
  "policia",
  "psicologico",
  "recuentos-de-la-vida",
  "romance",
  "samurai",
  "seinen",
  "shoujo",
  "shounen",
  "sobrenatural",
  "superpoderes",
  "suspenso",
  "terror",
  "vampiros",
  "yaoi",
  "yuri",
] as const;

export const ANIME_TYPES = [
  "tv",
  "movie",
  "special",
  "ova"
] as const

export const ANIME_STATUSES = {
  1: "En emisión",
  2: "Finalizado",
  3: "Próximamente",
} as const;

export const ANIME_ORDERS = [
  "default",
  "updated",
  "added",
  "title",
  "rating",
] as const;

export type AnimeOrder = typeof ANIME_ORDERS[number];
export type AnimeStatusId = keyof typeof ANIME_STATUSES;
export type Genre = typeof GENRES[number];
export type AnimeType = typeof ANIME_TYPES[number];

export interface Filters {
  types?: AnimeType[],
  genres?: Genre[],
  statuses?: AnimeStatusId[]
}
