import { forwardRef } from "react";

// prop-types is a library for typechecking of props
import PropTypes from "prop-types";

// Custom styles for MKSocialButton
import MKSocialButtonRoot from "components/SocialButton/SocialButtonRoot";

const SocialButton = forwardRef(
  ({ color, size, iconOnly, circular, children }, ref) => (
    <MKSocialButtonRoot
      ref={ref}
      variant="contained"
      color="primary"
      size={size}
      ownerState={{ color, size, iconOnly, circular }}
    >
      {children}
    </MKSocialButtonRoot>
  ),
);

SocialButton.displayName = 'SocialButton';

// Setting default values for the props of MKSocialButton
SocialButton.defaultProps = {
  size: "medium",
  color: "facebook",
  iconOnly: false,
  circular: false,
};

// Typechecking props for the MKSocialButton
SocialButton.propTypes = {
  size: PropTypes.oneOf(["small", "medium", "large"]),
  color: PropTypes.oneOf([
    "facebook",
    "twitter",
    "instagram",
    "linkedin",
    "pinterest",
    "youtube",
    "github",
    "vimeo",
    "slack",
    "dribbble",
    "reddit",
    "tumblr",
  ]),
  iconOnly: PropTypes.bool,
  circular: PropTypes.bool,
  children: PropTypes.node.isRequired,
};

export default SocialButton;
