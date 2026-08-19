import { create } from "zustand";

export interface AuthUser {
  readonly id: string;
  readonly username: string;
  readonly capabilities: readonly string[];
}

interface AuthState {
  user: AuthUser | null;
  setUser: (user: AuthUser | null) => void;
  hasCapability: (capability: string) => boolean;
}

export const useAuthStore = create<AuthState>((set, get) => ({
  user: null,
  setUser: (user) => {
    set({ user });
  },
  hasCapability: (capability) => get().user?.capabilities.includes(capability) ?? false,
}));
