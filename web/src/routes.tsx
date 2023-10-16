import { Navigate } from "react-router-dom";
import Dashboard from "layouts/Dashboard";
import Install from "layouts/Install";
import AdminIndex from "views/admin/Index";
import Index from "views/Index";
import OnlineEditor from "views/Editor";
import Tables from "views/Tables";
import Login from "views/Login";
import Speedupdate from "views/Speedupdate";

const adminRoutes = (isInstalled: boolean, isLogged: boolean) => {
  console.log("isInstalled : " + isInstalled);
  if (!isInstalled) {
    return <Navigate to="/install" replace />;
  } else {
    return <Dashboard />;
  }
};

const routes = (isInstalled: boolean, isLogged: boolean) => [
  { path: "/login", element: <Login /> },
  {
    path: "admin",
    element: adminRoutes(isInstalled, isLogged),
    children: [
      { path: "", element: <AdminIndex /> },
      { path: "editor", element: <OnlineEditor /> },
      { path: "tables", element: <Tables /> },
    ],
  },
  {
    path: "/install",
    element: <Install />, //isInstalled ? <Navigate to="/" replace /> : <Install />,
  },
  {
    path: "/",
    element: <Index />,
  },
  {
    path: "/update",
    element: <Dashboard />,
    children: [{ path: "", element: <Speedupdate /> }],
  },
];

export default routes;
