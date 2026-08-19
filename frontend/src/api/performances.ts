import { api } from "./client";
import type { PaginationParams } from "./types";

export const performances = {
  list: (params?: PaginationParams) => api.GET("/api/performances", { params: { query: params } }),

  get: (id: string) => api.GET("/api/performances/{id}", { params: { path: { id } } }),

  getLyrics: (id: string) => api.GET("/api/performances/{id}/lyrics", { params: { path: { id } } }),
};
