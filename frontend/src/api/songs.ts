import { api } from "./client";
import type { PaginationParams } from "./types";

export const songs = {
  list: (params?: PaginationParams) => api.GET("/api/songs", { params: { query: params } }),

  get: (id: string) => api.GET("/api/songs/{id}", { params: { path: { id } } }),

  getLyrics: (id: string) => api.GET("/api/songs/{id}/lyrics", { params: { path: { id } } }),
};
