import { Routes, Route } from "react-router-dom";

const App = () => {
  return (
    <Routes>
      <Route index element={<h1>Home</h1>} />
    </Routes>
  );
};

export default App;
