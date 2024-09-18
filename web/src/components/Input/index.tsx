import { forwardRef } from "react";

// prop-types is a library for typechecking of props
import PropTypes from "prop-types";

// Custom styles for MDInput
import InputRoot from "components/Input/InputRoot";

const Input = forwardRef(({ error, success, disabled }, ref) => (
  <InputRoot ref={ref} ownerState={{ error, success, disabled }} />
));

Input.displayName = "Input";

// Setting default values for the props of MDInput
Input.defaultProps = {
  error: false,
  success: false,
  disabled: false,
};

// Typechecking props for the MDInput
Input.propTypes = {
  error: PropTypes.bool,
  success: PropTypes.bool,
  disabled: PropTypes.bool,
};

export default Input;
