import { useState, useMemo, useEffect, useContext } from "react";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import TableRow from "@mui/material/TableRow";
import TableHead from "@mui/material/TableHead";
import TableCell from "@mui/material/TableCell";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TablePagination from "@mui/material/TablePagination";
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
import PublishIcon from "@mui/icons-material/Publish";
import UnpublishedIcon from "@mui/icons-material/Unpublished";
import ExitToAppIcon from "@mui/icons-material/ExitToApp";

// RPC Connect
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { createPromiseClient } from "@connectrpc/connect";
import { Repo } from "gen/speedupdate_connect";

// api
import {
  init,
  isInit,
  status,
  registerVersion,
  unregisterVersion,
  setCurrentVersion,
  registerPackage,
  unregisterPackage,
  fileToDelete,
} from "utils/speedupdaterpc";

//import { uploadFile } from "utils/minio";

enum RepoState {
  NotConnected,
  Connected,
  NotInitialized,
  Initialized,
}

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
  const [url, setUrl] = useState<string>(localStorage.getItem("url") || "");
  const [currentVersion, getCurrentVersion] = useState<string>("");
  const [size, setSize] = useState<number>();
  const [pack, setPack] = useState<any>();
  const [version, setVersion] = useState<any>();
  const [canBePublished, setCanBePublished] = useState<boolean[]>([]);
  const [listPackages, setListPackages] = useState<String[]>([]);
  const [availableBinaries, setAvailableBinaries] = useState<String[]>([]);
  const [listVersions, setListVersions] = useState<any>();
  const [selectedVersionsValues, setSelectedVersionsValues] = useState<
    kstring[]
  >([]);
  const [binariesPage, setBinariesPage] = useState(0);
  const [packagesPage, setPackagesPage] = useState(0);
  const [versionsPage, setVersionsPage] = useState(0);
  const [visibleVersions, setVisibleVersions] = useState<string[]>([]);
  const [visiblePackages, setVisiblePackages] = useState<string[]>([]);
  const [visibleBinaries, setVisibleBinaries] = useState<string[]>([]);
  const [path, setPath] = useState<string>(localStorage.getItem("path") || "");
  const [fileObjects, setFileObjects] = useState();
  const [files, setFiles] = useState<any>();
  const [packagesPerPage, setPackagesPerPage] = useState(5);
  const [versionsPerPage, setVersionsPerPage] = useState(5);
  const [binariesPerPage, setBinariesPerPage] = useState(5);
  const [repoState, setRepoState] = useState<RepoState>(RepoState.NotConnected);
  const [error, setError] = useState<String>("");
  const [selectedVersions, setSelectedVersions] = useState<readonly number[]>(
    [],
  );
  const [selectedPackages, setSelectedPackages] = useState<readonly number[]>(
    [],
  );
  const [selectedPackagesValues, setSelectedPackagesValues] = useState<
    string[]
  >([]);
  const [selectedBinaries, setSelectedBinaries] = useState<readonly number[]>(
    [],
  );

  const isVersionsSelected = (id: number) =>
    selectedVersions.indexOf(id) !== -1;
  const numVersionsSelected = selectedVersions.length;

  const isPackagesSelected = (id: number) =>
    selectedPackages.indexOf(id) !== -1;
  const numPackagesSelected = selectedPackages.length;

  const isBinariesSelected = (id: number) =>
    selectedBinaries.indexOf(id) !== -1;
  const numBinariesSelected = selectedBinaries.length;

  useEffect(() => {
    async function Status() {
      const call = client.status({
        path: path,
      });
      for await (const repo of call) {
        setSize(repo.size);
        getCurrentVersion(repo.currentVersion);
        setListVersions(repo.versions);
        let fullListPackages = [];
        repo.packages.map((row) => {
          fullListPackages.push({ name: row, published: true });
        });
        repo.availablePackages.map((row) => {
          fullListPackages.push({ name: row, published: false });
        });
        setListPackages(fullListPackages);
        setAvailableBinaries(repo.availableBinaries);
      }
    }

    if (listVersions) {
      setVisibleVersions(
        listVersions.slice(
          versionsPage * versionsPerPage,
          versionsPage * versionsPerPage + versionsPerPage,
        ),
      );
    }

    if (listPackages) {
      setVisiblePackages(
        listPackages.slice(
          packagesPage * packagesPerPage,
          packagesPage * packagesPerPage + packagesPerPage,
        ),
      );
    }

    if (availableBinaries) {
      setVisibleBinaries(
        availableBinaries.slice(
          binariesPage * binariesPerPage,
          binariesPage * binariesPerPage + binariesPerPage,
        ),
      );
    }

    if (repoState == RepoState.Initialized) {
      Status().catch((err) => {
        setError(err);
      });
    }
  });

  const Connection = () => {
    const transport = createGrpcWebTransport({
      baseUrl: url,
    });

    let newClient = createPromiseClient(Repo, transport);
    setClient(newClient);
    isInit(newClient, path)
      .then(() => setRepoState(RepoState.Initialized))
      .catch((err) => {
        if (err.code == 2) {
          setError(err.message);
          setRepoState(RepoState.NotConnected);
        }
        if (err.code == 13) {
          setRepoState(RepoState.NotInitialized);
        }
      });
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

  const RegisterPackages = () => {
    selectedPackagesValues.forEach((pack) => {
      registerPackage(client, path, pack);
    });
    setSelectedPackages([]);
    setSelectedPackagesValues([]);
    setCanBePublished([]);
  };

  const UnregisterPackages = () => {
    selectedPackagesValues.forEach((pack) => {
      unregisterPackage(client, path, pack);
    });
    setSelectedPackages([]);
    setSelectedPackagesValues([]);
    setCanBePublished([]);
  };

  const DeleteVersion = () => {
    selectedVersions.forEach((version) => {
      unregisterVersion(client, path, version);
    });
  };

  const DeletePackages = () => {
    selectedPackages.forEach((row) => {
      if (listPackages[row].published) {
        unregisterPackage(client, path, listPackages[row].name);
      }
      fileToDelete(client, listPackages[row].name);
      setSelectedPackages([]);
    });
  };

  const versionsSelection = (id: number, version: string) => {
    const selectedIndex = selectedVersions.indexOf(id);
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

    setSelectedVersions(newSelected);

    if (newSelected.includes(id)) {
      setSelectedVersionsValues((previous_version) => [
        ...previous_version,
        version,
      ]);
    } else {
      const updatedVersions = selectedVersions.filter((ver) => ver !== version);
      setSelectedVersionsValues(updatedVersions);
    }
  };

  const packagesSelection = (id: number, pack: String, published: boolean) => {
    const selectedIndex = selectedPackages.indexOf(id);
    let newSelected: readonly number[] = [];
    let newPublished: readonly boolean[] = [];
    let packagesValues: readonly String[] = [];

    if (selectedIndex === -1) {
      newSelected = newSelected.concat(selectedPackages, id);
      packagesValues = packagesValues.concat(selectedPackagesValues, pack);
      newPublished = newPublished.concat(canBePublished, published);
    } else if (selectedIndex === 0) {
      newSelected = newSelected.concat(selectedPackages.slice(1));
      packagesValues = packagesValues.concat(selectedPackagesValues.slice(1));
      newPublished = newPublished.concat(canBePublished.slice(1));
    } else if (selectedIndex === selectedPackages.length - 1) {
      newSelected = newSelected.concat(selectedPackages.slice(0, -1));
      packagesValues = packagesValues.concat(
        selectedPackagesValues.slice(0, -1),
      );
      newPublished = newPublished.concat(canBePublished.slice(0, -1));
    } else if (selectedIndex > 0) {
      newSelected = newSelected.concat(
        selectedPackages.slice(0, selectedIndex),
        selectedPackages.slice(selectedIndex + 1),
      );
    }

    setSelectedPackages(newSelected);
    setCanBePublished(newPublished);
    setSelectedPackagesValues(packagesValues);
  };

  let speedupdatecomponent;

  if (repoState == RepoState.NotConnected) {
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
        <TextField
          id="outlined-required"
          label="path"
          value={path}
          onChange={(e: any) => {
            setPath(e.currentTarget.value);
            localStorage.setItem("path", e.currentTarget.value);
          }}
        />
        {repoState == RepoState.NotConnected ? (
          <Button variant="contained" onClick={Connection}>
            Connection
          </Button>
        ) : null}
        {repoState == RepoState.NotInitialized ? (
          <Button
            variant="contained"
            onClick={() =>
              init(client, path)
                .then(() => setRepoState(RepoState.initialized))
                .catch((err) => setError(error))
            }
          >
            Initialize repo
          </Button>
        ) : null}
        <p>{error.message}</p>
      </div>
    );
  } else {
    if (size && repoState == RepoState.Initialized) {
      speedupdatecomponent = (
        <Box sx={{ width: "100%" }}>
          <Paper sx={{ width: "100%", mb: 2 }}>
            <p>Current version: {currentVersion}</p>
            Total packages size : {size + DisplaySizeUnit(size)}
            <p>
              <IconButton
                size="large"
                onClick={() => {
                  setClient(null);
                  setRepoState(RepoState.NotConnected);
                }}
              >
                <ExitToAppIcon />
              </IconButton>
            </p>
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
                      setCurrentVersion(client, path, selectedVersions[0]);
                      //setVersionsSelected([]);
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
                {visibleVersions
                  ? visibleVersions.map((current_version, index) => {
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
            <TablePagination
              rowsPerPageOptions={[5, 10, 25]}
              component="div"
              count={listVersions.length}
              rowsPerPage={versionsPerPage}
              page={versionsPage}
              labelRowsPerPage="Versions per page"
              onPageChange={(event, newPage) => setVersionsPage(newPage)}
              onRowsPerPageChange={(event) => {
                setVersionsPerPage(parseInt(event.target.value, 10));
                setVersionsPage(0);
              }}
            />
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
                {numPackagesSelected > 0 && !canBePublished.includes(false) ? (
                  <Tooltip title="Unpublish">
                    <IconButton
                      onClick={() =>
                        UnregisterPackages(client, path, selectedPackages)
                      }
                    >
                      <UnpublishedIcon />
                    </IconButton>
                  </Tooltip>
                ) : null}
                {numPackagesSelected > 0 && !canBePublished.includes(true) ? (
                  <Tooltip title="Publish">
                    <IconButton
                      onClick={() =>
                        RegisterPackages(client, path, selectedPackages)
                      }
                    >
                      <PublishIcon />
                    </IconButton>
                  </Tooltip>
                ) : null}
                {numPackagesSelected > 0 ? (
                  <Tooltip title="Delete">
                    <IconButton onClick={DeletePackages}>
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
                    {visiblePackages
                      ? visiblePackages.map((pack, index) => {
                          const isItemSelected = isPackagesSelected(index);
                          const labelId = `enhanced-table-checkbox-${index}`;
                          return (
                            <TableRow
                              hover
                              onClick={() =>
                                packagesSelection(
                                  index,
                                  pack.name,
                                  pack.published,
                                )
                              }
                              role="checkbox"
                              aria-checked={isItemSelected}
                              tabIndex={-1}
                              key={index}
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
                              <TableCell>{pack.name}</TableCell>
                              <TableCell>{pack.published.toString()}</TableCell>
                            </TableRow>
                          );
                        })
                      : null}
                  </TableBody>
                </Table>
              </TableContainer>
              <TablePagination
                rowsPerPageOptions={[5, 10, 25]}
                component="div"
                count={listPackages.length}
                rowsPerPage={packagesPerPage}
                page={packagesPage}
                labelRowsPerPage="Packages per page"
                onPageChange={(event, newPage) => setPackagesPage(newPage)}
                onRowsPerPageChange={(event) => {
                  setPackagesPerPage(parseInt(event.target.value, 10));
                  setPackagesPage(0);
                }}
              />
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
                  {visibleBinaries
                    ? visibleBinaries.map((binary, index) => {
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
              <TablePagination
                rowsPerPageOptions={[5, 10, 25]}
                component="div"
                count={availableBinaries.length}
                rowsPerPage={binariesPerPage}
                page={binariesPage}
                onPageChange={(event, newPage) => setBinariesPage(newPage)}
                onRowsPerPageChange={(event) => {
                  setBinariesPerPage(parseInt(event.target.value, 10));
                  setBinariesPage(0);
                }}
              />
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
  }
  return <div> {speedupdatecomponent} </div>;
}

export default Speedupdate;
