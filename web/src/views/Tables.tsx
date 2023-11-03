import { useState, useEffect } from "react";
import TextField from "@mui/material/TextField";
import TableContainer from "@mui/material/TableContainer";
import Table from "@mui/material/Table";
import TableHead from "@mui/material/TableHead";
import TableCell from "@mui/material/TableCell";
import TableRow from "@mui/material/TableRow";
import TableBody from "@mui/material/TableBody";
import Button from "@mui/material/Button";
import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";
import IconButton from "@mui/material/IconButton";

// icons
import ClearIcon from "@mui/icons-material/Clear";
import SaveIcon from "@mui/icons-material/Save";
import AddIcon from "@mui/icons-material/Add";
import DeleteIcon from "@mui/icons-material/DeleteOutline";
import CreateIcon from "@mui/icons-material/Create";

interface Data {
  id: number;
  username: string;
  email: string;
  role: string;
  createdat: string;
}

const CreateData = (
  id: number,
  username: string,
  email: string,
  role: string,
  createdat: string,
): Data => ({ id, username, email, role, createdat });

function Tables() {
  const [rows, setRows] = useState<Data[]>([]);
  const [editingIndex, setEditingIndex] = useState<any>(-1);

  const addRow = (index: any) => {
    setRows((state) => [...state, CreateData(index, " ", " ", " ", " ")]);
    setEditingIndex(index);
  };

  return (
    <div>
      <Button
        color="primary"
        startIcon={<AddIcon />}
        onClick={() => addRow(rows.length + 1)}
      >
        Add record
      </Button>
      <TableContainer>
        <Table sx={{ minWidth: 200 }}>
          <TableHead>
            <TableRow>
              <TableCell>id</TableCell>
              <TableCell>username</TableCell>
              <TableCell>email</TableCell>
              <TableCell>role</TableCell>
              <TableCell>createdAt</TableCell>
              <TableCell>Action</TableCell>
            </TableRow>
          </TableHead>
          <TableBody></TableBody>
        </Table>
      </TableContainer>
    </div>
  );
}

export default Tables;
