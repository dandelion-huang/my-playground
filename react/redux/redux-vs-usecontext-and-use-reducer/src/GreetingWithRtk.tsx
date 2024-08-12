import { setMessage, setName } from "./store/helloSlice";
import { useSelector, useDispatch } from "react-redux";
import { RootState } from "./store/store";

function GreetingWithRtk() {
  const dispatch = useDispatch();
  const hello = useSelector((state: RootState) => state.hello);
  const hell0 = useSelector((state: RootState) => state.hell0);

  return (
    <>
      <h1>Greetings</h1>
      <div>
        <h2>{hello.name + ": " + hello.message}</h2>
        <button onClick={() => dispatch(setName({ name: "Solong" }))}>
          Solong?
        </button>
        <button onClick={() => dispatch(setMessage({ message: "Bang!" }))}>
          Bang?
        </button>
      </div>
      <div>
        <h2>{hell0.name + ": " + hell0.message}</h2>
        <button onClick={() => dispatch(setName({ name: "Cara" }))}>
          Cara?
        </button>
        <button onClick={() => dispatch(setMessage({ message: "Cute!" }))}>
          Cute?
        </button>
      </div>
    </>
  );
}

export { GreetingWithRtk };
