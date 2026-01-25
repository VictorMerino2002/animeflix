import Cookies from "js-cookie";

export class TokenStorageService {

  static save(token: string) {
    Cookies.set('auth-token', token);
  }

  static get() {
    return Cookies.get('auth-token');
  }

  static clear() {
    Cookies.remove('auth-token');
  }
}
