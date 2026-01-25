import type { NotificationInstance } from "antd/es/notification/interface";
import { AnimeProvider } from "../providers/anime-provider";
import type { Anime } from "../../domain/interfaces";

export class GetOnAirAnimesUseCase {

  notification: NotificationInstance

  constructor(notification: NotificationInstance) {
    this.notification = notification;
  }

  async execute(): Promise<Anime[] | null> {
    const apiResponse = await AnimeProvider.getOnAirAnimes();

    if (!apiResponse.success) {
      this.notification.error({
        title: "Error",
        description: apiResponse.message,
      });
      return null;
    }

    return apiResponse.payload;
  }
}
