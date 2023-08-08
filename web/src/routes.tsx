import { Navigate } from "react-router-dom";
import Dashboard from "layouts/Dashboard";
import AdminIndex from "views/admin/Index";
import Index from "views/Index";
import OnlineEditor from "views/Editor";
import Tables from "views/Tables";
import Login from "views/Login";
import Setup from "views/Install";
import Speedupdate from "views/Speedupdate";

const routes = (isInstalled: boolean, isLogged: boolean) => [
  { path: "/login", element: <Login /> },
  {
    path: "admin",
    element: !isInstalled ? (
      <Navigate to="/install" />
    ) : !isLogged ? (
      <Navigate to="/login" />
    ) : (
      <Dashboard />
    ),
    children: [
      { path: "", element: <AdminIndex /> },
      { path: "editor", element: <OnlineEditor /> },
      { path: "tables", element: <Tables /> },
    ],
  },
  {
    path: "/install",
    element: <Setup />,
  },
  {
    path: "/",
    element: !isInstalled ? (
      <Navigate to="/install" />
    ) : <Index />,
  },
  {
    path: "/update",
    element: <Dashboard />,
    children: [{ path: "", element: <Speedupdate /> }],
  },
];

export default routes;
