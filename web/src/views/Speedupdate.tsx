import { useEffect, useState, useCallback } from 'react';
import {
    createConnectTransport,
    createPromiseClient,
} from '@bufbuild/connect-web';
import { Repo } from 'gen/speedupdate_connectweb';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import TableRow from '@mui/material/TableRow'
import TableHead from '@mui/material/TableHead';
import TableCell from '@mui/material/TableCell';
import Table from '@mui/material/Table';
import TableContainer from '@mui/material/TableContainer';
import InputAdornment from '@mui/material/InputAdornment';
import Paper from '@mui/material/Paper';
import IconButton from '@mui/material/IconButton';
import { DropzoneArea, DropzoneDialog } from "mui-file-dropzone";

//Icons
import AddCircleIcon from '@mui/icons-material/AddCircle';
import DeleteIcon from '@mui/icons-material/Delete';

const Speedupdate = () => {
  const [repoInit, setRepoInit] = useState<boolean>(false);
  const [url, setUrl] = useState<string>(localStorage.getItem('url') || "");
  const [currentVersion, setCurrentVersion] = useState<string>("");
  const [pack, setPack] = useState<any>();
  const [version, setVersion] = useState<any>();
  const [listPackages, setListPackages] = useState<string[]>([]);
  const [listVersions, setListVersions] = useState<string[]>([]);
  const [path, setPath] = useState<string>(localStorage.getItem('path') || "");
  const [client, setClient] = useState<any>();
  const [fileObjects, setFileObjects] = useState();

  const init = async(client: any, path: string) => {
    const call = client.init({
      path: path
    });
    const status = await call.status;
    if (status.code === "OK" ) {
      setRepoInit(true);
    }
  }

  const status = async(client: any, path: string) => {  
  const call = client.status({
      path: path
    });
    //TODO : Delete this
    setRepoInit(true);
    //const status = await call.status;
    //if (status.code === "OK") {
      //const trailers = await call.responses;
      for await (let response of call.responses) {
        if (response.repoinit) {
          setRepoInit(true);
          setCurrentVersion(response.currentVersion);
          setListVersions(response.versions);
	  setListPackages(response.packages);
        }
      }
   // }
  }
 
  const set_current_version = async(client: any, path: string, version: string) => {
    const call = client.setCurrentVersion({
      path: path,
      version: version
    });
    const response = await call.response;
  }

  const register_version = async(client: any, path: string, version: string) => {
    const call = client.registerVersion({
      path: path,
      version: version,
    });
    const response = await call.response;
  }

  const unregister_version = async(client: any, path: string, version: string) => { 
    const call = client.unregisterVersion({
      path: path,
      version: version,
    });
    const response = await call.response;
  }

  const register_package = async(client: any, path: string, name: string) => { 
    const call = client.registerPackage({
      path: path,
      name: name,
    });
    const response = await call.response;
  }

  const unregister_package = async(client: any, path: string, name: string) => {
    const call = client.unregisterPackage({
      path: path,
      name: name,
    });
    const response = await call.response;
  }

  useEffect(() => {
    const client = createPromiseClient(
        Repo,
        createConnectTransport({
            baseUrl: 'https://127.0.0.1:3000',
        })
    )
    setClient(client);
    status(client, path);
  }, [listVersions, listPackages]);

  return(
    <div>
      <TextField
        id="outlined-required"
        label="url"
        value={url}
        onChange={e => {
	  setUrl(e.currentTarget.value);
	  localStorage.setItem("url", e.currentTarget.value);
	  }
	}
      />
      <TextField
        id="outlined-required"
        label="path"
        value={path}
        onChange={e => {
          setPath(e.currentTarget.value);
          localStorage.setItem("path", e.currentTarget.value);
          }
        }
      />
      {
        !repoInit ? (
          <div>
            <Button 
              variant="contained"
              onClick={() => init(client, path)}
            >
              Initialize repository
            </Button>
	  </div>
	    ) : (
		<div>
	    <Paper sx={{ width: '65%', mb: 2 }}>
	    <TableContainer>
	    <Table
	      sx={{ width: '100%'}}
	    >
	      <TableHead>
                <TableRow>
                  <TableCell>
		    Versions
		  </TableCell>
		  <TableCell>
		  </TableCell>
                </TableRow>
	      </TableHead>
		{listVersions.map((version) => (
		  <TableRow>
                    <TableCell>
                     {version}
                    </TableCell>
		    <TableCell>
		      <IconButton
		        onClick={() => unregister_version(client, path, version)}
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
	onChange={e => setVersion(e.currentTarget.value)}
        InputProps={{
          endAdornment: (
            <InputAdornment 
	      onClick={() => {
		//register_version(client, path, version);
		setVersion("");
		}
	      } 
	      position="end">
              < AddCircleIcon color="success"/>
            </InputAdornment>
          ),
        }}
        variant="standard"
      />
	  <Paper sx={{ width: '65%', mb: 2 }}>
            <TableContainer>
            <Table
              sx={{ width: '100%'}}
            >
              <TableHead>
                <TableRow>
                  <TableCell>
                    Packages
                  </TableCell>
                  <TableCell>
                  </TableCell>
                </TableRow>
              </TableHead>
                {listPackages.map((bin) => (
                  <TableRow>
                    <TableCell>
                     {bin}
                    </TableCell>
                    <TableCell>
                      <IconButton
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
        label="Add new package"
        InputProps={{
          endAdornment: (
            <InputAdornment
              onClick={() => register_package(client, path, pack)}
              position="end">
              < AddCircleIcon color="success"/>
            </InputAdornment>
          ),
        }}
        variant="standard"
      />
	</div>
    )}
    Upload Binaries
    <DropzoneArea
	fileObjects={fileObjects}
     />
     <Button
       color="primary"
       sx={{
	position: 'absolute',
	right: '0'
       }}
     >
     Submit
     </Button>
    </div>
  );
}

export default Speedupdate;
