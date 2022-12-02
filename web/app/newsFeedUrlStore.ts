import create from "zustand";
import { NewsFeedUrl, getSdk, NewsFeedStatus } from "graphql/generated/graphql";
import { graphQLClient } from "graphql/client";
import { log } from "./log";

interface NewsFeedUrlState {
  newsFeedUrls: NewsFeedUrl[];
  newsFeedStatus: NewsFeedStatus;
  isLoading: boolean;
  getNewsFeedUrlsAndNewsFeedStatus: () => void;
}

export const useNewsFeedUrlStore = create<NewsFeedUrlState>(
  log((set: any, get: any) => ({
    newsFeedUrls: [],
    newsFeedStatus: null,
    isLoading: false,
    getNewsFeedUrlsAndNewsFeedStatus: async () => {
      if (get().newsFeedUrls.length == 0) {
        set({ isLoading: true });
        const sdk = getSdk(graphQLClient);
        const response = await sdk.GetNewsFeedUrlsAndNewsFeedStatus();
        set({
          newsFeedUrls: response.newsFeedUrls,
          newsFeedStatus: response.newsFeedStatus,
          isLoading: false
        });
      }
    }
  }))
);
