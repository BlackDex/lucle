import { useState, useEffect } from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import { install, connect  } from 'utils/rpc';

function Setup() {
  const [client, setClient] = useState<any>();
  const [selectedDB, setSelectedDB] = useState(2);

  useEffect(() => {
    const newclient = connect("127.0.0.1", "3000");
    setClient(newclient);
  }, []);

  return(
    <Box sx={{ minWidth: 120 }}>
      <FormControl>
        <Select
          labelId="select-database"
          id="select-database"
          value={selectedDB}
          label="Database"
          onChange={event => setSelectedDB(event.currentTarget.value) }
        >
          <MenuItem value={0}>Mysql</MenuItem>
	  <MenuItem value={1}>PostgreSQL</MenuItem>
	  <MenuItem value={2}>Sqlite</MenuItem>
        </Select>
      </FormControl>
      <Button
        variant="contained"
	onClick={() => install(client, selectedDB) }
      >Ok</Button>
    </Box>
  );
}

export default Setup;
