import { api } from "./client";

export const playlists = {
  list: () => api.GET("/api/playlists", {}),

  get: (id: string) => api.GET("/api/playlists/{id}", { params: { path: { id } } }),

  getPerformances: (id: string) =>
    api.GET("/api/playlists/{id}/performances", { params: { path: { id } } }),
};
