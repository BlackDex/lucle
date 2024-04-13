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

function InstalledRoutes({ isInstalled }: { isInstalled: boolean }) {
  return isInstalled ? <Outlet /> : <Navigate to="/install" replace />;
}

function UninstalledRoutes({ isInstalled }: { isInstalled: boolean }) {
  return isInstalled ? <Navigate to="/" replace /> : <Outlet />;
}

const routes = (isInstalled: boolean, isLogged: boolean) => [
  {
    element: <AnonymousRoutes isLogged={isLogged} />,
    children: [
      { path: "/login", element: <Login /> },
      { path: "/", element: <Landing /> },
      { path: "/install", element: <Install /> },
      { path: "/forgot", element: <ForgotPassword /> },
    ],
  },
  {
    element: <InstalledRoutes isInstalled />,
    children: [
      {
        element: <PrivateRoutes isLogged />,
        children: [
          {
            path: "admin",
            element: <Dashboard />,
            children: [
              { index: true, element: <AdminIndex /> },
              { path: "speedupdate", element: <Speedupdate /> },
              //              { path: "tables", element: <Tables /> },
            ],
          },
        ],
      },
      /* {
        element: <UninstalledRoutes isInstalled={isInstalled} />,
        children: [{ path: "/install", element: <Install /> }],
      }, */
    ],
  },
];

export default routes;
