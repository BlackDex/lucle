import { useState, useEffect } from "react";

import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";

// RPC Connect
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { createPromiseClient } from "@connectrpc/connect";
import { Lucle } from "gen/lucle_connect";

// RPC
import { forgotPassword } from "utils/rpc";

function ForgotPassword() {
  const [email, setEmail] = useState<string>("");
  const [client, setClient] = useState<any>();

  useEffect(() => {
    const transport = createGrpcWebTransport({
      baseUrl: `http://127.0.0.1:50051`,
    });
    const newclient = createPromiseClient(Lucle, transport);
    setClient(newclient);
  }, []);

  return (
    <div>
      <TextField
        margin="normal"
        required
        fullWidth
        id="email"
        label="Email Address"
        name="email"
        autoComplete="email"
        autoFocus
        value={email}
        onChange={(event) => setEmail(event.target.value)}
      />
      <Button
        type="submit"
        fullWidth
        variant="contained"
        sx={{ mt: 3, mb: 2 }}
        onClick={() => forgotPassword(client, email)}
      >
        Send
      </Button>
    </div>
  );
}

export default ForgotPassword;
