/**
=========================================================
* Material Dashboard 2 React - v2.2.0
=========================================================

* Product Page: https://www.creative-tim.com/product/material-dashboard-react
* Copyright 2023 Creative Tim (https://www.creative-tim.com)

Coded by www.creative-tim.com

 =========================================================

* The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
*/

import { useState, useEffect } from "react";

// react-router components
import { useLocation, useRoutes } from "react-router-dom";

// RPC Components
import { checkIfInstalled } from "utils/rpc";

// Material Dashboard 2 React routes
import routes from "routes";

// @mui material components
import { ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";

// Material Dashboard 2 React themes
import theme from "assets/theme";

// Material Dashboard 2 React Dark Mode themes
import themeDark from "assets/theme-dark";

// Material Dashboard 2 React contexts
import { useMaterialUIController, CLientConnectBuf } from "context";

export default function App() {
  const [isInstalled, setIsInstalled] = useState<boolean>(false);
  const content = useRoutes(routes(isInstalled));
  const { pathname } = useLocation();

  const [controller, dispatch] = useMaterialUIController();
  const { darkMode } = controller;

/*  useEffect(() => {
    if (location.pathname === "/admin") {
      checkIfInstalled(client)
        .then(() => setIsInstalled(true))
        .catch(() => setIsInstalled(false));
    } 
  }, [pathname]); */

  return (
    <ThemeProvider theme={darkMode ? themeDark : theme}>
      <CssBaseline />
      {content}
    </ThemeProvider>
  );
}
