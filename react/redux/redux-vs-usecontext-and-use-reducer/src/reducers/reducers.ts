import { helloReducer } from "./helloReducer";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type ReducersMap<T = any> = {
  [key: string]: (state: T | undefined, action: T) => T;
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type State<T = any> = {
  [key: string]: T;
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Action<T = any> = {
  type: string;
  payload?: T;
};

// combineReducer is used to combine multiple reducers into one reducer
function combineReducer(reducers: ReducersMap) {
  const reducerKeys = Object.keys(reducers);

  const createInitialState = (): State => {
    return reducerKeys.reduce((acc: State, key: string) => {
      acc[key] = reducers[key](undefined, { type: "" });
      return acc;
    }, {});
  };

  return (state?: State, action?: Action): State => {
    const currentState = state || createInitialState();
    if (action) {
      const newState: State = { ...currentState };
      reducerKeys.forEach((key) => {
        const previousStateForKey = currentState[key];
        newState[key] = reducers[key](previousStateForKey, action);
      });
      return newState;
    }
    return { ...currentState }; // immutable
  };
}

const reducers = combineReducer({
  hello: helloReducer,
  hell0: helloReducer, // it should be another reducer...
});

export { reducers };
export type { State, Action };
