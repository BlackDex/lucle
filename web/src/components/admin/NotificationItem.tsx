import { forwardRef } from "react";

// prop-types is a library for typechecking of props.
import PropTypes from "prop-types";

// @mui material components
import MenuItem from "@mui/material/MenuItem";
import Link from "@mui/material/Link";

// Material Dashboard 2 React components
import Box from "components/admin/Box";
import Typography from "components/admin/Typography";

// custom styles for the NotificationItem
import menuItem from "components/admin/NotificationItemStyle";

const NotificationItem = forwardRef(({ icon, title }, ref) => (
  <MenuItem ref={ref} sx={(theme) => menuItem(theme)}>
    <Box
      component={Link}
      py={0.5}
      display="flex"
      alignItems="center"
      lineHeight={1}
    >
      <Typography variant="body1" color="secondary" lineHeight={0.75}>
        {icon}
      </Typography>
      <Typography variant="button" fontWeight="regular" sx={{ ml: 1 }}>
        {title}
      </Typography>
    </Box>
  </MenuItem>
));

NotificationItem.displayName = "NotificationItem";

// Typechecking props for the NotificationItem
NotificationItem.propTypes = {
  icon: PropTypes.node.isRequired,
  title: PropTypes.string.isRequired,
};

export default NotificationItem;
