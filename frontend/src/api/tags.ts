import { api } from "./client";

export const tags = {
  list: () => api.GET("/api/tags", {}),
};
