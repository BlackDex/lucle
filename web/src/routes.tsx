import { Navigate, Outlet } from "react-router-dom";

import Landing from "layouts/Landing";
import Install from "layouts/Install";
import ForgotPassword from "views/ForgotPassword";
import AdminIndex from "views/AdminIndex";
import Speedupdate from "views/Speedupdate";
import Login from "views/Login";
import Dashboard from "layouts/Dashboard";

function AnonymousRoutes({ isLogged }: { isLogged: boolean }) {
  return isLogged ? <Navigate to="/admin" replace /> : <Outlet />;
}

function PrivateRoutes({ isLogged }: { isLogged: boolean }) {
  return isLogged ? <Outlet /> : <Navigate to="/login" replace />;
}

function InstalledRoutes({ isInstalled = false }: { isInstalled: boolean }) {
  //console.log(isInstalled);
  let isInstall = false;
  return isInstall ? <Outlet /> : <Navigate to="/install" replace />;
}

function UninstalledRoutes({ isInstalled }: { isInstalled: boolean }) {
  let isInstall = false;
  return isInstall ? <Navigate to="/" replace /> : <Outlet />;
}

const routes = (isInstalled: boolean, isLogged = false) => [
  {
    element: <AnonymousRoutes isLogged={isLogged} />,
    children: [{ path: "/", element: <Landing /> }],
  },
  {
    element: <InstalledRoutes isInstalled />,
    children: [
      { path: "/login", element: <Login /> },
      { path: "/forgot", element: <ForgotPassword /> },
      {
        element: <PrivateRoutes isLogged />,
        children: [
          {
            path: "admin/*",
            element: <Dashboard />,
            children: [
              { index: true, element: <AdminIndex /> },
              { path: "speedupdate", element: <Speedupdate /> },
              //{ path: "tables", element: <Tables /> },
            ],
          },
        ],
      },
    ],
  },
  {
    element: <UninstalledRoutes isInstalled={isInstalled} />,
    children: [{ path: "/install", element: <Install /> }],
  },
];

export default routes;
