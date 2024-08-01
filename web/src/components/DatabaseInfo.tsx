import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";

let dbInfos = {
  dbName: "",
  hostname: "",
  port: 0,
  username: "",
  password: "",
};

function DatabaseInfo({
  dbInfos,
  setDBInfos,
}: {
  dbInfos: dbInfos;
  setDBInfos: any;
}) {
  const handleChange = (e) => {
    const { id, value } = e.target;
    setDBInfos({
      ...dbInfos,
      [id]: value,
    });
  };

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
          id="dbName"
          label="Database name"
          onChange={(e) => handleChange(e)}
        />
        <TextField
          required
          margin="normal"
          id="hostname"
          label="Database hostname"
          onChange={handleChange}
        />
        <TextField
          required
          label="Port"
          type="number"
          margin="normal"
          id="port"
          onChange={handleChange}
        />
        <TextField
          required
          margin="normal"
          label="Username"
          id="username"
          onChange={handleChange}
        />
        <TextField
          required
          label="Password"
          id="password"
          margin="normal"
          type="password"
          onChange={handleChange}
        />
      </Box>
    </Box>
  );
}

export default DatabaseInfo;
