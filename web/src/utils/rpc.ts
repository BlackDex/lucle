import { createPromiseClient } from "@connectrpc/connect";
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { Lucle } from "gen/lucle_connect";
import { json } from "react-router";

export const connect = (url: string, port: string) => {
  const client = createPromiseClient(
    Lucle,
    createGrpcWebTransport({
      baseUrl: `http://${url}:${port}`,
    }),
  );
  return client;
};

export const check_if_installed = async (client: any) => {
  const { error } = await client.is_created_user({
    dbType: 2,
  });
  if (error) throw error;
};

export const db_connection = async (client: any, db: number) => {
  const { error } = await client.create_db({
    dbType: db,
  });
  if (error) throw error;
};

export const create_user = async (
  client: any,
  username: string,
  password: string,
) => {
  const { error } = await client.create_user({
    //TODO: delete this var
    database_path: "lucle.db",
    username: username,
    password: password,
  });
  if (error) throw error;
};

export const init = (client: any, path: string) =>
  new Promise((resolve, reject) => {
    client
      .init({
        path,
      })
      .then((status: any) => {
        if (status.code === "OK") {
          resolve(true);
        }
        resolve(false);
      })
      .catch((error: any) => reject(error.message));
  });

export const status = async (client: any, path: string) => {
  const call = client.status({
    path,
  });
  for await (const response of call.responses) {
    if (response.repoinit) {
      return {
        repoinit: true,
        currentversion: response.currentVersion,
        listVersion: response.version,
        istPackages: response.packages,
      };
    }
  }
};

export const setCurrentVersion = async (
  client: any,
  path: string,
  version: string,
) => {
  client.setCurrentVersion({
    path,
    version,
  });
};

export const registerVersion = async (
  client: any,
  path: string,
  version: string,
) => {
  client.registerVersion({
    path,
    version,
  });
};

export const unregisterVersion = async (
  client: any,
  path: string,
  version: string,
) => {
  client.unregisterVersion({
    path,
    version,
  });
};

export const registerPackage = async (
  client: any,
  path: string,
  name: string,
) => {
  client.registerPackage({
    path,
    name,
  });
};

export const unregisterPackage = async (
  client: any,
  path: string,
  name: string,
) => {
  client.unregisterPackage({
    path,
    name,
  });
};
