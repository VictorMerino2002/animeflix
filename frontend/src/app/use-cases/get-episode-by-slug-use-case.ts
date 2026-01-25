import type { NotificationInstance } from "antd/es/notification/interface";
import type { Episode } from "../../domain/interfaces";
import { AnimeProvider } from "../providers/anime-provider";

export class GetEpisodeBySlugUseCase {

  notification: NotificationInstance;

  constructor(notification: NotificationInstance) {
    this.notification = notification;
  }

  async execute(animeSlug: string, episodeSlug: string): Promise<Episode | null> {
    try {
      const apiResponse = await AnimeProvider.getEpisodeBySlug(animeSlug, episodeSlug);

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
        description: "Errro trying to get episode"
      });
      return null;
    }
  }
}
