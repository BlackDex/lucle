import { useContext, createContext, useState } from "react";
import { useNavigate } from "react-router-dom";

// Context
import { LucleRPC } from "context";

// RPC
import { connection } from "utils/rpc";

const AuthContext = createContext();

const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const navigate = useNavigate();
  const client = useContext(LucleRPC);
 
  const Login = (credentials) => {
    connection(client, credentials.login, credentials.password)
      .then((jwt) => {
        localStorage.setItem("token", jwt.token);
	navigate("/admin");
      })
      .catch((err) => console.log(err));
  };

  const Logout = () => {
    setUser(null);
    navigate("/login");
  };

return (
    <AuthContext.Provider value={{ user, Login, Logout }}>
      {children}
    </AuthContext.Provider>
  );

};

export default AuthProvider;

export const useAuth = () => {
  return useContext(AuthContext);
};
