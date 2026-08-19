import createClient from "openapi-fetch";

import type { paths } from "./generated";

// OpenAPI spec from generated schema
export const api = createClient<paths>({ baseUrl: "" });

export class ApiError extends Error {
  readonly status: number;
  readonly detail: unknown;

  constructor(status: number, detail: unknown) {
    super(`API error ${String(status)}`);
    this.name = "ApiError";
    this.status = status;
    this.detail = detail;
  }
}
