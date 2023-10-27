import Box from "@mui/material/Box";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import TextField from "@mui/material/TextField";
import Select from "@mui/material/Select";

//Components
import DatabaseURL from "components/DatabaseURL";

function CreateDB({
  selectedDB,
  setSelectedDB,
}: {
  selectedDB: any;
  setSelectedDB: any;
}) {
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
      {selectedDB == 2 ? (
        <TextField id="filled-basic" label="Filled" variant="filled" />
      ) : null}

      {selectedDB != 2 ? <DatabaseURL /> : null}
    </Box>
  );
}

export default CreateDB;
