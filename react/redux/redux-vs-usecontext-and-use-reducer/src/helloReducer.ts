// ./UseReducerComponent.tsx

const ACTION_TYPE = {
  SET_NAME: "SET_NAME",
  SET_MESSAGE: "SET_MESSAGE",
} as const;

type ActionType = (typeof ACTION_TYPE)[keyof typeof ACTION_TYPE];

type Action = {
  type: ActionType;
  payload: {
    name: string;
    message: string;
  };
};

type State = {
  name: string;
  message: string;
};

const initialState: State = {
  name: "Rose",
  message: "...?",
};

function helloReducer(state = initialState, action: Action) {
  switch (action.type) {
    case ACTION_TYPE.SET_NAME:
      return {
        ...state,
        name: action.payload.name,
      };
    case ACTION_TYPE.SET_MESSAGE:
      return {
        ...state,
        message: action.payload.message,
      };
    default:
      return state;
  }
}

export { helloReducer, ACTION_TYPE };
export type { Action, ActionType, State };
