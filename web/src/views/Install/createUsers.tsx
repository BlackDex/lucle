import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";

export default function CreateDefaultUser() {
  return (
    <Box
      component="form"
      sx={{
        "& > :not(style)": { m: 1, width: "25ch" },
      }}
      noValidate
      autoComplete="off"
    >
      <TextField id="login" label="Username" variant="standard" />
      <TextField
        id="password"
        label="password"
        variant="standard"
        type="password"
      />
    </Box>
  );
}
