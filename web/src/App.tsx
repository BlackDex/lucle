import { useState, useEffect } from "react";
import { ThemeProvider, StyledEngineProvider } from "@mui/material/styles";
import { useRoutes, useLocation } from "react-router-dom";
import GlobalStyles from "components/GlobalStyles";
import theme from "theme/Index";

// Components
import { checkIfInstalled } from "utils/rpc";

import { MaterialUIControllerProvider } from "context";

// RPC Connect
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { createPromiseClient } from "@connectrpc/connect";
import { Lucle } from "gen/lucle_connect";

import Routes from "./routes";

function App() {
  const [isInstalled, setIsInstalled] = useState<boolean>(false);

  const location = useLocation();
  const content = useRoutes(Routes(isInstalled));

  useEffect(() => {
    const transport = createGrpcWebTransport({
      baseUrl: `http://127.0.0.1:50051`,
    });
    const client = createPromiseClient(Lucle, transport);

    if (location.pathname === "/admin") {
      checkIfInstalled(client)
        .then(() => setIsInstalled(true))
        .catch(() => setIsInstalled(false));
    }
  }, [location.pathname]);

  return (
    <StyledEngineProvider injectFirst>
      <ThemeProvider theme={theme}>
        <GlobalStyles />
        <MaterialUIControllerProvider>{content}</MaterialUIControllerProvider>
      </ThemeProvider>
    </StyledEngineProvider>
  );
}

export default App;
