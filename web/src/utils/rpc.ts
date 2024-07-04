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

export const dbConnection = async (client: any, db: number) => {
  const { error } = await client.create_db({
    dbType: db,
  });
  if (error) throw error;
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
