import { AuthProvider } from "../providers/auth-provider";
import { TokenStorageService } from "../services/token-storage-service";
import type { NavigateFunction } from "react-router-dom";
import type { NotificationInstance } from "antd/es/notification/interface";

export class LoginUseCase {
  navigator: NavigateFunction;
  notification: NotificationInstance;

  constructor(navigator: NavigateFunction, notification: NotificationInstance) {
    this.navigator = navigator;
    this.notification = notification;
  }

  async execute(username: string, password: string) {
    try {
      const apiResponse = await AuthProvider.login(username, password);

      if (!apiResponse.success) {
        this.notification.error({
          title: "Login error",
          description: apiResponse?.message,
        });
        return;
      }

      TokenStorageService.save(apiResponse.payload);
      this.navigator("/");
    } catch {
      this.notification.error({
        title: "Login error"
      });
    }
  }
}
