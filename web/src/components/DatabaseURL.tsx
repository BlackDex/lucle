import { useState } from "react";
import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";

function DatabaseURL() {
  const [url, setURL] = useState<string>("");
  const [port, setPort] = useState<string>("");
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");

  return (
    <Box
      sx={{
        marginTop: 8,
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        border: "3px solid",
      }}
    >
      <Box sx={{ mt: 1 }}>
        <TextField
          margin="normal"
          required
          id="url"
          label="URL"
          value={url}
          onChange={(event) => setURL(event.target.value)}
        />
        <TextField
          margin="normal"
          required
          label="port"
          id="port"
          value={port}
          onChange={(event) => setPort(event.target.value)}
        />
        <TextField
          margin="normal"
          required
          label="username"
          id="username"
          value={username}
          onChange={(event) => setUsername(event.target.value)}
        />
        <TextField
          margin="normal"
          required
          label="password"
          id="password"
          type="password"
          value={password}
          onChange={(event) => setPassword(event.target.value)}
        />
      </Box>
    </Box>
  );
}

export default DatabaseURL;
