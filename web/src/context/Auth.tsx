import { useEffect, useContext, createContext, useState } from "react";
import { useNavigate } from "react-router-dom";

// Context
import { LucleRPC } from "context";

// RPC
import { connection } from "utils/rpc";

const AuthContext = createContext();

const AuthProvider = ({ children }) => {
  const [token, setToken] = useState(localStorage.getItem("token"));
  const [username, setUsername] = useState(localStorage.getItem("username"));
  const navigate = useNavigate();
  const client = useContext(LucleRPC);

  const Login = async (credentials) => {
    return new Promise((resolve, reject) => {
      connection(client, credentials.login, credentials.password)
        .then((user) => {
          setUsername(user.username);
          setToken(user.token);
          localStorage.setItem("token", user.token);
          localStorage.setItem("username", user.username);
          navigate("/admin");
        })
        .catch((err) => reject(err));
    });
  };

  const Logout = () => {
    setToken("");
    setUsername("");
    localStorage.removeItem("token");
    localStorage.removeItem("username");
    navigate("/login");
  };

  return (
    <AuthContext.Provider value={{ username, token, Login, Logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export default AuthProvider;

export const useAuth = () => {
  return useContext(AuthContext);
};
