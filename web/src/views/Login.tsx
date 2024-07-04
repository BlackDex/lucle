import { useState, useContext } from "react";
import Button from "@mui/material/Button";
import CssBaseline from "@mui/material/CssBaseline";
import TextField from "@mui/material/TextField";
import FormControlLabel from "@mui/material/FormControlLabel";
import Checkbox from "@mui/material/Checkbox";
import Grid from "@mui/material/Grid";
import Tabs from "@mui/material/Tabs";
import Tab from "@mui/material/Tab";
import TabPanel from "@mui/lab/TabPanel";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import Box from "@mui/material/Box";
import { Link } from "react-router-dom";
import Typography from "@mui/material/Typography";
import Container from "@mui/material/Container";
import { createTheme, ThemeProvider } from "@mui/material/styles";

// Components
import Signin from "components/Signin";
import Signup from "components/Signup";
import { createUser } from "utils/rpc";

// Context
import { LucleRPC } from "context";
import { useAuth } from "context/Auth";

const theme = createTheme();

function Login() {
  const [tab, setTab] = useState("1");
  const [login, setLogin] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [remember, setRemember] = useState<boolean>(false);
  const [error, setError] = useState<string>("");
  const [successfullSignup, setSuccessfullSignup] = useState<boolean>(false);
  const auth = useAuth();
  const client = useContext(LucleRPC);

  const handleSignup = (username, password, email) => {
    setError("");
    createUser(client, username, password, email)
      .then(() => setSuccessfullSignup(true))
      .catch((err) => setError(err.rawMessage));
  };

  const handleSignin = (username, password, remenber) => {
    setError("");
    if (remember) {
      localStorage.setItem("username", username);
      localStorage.setItem("password", password);
    }
    auth.Login({ username, password }).catch((err) => setError(err.rawMessage));
  };

  return (
    <ThemeProvider theme={theme}>
      <Container component="main" maxWidth="xs">
        <CssBaseline />
        <Box sx={{ borderBottom: 1, borderColor: "divider" }}>
          <TabContext value={tab}>
            <TabList
              onChange={(_, activeTab) => setTab(activeTab)}
              aria-label="signin-signup"
            >
              <Tab label="Sign In" value="1" />
              <Tab label="Sign Up" value="2" />
            </TabList>
            <TabPanel value="1">
              <Signin onSignin={handleSignin} error={setError} />
            </TabPanel>
            <TabPanel value="2">
              <Signup
                successfullSignup={successfullSignup}
                onSignup={handleSignup}
                error={setError}
              />
            </TabPanel>
          </TabContext>
          {error}
        </Box>
      </Container>
    </ThemeProvider>
  );
}

export default Login;
