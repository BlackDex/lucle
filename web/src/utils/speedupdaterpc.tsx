const token = localStorage.getItem("token");
const headers = new Headers();
headers.set("Authorization", `Bearer ${  token}`);

export const init = async (client: any, path: string) => new Promise((resolve, reject) => {
    client
      .init(
        {
          path,
        },
        { headers },
      )
      .then(() => resolve())
      .catch((error: any) => {
        reject(error);
      });
  });

export const isInit = async (client: any, path: string) => new Promise((resolve, reject) => {
    client
      .is_init(
        {
          path,
        },
        { headers },
      )
      .then(() => {
        resolve();
      })
      .catch((error: string) => {
        reject(error);
      });
  });

export const setCurrentVersion = async (
  client: any,
  path: string,
  version: string,
) => new Promise((resolve, reject) => {
    client
      .set_current_version(
        {
          path,
          version,
        },
        { headers },
      )
      .then(() => {
        resolve();
      })
      .catch((error: string) => {
        reject(error);
      });
  });

export const registerVersion = async (
  client: any,
  path: string,
  version: string,
  description: string,
) => new Promise((resolve, reject) => {
    client
      .register_version({
        path,
        version,
        description,
      })
      .then(() => resolve())
      .catch((error: string) => reject(error));
  });

export const unregisterVersion = async (
  client: any,
  path: string,
  version: string,
) => new Promise((resolve, reject) => {
    client
      .unregister_version(
        {
          path,
          version,
        },
        { headers },
      )
      .then(() => resolve())
      .catch((error: string) => reject(error));
  });

export const registerPackage = async (
  client: any,
  path: string,
  name: string,
) => new Promise((resolve, reject) => {
    client
      .register_package(
        {
          path,
          name,
        },
        { headers },
      )
      .then(() => resolve())
      .catch((error: string) => reject(error));
  });

export const unregisterPackage = async (
  client: any,
  path: string,
  name: string,
) => new Promise((resolve, reject) => {
    client
      .unregister_package(
        {
          path,
          name,
        },
        { headers },
      )
      .then(() => resolve())
      .catch((error: string) => reject(error));
  });

export const fileToDelete = async (client: any, file: string) => new Promise((resolve, reject) => {
    client
      .delete_file({
        file,
      })
      .then(() => resolve())
      .catch((error: string) => reject(error));
  });
