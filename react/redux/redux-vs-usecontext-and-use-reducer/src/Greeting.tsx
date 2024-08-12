import { useContext } from "react";
import { ReducerContext } from "./ReducerContextProvider";

function Greeting() {
  const [state, dispatch] = useContext(ReducerContext)!;
  return (
    <>
      <h1>Greetings</h1>
      <div>
        <h2>{state.hello.name + ": " + state.hello.message}</h2>
        <button
          onClick={() =>
            dispatch({
              type: "SET_NAME",
              payload: { name: "William" },
            })
          }
        >
          William?
        </button>
        <button
          onClick={() =>
            dispatch({
              type: "SET_MESSAGE",
              payload: { message: "Good night~" },
            })
          }
        >
          Good night?
        </button>
      </div>
      <div>
        <h2>{state.hell0.name + ": " + state.hell0.message}</h2>
        <button
          onClick={() =>
            dispatch({
              type: "SET_NAME",
              payload: { name: "Karen" },
            })
          }
        >
          Karen?
        </button>
        <button
          onClick={() =>
            dispatch({
              type: "SET_MESSAGE",
              payload: { message: "Good afternoon." },
            })
          }
        >
          Good afternoon?
        </button>
      </div>
    </>
  );
}

export { Greeting };
