export const init = async (client: any, path: string) => {
  const call = client
    .init({
      path,
    })
    .then((value: any) => {
      if (value.length === 0) {
        return true;
      }
    })
    .catch((error: any) => {
      return error;
    });
};

export const isInit = async (client: any, path: string) => {
  const call = client
    .is_init({
      path,
    })
    .then((value: any) => {
      return true;
    })
    .catch((error: any) => {
      return error;
    });
};

export const status = async (client: any, path: string) => {
  const call = client.status({
    path: path,
  });
  for await (const response of call) {
    return {
      size: response.size,
      repoinit: response.repoinit,
      currentVersion: response.currentVersion,
      listVersion: response.versions,
      listPackages: response.packages,
      availablePackages: response.availablePackages,
      availableBinaries: response.availableBinaries,
    };
  }
};

export const setCurrentVersion = async (
  client: any,
  path: string,
  version: string,
) => {
  client.set_current_version({
    path,
    version,
  });
};

export const registerVersion = async (
  client: any,
  path: string,
  version: string,
) => {
  client.register_version({
    path,
    version,
  });
};

export const unregisterVersion = async (
  client: any,
  path: string,
  version: string,
) => {
  client.unregister_version({
    path,
    version,
  });
};

export const registerPackage = async (
  client: any,
  path: string,
  name: string,
) => {
  client.register_package({
    path,
    name,
  });
};

export const unregisterPackage = async (
  client: any,
  path: string,
  name: string,
) => {
  client.unregister_package({
    path,
    name,
  });
};

export const fileToDelete = async (client: any, file: string) => {
  client.delete_file({
    file: file,
  });
};
