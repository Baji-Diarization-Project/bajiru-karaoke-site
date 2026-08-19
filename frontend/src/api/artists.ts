import { api } from "./client";
import type { PaginationParams } from "./types";

export const artists = {
  list: (params?: PaginationParams) => api.GET("/api/artists", { params: { query: params } }),

  get: (id: string) => api.GET("/api/artists/{id}", { params: { path: { id } } }),
};
