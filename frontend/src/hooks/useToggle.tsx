import {
  useState,
  useCallback,
  type Dispatch,
  type SetStateAction,
} from "react";

const useToggle = (
  initialState = false
): readonly [boolean, () => void, Dispatch<SetStateAction<boolean>>] => {
  const [state, setState] = useState<boolean>(initialState);

  const Toggle = useCallback(() => setState(!state), [state]);

  return [state, Toggle, setState] as const;
};

export default useToggle;
