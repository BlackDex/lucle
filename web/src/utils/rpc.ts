import { createPromiseClient } from "@connectrpc/connect";
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { Lucle } from "gen/lucle_connect";

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

export const forgot_password = async (client: any, email: string) => {
  const { error } = await client.forgot_password({
    email: email,
  });
  if (error) throw error;
};

export const Connection = async (
  client: any,
  username: string,
  password: string,
) => {
  const { error } = await client.login({
    username: username,
    password: password,
  });
  if (error) throw error;
};

export const create_user = async (
  client: any,
  username: string,
  password: string,
  email: string,
) => {
  const { error } = await client.create_user({
    //TODO: delete this var
    database_path: "lucle.db",
    username: username,
    password: password,
    email: email,
  });
  if (error) throw error;
};

