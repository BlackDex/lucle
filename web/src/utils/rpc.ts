import { createPromiseClient } from "@connectrpc/connect";
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { Lucle } from "gen/lucle_connect";

export const checkIfInstalled = async (client: any) => {
  return new Promise((resolve, reject) => {
    client
      .is_created_user({
        dbType: 2,
      })
      .then((install) => resolve(true))
      .catch((err) => reject(err));
  });
};

export const createDB = async (client: any, db: number) => {
  return new Promise((resolve, reject) => {
    client
      .create_db({
        dbType: db,
      })
      .then(() => resolve())
      .catch((err) => reject(err));
  });
};

export const forgotPassword = async (client: any, user_mail: string) => {
  const { error } = await client.forgot_password({
    email: user_mail,
  });
  if (error) throw error;
};

export const connection = async (
  client: any,
  login: string,
  user_password: string,
) => {
  return new Promise((resolve, reject) => {
    client
      .login({
        usernameOrEmail: login,
        password: user_password,
      })
      .then((token) => resolve(token))
      .catch((err) => reject(err));
  });
};

export const createUser = async (
  client: any,
  login: string,
  user_password: string,
  user_mail: string,
  role: string,
) => {
  const { error } = await client.create_user({
    // TODO: delete this var
    database_path: "lucle.db",
    username: login,
    password: user_password,
    email: user_mail,
    role: role,
  });
  if (error) throw error;
};

export const registerUpdateServer = async (
  client: any,
  username: string,
  repo: string,
) => {
  return new Promise((resolve, reject) => {
    client
      .register_update_server({
        path: repo,
        username: username,
      })
      .then(() => resolve())
      .catch((err) => reject(err));
  });
};
