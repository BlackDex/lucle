import { useState, useEffect, useContext } from "react";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import TableRow from "@mui/material/TableRow";
import TableHead from "@mui/material/TableHead";
import TableCell from "@mui/material/TableCell";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
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
import CheckIcon from "@mui/icons-material/Check";

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
  setCurrentVersion,
  registerPackage,
} from "utils/speedupdaterpc";

//import { uploadFile } from "utils/minio";

const DisplaySizeUnit = (TotalSize) => {
  if (TotalSize > 0 && TotalSize < 1024) {
    return "B";
  }
  if (TotalSize < 1024 * 1024) {
    return "kB";
  }
  if (TotalSize < 1024 * 1024 * 1024) {
    return "MB";
  }
  if (TotalSize < 1024 * 1024 * 1024 * 1024) {
    return "GB";
  }
  return "-";
};

function Speedupdate() {
  const [client, setClient] = useState<any>();
  const [repoInit, setRepoInit] = useState<boolean>(false);
  const [url, setUrl] = useState<string>(localStorage.getItem("url") || "");
  const [currentVersion, getCurrentVersion] = useState<string>("");
  const [size, setSize] = useState<number>();
  const [pack, setPack] = useState<any>();
  const [version, setVersion] = useState<any>();
  const [listPackages, setListPackages] = useState<String[]>([]);
  const [listAvailablePackages, setListAvailablePackages] = useState<String[]>(
    [],
  );
  const [availableBinaries, setAvailableBinaries] = useState<String[]>([]);
  const [listVersions, setListVersions] = useState<any>();
  const [selectedVersions, setSelectedVersions] = useState<string[]>([]);
  const [path, setPath] = useState<string>(localStorage.getItem("path") || "");
  const [fileObjects, setFileObjects] = useState();
  const [files, setFiles] = useState<any>();
  const [error, setError] = useState<String>("");
  const [versionsSelected, setVersionsSelected] = useState<readonly number[]>(
    [],
  );
  const [packagesSelected, setPackagesSelected] = useState<readonly number[]>(
    [],
  );
  const [binariesSelected, setBinariesSelected] = useState<readonly number[]>(
    [],
  );

  const isVersionsSelected = (id: number) =>
    versionsSelected.indexOf(id) !== -1;
  const numVersionsSelected = versionsSelected.length;

  const isPackagesSelected = (id: number) =>
    packagesSelected.indexOf(id) !== -1;
  const numPackagesSelected = packagesSelected.length;

  const isBinariesSelected = (id: number) =>
    binariesSelected.indexOf(id) !== -1;
  const numBinariesSelected = binariesSelected.length;

  useEffect(() => {
    if (client) {
      status(client, path)
        .then((repo: any) => {
          if (repo.repoinit) {
            setRepoInit(true);
            setSize(repo.size);
            getCurrentVersion(repo.currentVersion);
            console.log("stream : ", repo.currentVersion);
            setListVersions(repo.listVersion);
            setListPackages(repo.listPackages);
            setListAvailablePackages(repo.availablePackages);
            setAvailableBinaries(repo.availableBinaries);
          }
        })
        .catch((err) => console.log("err : ", err));
    }
  }, [client]);

  const Connection = () => {
    const transport = createGrpcWebTransport({
      baseUrl: url,
    });

    let newClient = createPromiseClient(Repo, transport);
    setClient(newClient);
  };

  const uploadFile = () => {
    let formData = new FormData();
    formData.append("file", files[0]);
    fetch("http://localhost:3000/file/" + files[0].name, {
      method: "POST",
      body: formData,
    })
      .then((val) => console.log("answer: ", val))
      .catch((err) => console.log("error: ", err));
  };

  const DeleteVersion = () => {
    selectedVersions.forEach((version) => {
      unregisterVersion(client, path, version);
    });
  };

  const versionsSelection = (id: number, version: string) => {
    const selectedIndex = versionsSelected.indexOf(id);
    let newSelected: readonly number[] = [];

    if (selectedIndex === -1) {
      newSelected = newSelected.concat(versionsSelected, id);
    } else if (selectedIndex === 0) {
      newSelected = newSelected.concat(versionsSelected.slice(1));
    } else if (selectedIndex === versionsSelected.length - 1) {
      newSelected = newSelected.concat(versionsSelected.slice(0, -1));
    } else if (selectedIndex > 0) {
      newSelected = newSelected.concat(
        versionsSelected.slice(0, selectedIndex),
        versionsSelected.slice(selectedIndex + 1),
      );
    }

    setVersionsSelected(newSelected);

    if (newSelected.includes(id)) {
      setSelectedVersions((previous_version) => [...previous_version, version]);
    } else {
      const updatedVersions = selectedVersions.filter((ver) => ver !== version);
      setSelectedVersions(updatedVersions);
    }
  };

  const packagesSelection = (id: number, pack: string) => {
    const selectedIndex = packagesSelected.indexOf(id);
    let newSelected: readonly number[] = [];

    if (selectedIndex === -1) {
      newSelected = newSelected.concat(packagesSelected, id);
    } else if (selectedIndex === 0) {
      newSelected = newSelected.concat(packagesSelected.slice(1));
    } else if (selectedIndex === packagesSelected.length - 1) {
      newSelected = newSelected.concat(packagesSelected.slice(0, -1));
    } else if (selectedIndex > 0) {
      newSelected = newSelected.concat(
        packagesSelected.slice(0, selectedIndex),
        packagesSelected.slice(selectedIndex + 1),
      );
    }

    setPackagesSelected(newSelected);
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
        {error}
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
            init(client, path)
              .then(() => setRepoInit(true))
              .catch((error: any) => {
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
          <p>Current version: {currentVersion}</p>
          Total packages size : {size + DisplaySizeUnit(size)}
        </Paper>
        <Paper sx={{ width: "100%", mb: 2 }}>
          <Toolbar
            sx={{
              pl: { sm: 2 },
              pr: { xs: 1, sm: 1 },
              ...(numVersionsSelected > 0 && {
                bgcolor: (theme) =>
                  alpha(
                    theme.palette.primary.main,
                    theme.palette.action.activatedOpacity,
                  ),
              }),
            }}
          >
            {numVersionsSelected > 0 ? (
              <Typography
                sx={{ flex: "1 1 100%" }}
                color="inherit"
                variant="subtitle1"
                component="div"
              >
                {numVersionsSelected} selected
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
            {numVersionsSelected == 1 ? (
              <Tooltip title="SetVersion">
                <IconButton
                  onClick={() => {
                    setVersionsSelected([]);
                    setCurrentVersion(client, path, selectedVersions[0]);
                  }}
                >
                  <CheckIcon />
                </IconButton>
              </Tooltip>
            ) : null}
            {numVersionsSelected > 0 ? (
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
                    const isItemSelected = isVersionsSelected(index + 1);
                    const labelId = `enhanced-table-checkbox-${index}`;
                    return (
                      <TableRow
                        hover
                        onClick={() =>
                          versionsSelection(index + 1, current_version)
                        }
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
        <Box sx={{ width: "100%" }}>
          <Paper sx={{ width: "100%", mb: 2 }}>
            <Toolbar
              sx={{
                pl: { sm: 2 },
                pr: { xs: 1, sm: 1 },
                ...(numPackagesSelected > 0 && {
                  bgcolor: (theme) =>
                    alpha(
                      theme.palette.primary.main,
                      theme.palette.action.activatedOpacity,
                    ),
                }),
              }}
            >
              {numPackagesSelected > 0 ? (
                <Typography
                  sx={{ flex: "1 1 100%" }}
                  color="inherit"
                  variant="subtitle1"
                  component="div"
                >
                  {numPackagesSelected} selected
                </Typography>
              ) : (
                <Typography
                  sx={{ flex: "1 1 100%" }}
                  variant="h6"
                  id="tableTitle"
                  component="div"
                >
                  Packages
                </Typography>
              )}
              {numPackagesSelected > 0 ? (
                <Tooltip title="Delete">
                  <IconButton onClick={() => {}}>
                    <DeleteIcon />
                  </IconButton>
                </Tooltip>
              ) : null}
            </Toolbar>
            <TableContainer>
              <Table sx={{ width: "100%" }}>
                <TableHead>
                  <TableRow>
                    <TableCell></TableCell>
                    <TableCell>Name</TableCell>
                    <TableCell>Published</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {listPackages
                    ? listPackages.map((bin, index) => {
                        const isItemSelected = isPackagesSelected(index + 1);
                        const labelId = `enhanced-table-checkbox-${index}`;
                        return (
                          <TableRow
                            hover
                            onClick={() => packagesSelection(index + 1, bin)}
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
                            <TableCell>{bin}</TableCell>
                            <TableCell>true</TableCell>
                          </TableRow>
                        );
                      })
                    : null}
                  {listAvailablePackages
                    ? listAvailablePackages.map((bin, index) => {
                        const isItemSelected = isPackagesSelected(
                          listPackages.length + 1,
                        );
                        const labelId = `enhanced-table-checkbox-${index}`;
                        return (
                          <TableRow
                            hover
                            onClick={() =>
                              packagesSelection(listPackages.length + 1, bin)
                            }
                            role="checkbox"
                            aria-checked={isItemSelected}
                            tabIndex={-1}
                            key={index + listPackages.length}
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
                            <TableCell>{bin}</TableCell>
                            <TableCell>false</TableCell>
                          </TableRow>
                        );
                      })
                    : null}
                </TableBody>
              </Table>
            </TableContainer>
          </Paper>
        </Box>
        <Box>
          <Paper sx={{ width: "100%", mb: 2 }}>
            <Toolbar
              sx={{
                pl: { sm: 2 },
                pr: { xs: 1, sm: 1 },
                ...(numBinariesSelected > 0 && {
                  bgcolor: (theme) =>
                    alpha(
                      theme.palette.primary.main,
                      theme.palette.action.activatedOpacity,
                    ),
                }),
              }}
            >
              {numBinariesSelected > 0 ? (
                <Typography
                  sx={{ flex: "1 1 100%" }}
                  color="inherit"
                  variant="subtitle1"
                  component="div"
                >
                  {numBinariesSelected} selected
                </Typography>
              ) : (
                <Typography
                  sx={{ flex: "1 1 100%" }}
                  variant="h6"
                  id="tableTitle"
                  component="div"
                >
                  Binaries
                </Typography>
              )}
              {numBinariesSelected > 0 ? (
                <Tooltip title="Delete">
                  <IconButton>
                    <DeleteIcon />
                  </IconButton>
                </Tooltip>
              ) : null}
            </Toolbar>
            <TableContainer>
              <Table sx={{ width: "100%" }}>
                {availableBinaries
                  ? availableBinaries.map((binary, index) => {
                      const isItemSelected = isBinariesSelected(index + 1);
                      const labelId = `enhanced-table-checkbox-${index}`;
                      return (
                        <TableRow
                          hover
                          //onClick={() =>
                          //  versionsSelection(index + 1, current_version)
                          //}
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
                          <TableCell>{binary}</TableCell>
                        </TableRow>
                      );
                    })
                  : null}
              </Table>
            </TableContainer>
          </Paper>
        </Box>
        Upload Binaries
        <DropzoneArea
          fileObjects={fileObjects}
          onChange={(files) => setFiles(files)}
        />
        <Button
          color="primary"
          sx={{
            position: "absolute",
            right: "0",
          }}
          onClick={uploadFile}
        >
          Submit
        </Button>
      </Box>
    );
  }

  return <div> {speedupdatecomponent} </div>;
}

export default Speedupdate;
