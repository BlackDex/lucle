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
  console.log("status");
  const call = client.status({
    path,
  });
  for await (const response of call) {
    console.log("allo : ", await response);
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
