import { create } from "zustand";

export const useTokenStore = create((set) => ({
  token: "",
  tier: "Free",
  setTier: (new_tier) => set((state) => ({ tier: new_tier })),
  setToken: (jwt) => set((state) => ({ token: jwt })),
}));
