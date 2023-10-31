import Button from "@mui/material/Button";
import Box from "@mui/material/Box";
import Paper from "@mui/material/Paper";
import TableContainer from "@mui/material/TableContainer";
import Table from "@mui/material/Table";
import TableHead from "@mui/material/TableHead";
import TableCell from "@mui/material/TableCell";
import TableRow from "@mui/material/TableRow";
import TableBody from "@mui/material/TableBody";

function Index() {

  return (
    <div>
      <Button variant="contained">Create Table</Button>
      <Box sx={{ width: "100%" }}>
        <Paper sx={{ width: "100%", mb: 2 }}>
          <TableContainer>
            <Table sx={{ minWidth: 200 }}>
              <TableHead>
                <TableRow>
                  <TableCell key="table">Tables</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
              </TableBody>
            </Table>
          </TableContainer>
        </Paper>
      </Box>
      <Button variant="contained">Deploy</Button>
    </div>
  );
}

export default Index;
