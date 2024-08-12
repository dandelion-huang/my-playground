import { configureStore } from "@reduxjs/toolkit";
import { useDispatch } from "react-redux";
import { helloSlice } from "./helloSlice";

const store = configureStore({
  reducer: {
    hello: helloSlice.reducer,
    hell0: helloSlice.reducer, // it should be another reducer...
  },
});

export { store };
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
export const useAppDispatch = useDispatch.withTypes<AppDispatch>();
