import { createContext, useReducer, Dispatch } from "react";
import { reducers, State, Action } from "./reducers";

const ReducerContext = createContext<[State, Dispatch<Action>] | null>(null);
export { ReducerContext };

const initialState = reducers();

function ReducerContextProvider({ children }: { children: React.ReactNode }) {
  const reducer = useReducer(reducers, initialState);

  return (
    <ReducerContext.Provider value={reducer}>
      {children}
    </ReducerContext.Provider>
  );
}

export { ReducerContextProvider };
