import type { NotificationInstance } from "antd/es/notification/interface";
import type { Anime, Pagination } from "../../domain/interfaces";
import { AnimeProvider } from "../providers/anime-provider";

export class SearchAnimeUseCase {
  notification: NotificationInstance;

  constructor(notification: NotificationInstance) {
    this.notification = notification;
  }

  async execute(query: string, page: number): Promise<Pagination<Anime> | null> {
    try {
      const apiResponse = await AnimeProvider.searchAnime(query, page);

      if (!apiResponse.success) {
        this.notification.error({
          title: "Error",
          description: apiResponse.message
        });
        return null;
      }

      return apiResponse.payload;
    } catch {
      this.notification.error({
        title: "Error",
        description: "An error has ocurred while searching animes"
      })
      return null;
    }
  }
}
