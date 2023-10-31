import { useState } from "react";
import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";

import PasswordStrengthBar from "react-password-strength-bar";

export default function CreateDefaultUser({ user, passwd, email }) {
  const [password, setPassword] = useState<string>();
  const [confirmpassword, setConfirmPassword] = useState<string>();
  const [matchpassword, setMatchPassword] = useState<boolean>(false);

  return (
    <Box
      component="form"
      sx={{
        "& > :not(style)": { m: 1, width: "25ch" },
      }}
      noValidate
      autoComplete="off"
    >
      <TextField
        id="login"
        label="Username"
        variant="standard"
        onChange={(event) => {
          user(event.target.value);
        }}
      />
      <TextField
        id="password"
        label="Password"
        variant="standard"
        type="password"
        value={password}
        onChange={(event) => {
          passwd(event.target.value);
          setPassword(event.target.value);
        }}
      />
      <TextField
        id="password-confirm"
        label="Confirm password"
        variant="standard"
        type="password"
        value={confirmpassword}
        onChange={(event) => {
          setConfirmPassword(event.target.value);
          console.log(password);
          console.log(confirmpassword);
          {
            password !== confirmpassword
              ? setMatchPassword(false)
              : setMatchPassword(true);
          }
        }}
      />
      {matchpassword ? "Password doesn't match !" : null}
      <PasswordStrengthBar password={password} />
      <TextField
        id="email"
        label="email"
        variant="standard"
        onChange={(event) => {
          email(event.target.value);
        }}
      />
    </Box>
  );
}
