import { defineStore } from 'pinia';

export const useAccountStore = defineStore('account', {
  state: () => ({
    name: "",
    email: "",
  }),
  actions: {
    changeName(to: string) {
      this.name = to;
    },
    changeEmail(to: string) {
      this.email = to;
    },
  },
  persist: {
    storage: persistedState.sessionStorage,
  },
});
