// Shared request parameter types used across API modules.
// Response types will come from generated.ts once `pnpm api:gen` is run.

export type PaginationParams = Readonly<{
  page?: number;
  per_page?: number;
}>;

export type CredentialsBody = Readonly<{
  username: string;
  password: string;
}>;
