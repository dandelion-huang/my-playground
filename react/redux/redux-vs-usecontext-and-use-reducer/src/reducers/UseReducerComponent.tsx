import { useReducer } from "react";

type GreetingStatus = "HELLO" | "HAUNTING";

type State = {
  status: GreetingStatus;
  greeting: string;
};

type Action = {
  type: GreetingStatus;
  payload: string;
};

const initialState: State = {
  greeting: "Hello, useReducer!",
  status: "HELLO",
};

function reducer(state: State, action: Action): State {
  switch (action.type) {
    case "HELLO":
      return {
        ...state,
        greeting: action.payload + " " + "Hello, useReducer!",
      };
    case "HAUNTING":
      return {
        ...state,
        greeting: action.payload + " " + "Halloween, tee-hee!",
      };
    default:
      throw new Error("Unknown action type");
  }
}

function Greeting() {
  const [state, dispatch] = useReducer(reducer, initialState);

  return (
    <div>
      <h1>useReducer</h1>
      <p>{state.greeting}</p>
      <button
        onClick={() => dispatch({ type: "HELLO", payload: "Hello~~~~?" })}
      >
        Hello
      </button>
      <button
        onClick={() => dispatch({ type: "HAUNTING", payload: "HAUNTING!" })}
      >
        Haunting
      </button>
    </div>
  );
}

export { Greeting as UseReducerComponent };
export type { Action, GreetingStatus, State };
