import Icon from "@mui/material/Icon";
import Speedupdate from "views/Speedupdate";
import Dashboard from "layouts/Dashboard";

const adminroutes = [
  {
    type: "collapse",
    name: "Home",
    key: "admin",
    icon: <Icon fontSize="small">dashboard</Icon>,
    route: "/admin/*",
    //component: <Dashboard />,
  },
  {
    type: "collapse",
    name: "Speedupdate",
    key: "speedupdate",
    icon: <Icon fontSize="small">dashboard</Icon>,
    route: "/speedupdate",
    component: <Speedupdate />,
  },
];

export default adminroutes;
