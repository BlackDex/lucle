import { useState, useEffect } from "react";
import { Navigate, Outlet } from "react-router-dom";
import Icon from "@mui/material/Icon";

import Dashboard from "layouts/Dashboard";
import Landing from "layouts/Landing";
import Install from "layouts/Install";
import ForgotPassword from "views/ForgotPassword";
import AdminIndex from "views/AdminIndex";
import OnlineEditor from "views/Editor";
import Tables from "views/Tables";
import Login from "views/Login";

function AnonymousRoutes({ isLogged }: { isLogged: boolean }) {
  return isLogged ? <Navigate to="/admin" replace /> : <Outlet />;
}

function PrivateRoutes({ isLogged }: { isLogged: boolean }) {
  return isLogged ? <Outlet /> : <Navigate to="/login" replace />;
}

function InstalledRoutes({ isInstalled }: { isInstalled: boolean }) {
  return isInstalled ? <Outlet /> : null;
}

function UninstalledRoutes({ isInstalled }: { isInstalled: boolean }) {
  return isInstalled ? <Navigate to="/" replace /> : <Outlet />;
}

const SiteRoutes = (isInstalled: boolean) => {
  const [isLogged, setIsLogged] = useState<boolean>(false);

  useEffect(() => {
    const result = localStorage.getItem("token");
    if (result) {
      setIsLogged(true);
    }
  }, [setIsLogged]);

  const handleConnection = () => {
    setIsLogged(true);
  };

  return [
    {
      element: <AnonymousRoutes isLogged={isLogged} />,
      children: [
        { path: "/login", element: <Login setIsLogged={handleConnection} /> },
        { path: "/", element: <Landing /> },
        { path: "/install", element: <Install /> },
        { path: "/admin", element: <Dashboard /> },
        { path: "/forgot", element: <ForgotPassword /> },
      ],
    },
/*     {
      element: <InstalledRoutes isInstalled={isInstalled} />,
      children: [
        {
          element: <PrivateRoutes isLogged={isLogged} />,
          children: [
            {
              path: "admin",
              element: <Dashboard />,
              children: [
                { index: true, element: <AdminIndex /> },
                { path: "editor", element: <OnlineEditor /> },
                { path: "tables", element: <Tables /> },
              ],
            },
          ],
        },
        {
          element: <UninstalledRoutes isInstalled={isInstalled} />,
          children: [{ path: "/install", element: <Install /> }],
        },
      ],
    }, */
  ];
};

const adminroutes = [
  {
    type: "collapse",
    name: "Home",
    key: "admin",
    icon: <Icon fontSize="small">dashboard</Icon>,
    route: "/admin",
    component: <Dashboard />,
  },
];

export { SiteRoutes, adminroutes };
