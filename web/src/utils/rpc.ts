import {
    createGrpcWebTransport,
    createPromiseClient,
} from '@bufbuild/connect-web';
import { Lucle } from 'gen/lucle_connectweb';
import { DatabaseType } from 'gen/lucle_pb';

export const connect = (url: string, port: string) => { 
  const client = createPromiseClient(
        Lucle,
        createGrpcWebTransport({
            baseUrl: 'http://' + url + ':' + port,
        })
    )
    return client;
}

export const install = async(client: any, db: number) => {
  const call = client.install({
   dbType: db,
  });
  const response = await call.response;
  console.log(JSON.stringify(response));
};

export const init = async(client: any, path: string) => {
    const call = client.init({
      path: path
    });
    const status = await call.status;
    if (status.code === "OK" ) {
      return true;
    }
    else
      return false;
}

export const status = async(client: any, path: string) => {  
  const call = client.status({
      path: path
    });

    //const status = await call.status;
    //if (status.code === "OK") {
      //const trailers = await call.responses;
      for await (let response of call.responses) {
        if (response.repoinit) {
	  return({
	    'repoinit': true,
	    'currentversion': response.currentVersion,
            'listVersion': response.version,
	    'istPackages': response.packages,
	   });
        }
      }
   // }
  }

export const set_current_version = async(client: any, path: string, version: string) => {
    const call = client.setCurrentVersion({
      path: path,
      version: version
    });
  }

export const register_version = async(client: any, path: string, version: string) => {
    client.registerVersion({
      path: path,
      version: version,
    });
  }

export const unregister_version = async(client: any, path: string, version: string) => { 
    client.unregisterVersion({
      path: path,
      version: version,
    });
  }

export const register_package = async(client: any, path: string, name: string) => { 
    client.registerPackage({
      path: path,
      name: name,
    });
  }

 export const unregister_package = async(client: any, path: string, name: string) => {
    client.unregisterPackage({
      path: path,
      name: name,
    });
  }
