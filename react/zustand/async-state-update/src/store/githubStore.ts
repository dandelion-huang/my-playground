// https://medium.com/@Hsu.Yang-Min/day25-%E6%88%91%E7%9A%84-react-%E5%AD%B8%E7%BF%92%E8%A8%98%E9%8C%84-zustand-81fd9abfb4e

import { create } from "zustand";

type FetchStatus = "IDLE" | "LOADING" | "SUCCESS";

type Data = {
  data: Data[];
  full_name: string;
  html_url: string;
};

interface DataState {
  data: Data[];
  status: FetchStatus;
  getData: (query: string) => void;
}

const useDataStore = create<DataState>((set) => ({
  data: [],
  status: "IDLE",
  getData: async (query: string) => {
    set({ status: "LOADING" });
    const response = await fetch(
      `https://api.github.com/search/repositories?q=${query}`
    );
    const data = await response.json();
    set({ data: data.items, status: "SUCCESS" });
  },
}));

export { useDataStore };
export type { DataState, FetchStatus };
