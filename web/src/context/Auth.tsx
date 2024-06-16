import { useContext, createContext, useState } from "react";
import { useNavigate } from "react-router-dom";

// Context
import { LucleRPC } from "context";

// RPC
import { connection } from "utils/rpc";

const AuthContext = createContext();

const AuthProvider = ({ children }) => {
  const [token, setToken] = useState();
  const navigate = useNavigate();
  const client = useContext(LucleRPC);

  const Login = async (credentials) => {
    return new Promise((resolve, reject) => {
      connection(client, credentials.login, credentials.password)
        .then((jwt) => {
          setToken(jwt.token);
          localStorage.setItem("token", jwt.token);
          navigate("/admin");
        })
        .catch((err) => reject(err));
    });
  };

  const Logout = () => {
    setToken("");
    localStorage.removeItem("token");
    navigate("/login");
  };

  return (
    <AuthContext.Provider value={{ token, Login, Logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export default AuthProvider;

export const useAuth = () => {
  return useContext(AuthContext);
};
