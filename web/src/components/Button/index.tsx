import { forwardRef } from "react";

// prop-types is a library for typechecking of props
import PropTypes from "prop-types";

// Custom styles for MDButton
import ButtonRoot from "components/Button/ButtonRoot";

// Material Dashboard 2 React contexts
import { useMaterialUIController } from "context";

const Button = forwardRef(
  ({ color, variant, size, circular, iconOnly, children }, ref) => {
    const [controller] = useMaterialUIController();
    const { darkMode } = controller;

    return (
      <ButtonRoot
        ref={ref}
        color="primary"
        variant={variant === "gradient" ? "contained" : variant}
        size={size}
        ownerState={{ color, variant, size, circular, iconOnly, darkMode }}
      >
        {children}
      </ButtonRoot>
    );
  },
);

Button.displayName = "Button";

// Setting default values for the props of MDButton
Button.defaultProps = {
  size: "medium",
  variant: "contained",
  color: "white",
  circular: false,
  iconOnly: false,
};

// Typechecking props for the MDButton
Button.propTypes = {
  size: PropTypes.oneOf(["small", "medium", "large"]),
  variant: PropTypes.oneOf(["text", "contained", "outlined", "gradient"]),
  color: PropTypes.oneOf([
    "white",
    "primary",
    "secondary",
    "info",
    "success",
    "warning",
    "error",
    "light",
    "dark",
  ]),
  circular: PropTypes.bool,
  iconOnly: PropTypes.bool,
  children: PropTypes.node.isRequired,
};

export default Button;
