import { useEffect, useState, useCallback } from 'react';
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

// Icons
import AddCircleIcon from '@mui/icons-material/AddCircle';
import DeleteIcon from '@mui/icons-material/Delete';

// api
import {init, unregister_version, register_package} from 'utils/rpc'

function Speedupdate() {
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
		  <TableCell />
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
		// register_version(client, path, version);
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
                  <TableCell />
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
