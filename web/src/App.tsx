import { useState, useEffect } from "react";
import { ThemeProvider, StyledEngineProvider } from "@mui/material/styles";
import { useRoutes, useLocation } from "react-router-dom";
import GlobalStyles from "components/GlobalStyles";
import theme from "theme/Index";
import routes from "./routes";

// Components
import { check_if_installed } from "utils/rpc";

// RPC Connect
import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { createPromiseClient } from "@bufbuild/connect";
import { Lucle } from "gen/lucle_connect";

function App() {
  const [isLogged, setIsLogged] = useState<boolean>(false);
  const [isInstalled, setIsInstalled] = useState<boolean>(false);
  const [client, setClient] = useState<any>();
  const content = useRoutes(routes(isInstalled, isLogged));
  const location = useLocation();

  useEffect(() => {
    // TODO: check if credentials are stored
    setIsLogged(false);
    // TODO: Check if cms is installed
    setIsInstalled(false);
    const transport = createGrpcWebTransport({
      baseUrl: `http://127.0.0.1:50051`,
    });
    const client = createPromiseClient(Lucle, transport);
    setClient(client);

    if (location.pathname == "/admin") {
      check_if_installed(client);
      // .then(() => setIsInstalled(true))
      // .catch(() => setIsInstalled(false));
    }
  }, []);

  return (
    <StyledEngineProvider injectFirst>
      <ThemeProvider theme={theme}>
        <GlobalStyles />
        {content}
      </ThemeProvider>
    </StyledEngineProvider>
  );
}

export default App;
