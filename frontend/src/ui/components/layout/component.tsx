import { Outlet } from "react-router-dom";
import "./style.css";
import { Layout as AntLayout } from "antd";

export default function Layout() {
  return (
    <AntLayout>
      <Outlet />
    </AntLayout>
  )
}
