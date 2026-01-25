import { Route, Routes } from "react-router-dom";
import LoginPage from "./pages/login/page";
import Layout from "./components/layout/component";
import HomePage from "./pages/home/page";
import PlayerPage from "./pages/player/page";

export default function Router() {
  return (
    <Routes>
      <Route element={<Layout />}>
        <Route path="/login" element={<LoginPage />} />
        <Route path="/" element={<HomePage />} />
        <Route path="/anime/:animeSlug/play/:episodeSlug" element={<PlayerPage />} />
      </Route>
    </Routes>
  )
}
