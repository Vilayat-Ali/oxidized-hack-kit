import { type ReactNode } from "react";

type Props = {
  children: ReactNode;
};

const layout = ({ children }: Props) => {
  return <>{children}</>;
};

export default layout;
