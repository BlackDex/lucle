const token = localStorage.getItem("token");
const headers = new Headers();
headers.set("Authorization", "Bearer " + token);

export const init = async (client: any, path: string) => {
  return new Promise((resolve, reject) => {
    client
      .init(
        {
          path,
        },
        { headers: headers },
      )
      .then(() => resolve())
      .catch((error: any) => {
        reject(error);
      });
  });
};

export const isInit = async (client: any, path: string) => {
  return new Promise((resolve, reject) => {
    client
      .is_init(
        {
          path,
        },
        { headers: headers },
      )
      .then(() => {
        resolve();
      })
      .catch((error: string) => {
        reject(error);
      });
  });
};

export const setCurrentVersion = async (
  client: any,
  path: string,
  version: string,
) => {
  client.set_current_version(
    {
      path,
      version,
    },
    { headers: headers },
  );
};

export const registerVersion = async (
  client: any,
  path: string,
  version: string,
) => {
  return new Promise((resolve, reject) => {
    client
      .register_version({
        path,
        version,
      })
      .then(() => resolve())
      .catch((error: string) => reject(error));
  });
};

export const unregisterVersion = async (
  client: any,
  path: string,
  version: string,
) => {
  client.unregister_version(
    {
      path,
      version,
    },
    { headers: headers },
  );
};

export const registerPackage = async (
  client: any,
  path: string,
  name: string,
) => {
  client.register_package(
    {
      path,
      name,
    },
    { headers: headers },
  );
};

export const unregisterPackage = async (
  client: any,
  path: string,
  name: string,
) => {
  client.unregister_package(
    {
      path,
      name,
    },
    { headers: headers },
  );
};

export const fileToDelete = async (client: any, file: string) => {
  client.delete_file({
    file: file,
  });
};
