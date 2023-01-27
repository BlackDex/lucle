import axios from "axios";

const host = "http://127.0.0.1";
const port = "8000";

export const get = (path: any) =>
  new Promise((resolve, reject) => {
    axios
      .get(`${host}:${port}${path}`, {})
      .then((response) => {
        resolve(response);
      })
      .catch((error) => {
        reject(error);
      });
  });

export const post = (path: any, key: any, value: any) =>
  new Promise((resolve, reject) => {
    axios
      .post(`${host}:${port}${path}`, {
        key: value,
      })
      .then((response) => {
        resolve(response);
      })
      .catch((error) => {
        reject(error);
      });
  });

export const adelete = (path: any) =>
  new Promise((resolve, reject) => {
    axios
      .delete(`${host}:${port}${path}`)
      .then((response) => {
        resolve(response);
      })
      .catch((error) => {
        reject(error);
      });
  });
