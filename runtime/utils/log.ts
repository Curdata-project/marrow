import * as signale from "signale";

signale.config({
  displayFilename: true,
  displayTimestamp: true,
});

export const log = () => {
  return signale;
};
