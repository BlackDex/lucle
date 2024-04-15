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
import IconButton from "@mui/material/IconButton";
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
  const [listVersions, setListVersions] = useState<string[]>([]);
  const [path, setPath] = useState<string>(localStorage.getItem("path") || "");
  const [fileObjects, setFileObjects] = useState();
  const [error, setError] = useState<String>("");

   useEffect(() => {
    if (client) {
      status(client, path).then((repo: any) => {
        setRepoInit(true);
        setCurrentVersion(repo.currentVersion);
        setListVersions(repo.listVersion);
        setListPackages(repo.packages);
      });
    }
  });

  const Connection = () => {
    const transport = createGrpcWebTransport({
      baseUrl: 'http://0.0.0.0:50051',
    });
    const newclient = createPromiseClient(Repo, transport);
    setClient(newclient);
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
  }   else if (!repoInit) {
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
  }  else {
    speedupdatecomponent = (
      <div>
        <Paper sx={{ width: "65%", mb: 2 }}>
          <TableContainer>
            <Table sx={{ width: "100%" }}>
              <TableHead>
                <TableRow>
                  <TableCell>Versions</TableCell>
                  <TableCell />
                </TableRow>
              </TableHead>
              {listVersions.map((current_version: any) => (
                <TableRow key={current_version}>
                  <TableCell>{current_version}</TableCell>
                  <TableCell>
                    <IconButton
                      onClick={() => unregisterVersion(client, path, pack)}
                    >
                      <DeleteIcon />
                    </IconButton>
                  </TableCell>
                </TableRow>
              ))}
            </Table>
          </TableContainer>
        </Paper>
        <TextField
          id="input-with-icon-textfield"
          label="Add new version"
          value={version}
          onChange={(e: any) => setVersion(e.currentTarget.value)}
          InputProps={{
            endAdornment: (
              <InputAdornment
                onClick={() => {
                  // register_version(client, path, version);
                  setVersion("");
                }}
                position="end"
              >
                <AddCircleIcon color="success" />
              </InputAdornment>
            ),
          }}
          variant="standard"
        />
        <Paper sx={{ width: "65%", mb: 2 }}>
          <TableContainer>
            <Table sx={{ width: "100%" }}>
              <TableHead>
                <TableRow>
                  <TableCell>Packages</TableCell>
                  <TableCell />
                </TableRow>
              </TableHead>
              {listPackages.map((bin: any) => (
                <TableRow key={bin}>
                  <TableCell>{bin}</TableCell>
                  <TableCell>
                    <IconButton>
                      <DeleteIcon />
                    </IconButton>
                  </TableCell>
                </TableRow>
              ))}
            </Table>
          </TableContainer>
        </Paper>
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
        /* Upload Binaries
        <DropzoneArea fileObjects={fileObjects} />
        <Button
          color="primary"
          sx={{
            position: "absolute",
            right: "0",
          }}
          //onClick={uploadFile}
        >
          Submit
        </Button>*/
         )
      </div>
    );
  }

  return (<div> {speedupdatecomponent} </div>);
}

export default Speedupdate;
