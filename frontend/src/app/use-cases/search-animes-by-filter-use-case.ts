import type { NotificationInstance } from "antd/es/notification/interface";
import type { Anime, AnimeOrder, Filters, Pagination } from "../../domain/interfaces";
import { AnimeProvider } from "../providers/anime-provider";

export class SearchAnimesByFilters {

  notification: NotificationInstance;

  constructor(notification: NotificationInstance) {
    this.notification = notification;
  }

  async execute(filters: Filters | undefined = undefined, order: AnimeOrder, page: number): Promise<Pagination<Anime> | null> {
    try {
      const apiResponse = await AnimeProvider.searchAnimeByFilters({
        filters,
        page,
        order
      });

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
        description: "Error loading animes"
      });
      return null;
    }
  }
}
