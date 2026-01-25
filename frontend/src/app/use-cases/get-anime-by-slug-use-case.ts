import type { NotificationInstance } from "antd/es/notification/interface";
import type { Anime } from "../../domain/interfaces";
import { AnimeProvider } from "../providers/anime-provider";

export class GetAnimeBySlugUseCase {

  notification: NotificationInstance;

  constructor(notification: NotificationInstance) {
    this.notification = notification;
  }

  async execute(slug: string): Promise<Anime | null> {
    try {
      const apiResponse = await AnimeProvider.getAnimeBySlug(slug);

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
        description: "Error loading anime"
      });
      return null;
    }
  }
}
