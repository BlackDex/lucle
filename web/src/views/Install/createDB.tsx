import { useState, useEffect } from "react";
import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";
import { install, connect } from "utils/rpc";

import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { createPromiseClient } from "@bufbuild/connect";
import { Lucle } from "gen/lucle_connect";

function CreateDB() {
  const [client, setClient] = useState<any>();
  const [selectedDB, setSelectedDB] = useState<any>(2);

  useEffect(() => {
    //const newclient = connect("127.0.0.1", "3000");
    const transport = createGrpcWebTransport({
      baseUrl: `http://127.0.0.1:50051`,
    });
    const client = createPromiseClient(Lucle, transport);
    setClient(client);
  }, []);

  return (
    <Box sx={{ minWidth: 120 }}>
      <FormControl>
        <Select
          labelId="select-database"
          id="select-database"
          value={selectedDB}
          label="Database"
          onChange={(event) => setSelectedDB(event.target.value)}
        >
          <MenuItem value={0}>Mysql</MenuItem>
          <MenuItem value={1}>PostgreSQL</MenuItem>
          <MenuItem value={2}>Sqlite</MenuItem>
        </Select>
      </FormControl>
      <Button variant="contained" onClick={() => install(client, selectedDB)}>
        Ok
      </Button>
    </Box>
  );
}

export default CreateDB;
