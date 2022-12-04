import { useState, useEffect } from 'react';
import {
    createConnectTransport,
    createPromiseClient,
} from '@bufbuild/connect-web';
import { Lucle } from 'gen/lucle_connectweb';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import { install, connect  } from 'utils/rpc';

const Setup = () => {
  const [client, setClient] = useState<any>();

  useEffect(() => {
    const client = createPromiseClient(
        Lucle,
        createConnectTransport({
            baseUrl: 'http://127.0.0.1:3000',
        })
    )
    setClient(client);
  }, []);

  return(
    <Box sx={{ minWidth: 120 }}>
      <FormControl>
        <Select
          labelId="select-database"
          id="select-database"
          value={0}
          label="Database"
          //onChange={handleChange}
        >
          <MenuItem value={0}>SQLite</MenuItem>
	  <MenuItem value={1}>MySQL</MenuItem>
	  <MenuItem value={2}>PostgreSQL</MenuItem>
        </Select>
      </FormControl>
      <Button
        variant="contained"
	onClick={() => install(client, "sqlite") }
      >Ok</Button>
    </Box>
  );
}

export default Setup;
