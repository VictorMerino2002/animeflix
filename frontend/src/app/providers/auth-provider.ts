import { API_URL } from "../../config/constants";
import type { ApiResponse } from "../../domain/interfaces";

export class AuthProvider {

  static async login(username: string, password: string): Promise<ApiResponse<string>> {
    const response = await fetch(`${API_URL}/auth/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ username, password })
    });
    return await response.json();
  }
}
