import { createSlice } from "@reduxjs/toolkit";

const helloSlice = createSlice({
  name: "hello",
  initialState: {
    name: "Rose",
    message: "...?",
  },
  reducers: {
    setName: (state, action) => {
      state.name = action.payload.name;
    },
    setMessage: (state, action) => {
      state.message = action.payload.message;
    },
  },
});

export { helloSlice };
export const { setName, setMessage } = helloSlice.actions;
