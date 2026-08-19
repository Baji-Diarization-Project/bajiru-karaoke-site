import { api } from "./client";
import type { CredentialsBody } from "./types";

export const auth = {
  me: () => api.GET("/auth/me", {}),

  register: (body: CredentialsBody) => api.POST("/auth/register", { body }),

  login: (body: CredentialsBody) => api.POST("/auth/login", { body }),

  logout: () => api.POST("/auth/logout", {}),
};
