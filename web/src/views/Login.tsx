import { useState, useEffect } from "react";
import Button from "@mui/material/Button";
import CssBaseline from "@mui/material/CssBaseline";
import TextField from "@mui/material/TextField";
import FormControlLabel from "@mui/material/FormControlLabel";
import Checkbox from "@mui/material/Checkbox";
import Grid from "@mui/material/Grid";
import Box from "@mui/material/Box";
import { Link } from "react-router-dom";
import Typography from "@mui/material/Typography";
import Container from "@mui/material/Container";
import { createTheme, ThemeProvider } from "@mui/material/styles";

// RPC Connect
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { createPromiseClient } from "@connectrpc/connect";
import { Lucle } from "gen/lucle_connect";

// RPC
import { Connection } from "utils/rpc";

const theme = createTheme();

function Login({ setIsLogged }: { setIsLogged: any }) {
  const [login, setLogin] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [remember, setRemember] = useState<any>();
  const [error, setError] = useState<string>("");
  const [client, setClient] = useState<any>();

  useEffect(() => {
    const transport = createGrpcWebTransport({
      baseUrl: `http://127.0.0.1:50051`,
    });
    const newclient = createPromiseClient(Lucle, transport);
    setClient(newclient);
  }, []);

  const handleLogin = () => {
    if (remember) {
      localStorage.setItem("username", login);
      localStorage.setItem("password", password);
    }
    Connection(client, login, password)
      .then(() => {
        setIsLogged(true);
      })
      .catch((error) => setError(error));
  };

  return (
    <ThemeProvider theme={theme}>
      <Container component="main" maxWidth="xs">
        <CssBaseline />
        <Box
          sx={{
            marginTop: 8,
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
          }}
        >
          <Typography component="h1" variant="h5">
            Sign in
          </Typography>
          <Box sx={{ mt: 1 }}>
            <TextField
              margin="normal"
              required
              fullWidth
              id="email"
              label="Email Address"
              name="email"
              autoComplete="email"
              autoFocus
              value={login}
              onChange={(event) => setLogin(event.target.value)}
            />
            <TextField
              margin="normal"
              required
              fullWidth
              name="password"
              label="Password"
              type="password"
              id="password"
              autoComplete="current-password"
              value={password}
              onChange={(event) => setPassword(event.target.value)}
            />
            <FormControlLabel
              control={
                <Checkbox
                  value="remember"
                  color="primary"
                  onChange={() => setRemember(true)}
                />
              }
              label="Remember me"
            />
            <Button
              type="submit"
              fullWidth
              variant="contained"
              sx={{ mt: 3, mb: 2 }}
              onClick={handleLogin}
            >
              Sign In
            </Button>
            <Grid container>
              <Grid item xs>
                <Link to="/forgot">Forgot password?</Link>
              </Grid>
            </Grid>
            {error}
          </Box>
        </Box>
      </Container>
    </ThemeProvider>
  );
}

export default Login;
