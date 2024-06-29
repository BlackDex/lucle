import { useState } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import TextField from "@mui/material/TextField";
import FormControlLabel from "@mui/material/FormControlLabel";
import Checkbox from "@mui/material/Checkbox";
import Button from "@mui/material/Button";
import Grid from "@mui/material/Grid";
import { Link } from "react-router-dom";

function Signin({ onSignin }: { onSignin: void }) {
  const [username, setUsername] = useState<string>(
    localStorage.getItem("username"),
  );
  const [password, setPassword] = useState<string>(
    localStorage.getItem("password"),
  );
  const [remember, setRemember] = useState<boolean>(false);

  return (
    <Box
      sx={{
        marginTop: 8,
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
      }}
    >
      <Box sx={{ mt: 1 }}>
        <TextField
          margin="normal"
          required
          fullWidth
          id="user"
          label="Username or Email"
          name="username"
          value={username}
          onChange={(event) => setUsername(event.target.value)}
        />
        <TextField
          margin="normal"
          required
          fullWidth
          name="password"
          label="Password"
          type="password"
          id="password"
          value={password}
          onChange={(event) => setPassword(event.target.value)}
        />
        <FormControlLabel
          control={
            <Checkbox
              checked={remember}
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
          onClick={() => onSignin(username, password, remember)}
        >
          Sign In
        </Button>
        <Grid container>
          <Grid item xs>
            <Link to="/forgot">Forgot password?</Link>
          </Grid>
        </Grid>
      </Box>
    </Box>
  );
}

export default Signin;
