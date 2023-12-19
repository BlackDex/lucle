import rgba from "assets/theme/functions/rgba";
import pxToRem from "assets/theme/functions/pxToRem";

function boxShadow(color, opacity, inset = "", offset = [], radius = []) {
  const [x, y] = offset;
  const [blur, spread] = radius;

  return `${inset} ${pxToRem(x)} ${pxToRem(y)} ${pxToRem(blur)} ${pxToRem(
    spread,
  )} ${rgba(color, opacity)}`;
}

export default boxShadow;
