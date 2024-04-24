import { useState, useEffect, useContext } from "react";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import TableRow from "@mui/material/TableRow";
import TableHead from "@mui/material/TableHead";
import TableCell from "@mui/material/TableCell";
import Table from "@mui/material/Table";
import TableContainer from "@mui/material/TableContainer";
import InputAdornment from "@mui/material/InputAdornment";
import Paper from "@mui/material/Paper";
import Typography from "@mui/material/Typography";
import Checkbox from "@mui/material/Checkbox";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import Tooltip from "@mui/material/Tooltip";
import IconButton from "@mui/material/IconButton";
import { alpha } from "@mui/material/styles";
import { DropzoneArea } from "mui2-file-dropzone";

// Icons
import AddCircleIcon from "@mui/icons-material/AddCircle";
import DeleteIcon from "@mui/icons-material/Delete";

// RPC Connect
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { createPromiseClient } from "@connectrpc/connect";
import { Repo } from "gen/speedupdate_connect";

// api
import {
  init,
  status,
  registerVersion,
  unregisterVersion,
  registerPackage,
} from "utils/speedupdaterpc";

//import { uploadFile } from "utils/minio";

function Speedupdate() {
  const [client, setClient] = useState<any>();
  const [repoInit, setRepoInit] = useState<boolean>(false);
  const [url, setUrl] = useState<string>(localStorage.getItem("url") || "");
  const [currentVersion, setCurrentVersion] = useState<string>("");
  const [pack, setPack] = useState<any>();
  const [version, setVersion] = useState<any>();
  const [listPackages, setListPackages] = useState<string[]>([]);
  const [listVersions, setListVersions] = useState<any>();
  const [selectedVersions, setSelectedVersions] = useState<string[]>([]);
  const [path, setPath] = useState<string>(localStorage.getItem("path") || "");
  const [fileObjects, setFileObjects] = useState();
  const [error, setError] = useState<String>("");
  const [selected, setSelected] = useState<readonly number[]>([]);

  const isSelected = (id: number) => selected.indexOf(id) !== -1;
  const numSelected = selected.length;

  useEffect(() => {
    if (client) {
      status(client, path).then((repo: any) => {
        if (repo.repoinit) {
          setRepoInit(true);
          setCurrentVersion(repo.currentVersion);
          setListVersions(repo.listVersion);
          setListPackages(repo.packages);
        }
      });
    }
  }, [client, listVersions]);

  const DeleteVersion = () => {
    selectedVersions.forEach((version) => {
      unregisterVersion(client, path, version);
    });
  };

  const Connection = () => {
    const transport = createGrpcWebTransport({
      baseUrl: "http://0.0.0.0:50051",
    });
    const newclient = createPromiseClient(Repo, transport);
    setClient(newclient);
  };

  const handleClick = (id: number, version: string) => {
    const selectedIndex = selected.indexOf(id);
    let newSelected: readonly number[] = [];

    if (selectedIndex === -1) {
      newSelected = newSelected.concat(selected, id);
    } else if (selectedIndex === 0) {
      newSelected = newSelected.concat(selected.slice(1));
    } else if (selectedIndex === selected.length - 1) {
      newSelected = newSelected.concat(selected.slice(0, -1));
    } else if (selectedIndex > 0) {
      newSelected = newSelected.concat(
        selected.slice(0, selectedIndex),
        selected.slice(selectedIndex + 1),
      );
    }

    setSelected(newSelected);

    if (newSelected.includes(id)) {
      setSelectedVersions((previous_version) => [...previous_version, version]);
    } else {
      const updatedVersions = selectedVersions.filter((ver) => ver !== version);
      setSelectedVersions(updatedVersions);
    }
  };

  let speedupdatecomponent;

  if (!client) {
    speedupdatecomponent = (
      <div>
        <TextField
          id="outlined-required"
          label="url"
          value={url}
          onChange={(e: any) => {
            setUrl(e.currentTarget.value);
            localStorage.setItem("url", e.currentTarget.value);
          }}
        />
        <Button variant="contained" onClick={Connection}>
          Connection
        </Button>
      </div>
    );
  } else if (!repoInit) {
    speedupdatecomponent = (
      <div>
        <TextField
          id="outlined-required"
          label="path"
          value={path}
          onChange={(e: any) => {
            setPath(e.currentTarget.value);
            localStorage.setItem("path", e.currentTarget.value);
          }}
        />
        <Button
          variant="contained"
          onClick={() => {
            init(client, path).catch((error: any) => {
              setRepoInit(false);
              setError(error);
            });
          }}
        >
          Initialize repository
        </Button>
        {error}
      </div>
    );
  } else {
    speedupdatecomponent = (
      <Box sx={{ width: "100%" }}>
        <Paper sx={{ width: "100%", mb: 2 }}>
          <Toolbar
            sx={{
              pl: { sm: 2 },
              pr: { xs: 1, sm: 1 },
              ...(numSelected > 0 && {
                bgcolor: (theme) =>
                  alpha(
                    theme.palette.primary.main,
                    theme.palette.action.activatedOpacity,
                  ),
              }),
            }}
          >
            {numSelected > 0 ? (
              <Typography
                sx={{ flex: "1 1 100%" }}
                color="inherit"
                variant="subtitle1"
                component="div"
              >
                {numSelected} selected
              </Typography>
            ) : (
              <Typography
                sx={{ flex: "1 1 100%" }}
                variant="h6"
                id="tableTitle"
                component="div"
              >
                Versions
              </Typography>
            )}
            {numSelected > 0 ? (
              <Tooltip title="Delete">
                <IconButton onClick={DeleteVersion}>
                  <DeleteIcon />
                </IconButton>
              </Tooltip>
            ) : null}
          </Toolbar>
          <TableContainer>
            <Table sx={{ width: "100%" }}>
              {listVersions
                ? listVersions.map((current_version, index) => {
                    const isItemSelected = isSelected(index + 1);
                    const labelId = `enhanced-table-checkbox-${index}`;
                    return (
                      <TableRow
                        hover
                        onClick={() => handleClick(index + 1, current_version)}
                        role="checkbox"
                        aria-checked={isItemSelected}
                        tabIndex={-1}
                        key={index + 1}
                        selected={isItemSelected}
                        sx={{ cursor: "pointer" }}
                      >
                        <TableCell padding="checkbox">
                          <Checkbox
                            color="primary"
                            checked={isItemSelected}
                            inputProps={{
                              "aria-labelledby": labelId,
                            }}
                          />
                        </TableCell>
                        <TableCell>{current_version}</TableCell>
                      </TableRow>
                    );
                  })
                : null}
              <TableRow>
                <TableCell colSpan={3}>
                  <TextField
                    fullWidth
                    id="input-with-icon-textfield"
                    label="Add new version"
                    value={version}
                    onChange={(e: any) => setVersion(e.currentTarget.value)}
                    InputProps={{
                      endAdornment: (
                        <InputAdornment
                          onClick={() => {
                            registerVersion(client, path, version);
                            setVersion("");
                          }}
                          position="end"
                        >
                          <AddCircleIcon fontSize="large" color="success" />
                        </InputAdornment>
                      ),
                    }}
                    variant="standard"
                  />
                </TableCell>
              </TableRow>
            </Table>
          </TableContainer>
        </Paper>
        <TableContainer>
          <Table sx={{ width: "100%" }}>
            <TableHead>
              <TableRow>
                <TableCell>Packages</TableCell>
                <TableCell />
              </TableRow>
            </TableHead>
            {listPackages
              ? listPackages.map((bin: any) => (
                  <TableRow key={bin}>
                    <TableCell>{bin}</TableCell>
                    <TableCell>
                      <IconButton>
                        <DeleteIcon />
                      </IconButton>
                    </TableCell>
                  </TableRow>
                ))
              : null}
          </Table>
        </TableContainer>
        <TextField
          id="input-with-icon-textfield"
          label="Add new package"
          InputProps={{
            endAdornment: (
              <InputAdornment
                onClick={() => registerPackage(client, path, pack)}
                position="end"
              >
                <AddCircleIcon color="success" />
              </InputAdornment>
            ),
          }}
          variant="standard"
        />
        Upload Binaries
        <DropzoneArea fileObjects={fileObjects} />
        <Button
          color="primary"
          sx={{
            position: "absolute",
            right: "0",
          }}
          //        onClick={uploadFile}
        >
          Submit
        </Button>
      </Box>
    );
  }

  return <div> {speedupdatecomponent} </div>;
}

export default Speedupdate;
