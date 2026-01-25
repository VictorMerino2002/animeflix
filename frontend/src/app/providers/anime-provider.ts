import { API_URL } from "../../config/constants";
import type { Anime, AnimeOrder, ApiResponse, Episode, Filters, Pagination } from "../../domain/interfaces";
import { TokenStorageService } from "../services/token-storage-service";

export class AnimeProvider {

  static getHeaders() {
    const token = TokenStorageService.get();
    if (!token) {
      window.location.pathname = "/login";
    }

    return {
      "Authorization": `Bearer ${token}`,
      "Content-Type": "application/json"
    }
  }

  static async getOnAirAnimes(): Promise<ApiResponse<Anime[]>> {
    const response = await fetch(`${API_URL}/anime/list/on-air`, {
      headers: this.getHeaders()
    });

    return await response.json();
  }

  static async getAnimeBySlug(slug: string): Promise<ApiResponse<Anime>> {
    const response = await fetch(`${API_URL}/anime/${slug}`, {
      headers: this.getHeaders()
    });

    return await response.json();
  }

  static async getEpisodeBySlug(animeSlug: string, episodeSlug: string): Promise<ApiResponse<Episode>> {
    const response = await fetch(`${API_URL}/anime/${animeSlug}/episode/${episodeSlug}`, {
      headers: this.getHeaders()
    });

    return await response.json();
  }

  static async getAnimeHistory(): Promise<ApiResponse<Anime[]>> {
    const response = await fetch(`${API_URL}/anime/history`, {
      headers: this.getHeaders()
    });
    return await response.json();
  }

  static async searchAnime(query: string, page: number): Promise<ApiResponse<Pagination<Anime>>> {
    const response = await fetch(`${API_URL}/anime/search?query=${encodeURIComponent(query)}&page=${page}`, {
      headers: this.getHeaders()
    });
    return await response.json();
  }

  static async searchAnimeByFilters({ filters, page, order }: { filters?: Filters, page: number, order: AnimeOrder }): Promise<ApiResponse<Pagination<Anime>>> {
    const response = await fetch(`${API_URL}/anime/search/by-filters?order=${order}&page=${page}`, {
      method: "POST",
      headers: this.getHeaders(),
      body: filters ? JSON.stringify(filters) : undefined,
    });

    return await response.json();
  }
}
