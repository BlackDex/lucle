import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import { get } from 'utils/Api';

const Install = () => {
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
	onClick={() => get("/api") }
      >Ok</Button>
    </Box>
  );
}

export default Install;
