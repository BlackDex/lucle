import { useState, useEffect, Fragment } from "react";
import Box from "@mui/material/Box";
import Stepper from "@mui/material/Stepper";
import Step from "@mui/material/Step";
import StepLabel from "@mui/material/StepLabel";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { createPromiseClient } from "@bufbuild/connect";
import CreateDB from "views/Install/createDB";
import CreateDefaultUser from "views/Install/createUsers";

import { Lucle } from "gen/lucle_connect";

const steps = ["Create Database", "Create default user"];

export default function Install() {
  const [client, setClient] = useState<any>();
  const [error, setError] = useState<boolean>(false);
  const [activeStep, setActiveStep] = useState<number>(0);

  function InstallStep(step: number) {
    switch (step) {
      case 1:
        return <CreateDB InstallError={() => setError(true)}/>;
      case 2:
        return <CreateDefaultUser />;
      default:
        break;
    }
  }

  /*   useEffect(() => {
    //const newclient = connect("127.0.0.1", "3000");
    const transport = createGrpcWebTransport({
      baseUrl: `http://127.0.0.1:50051`,
    });
    const client = createPromiseClient(Lucle, transport);
    setClient(client);
  }, []); */

  const isStepFailed = (step: number) => step === activeStep;

  return (
    <Box sx={{ width: "100%" }}>
      <Stepper activeStep={activeStep}>
        {steps.map((label, index) => {
          const stepProps: {
            completed?: boolean;
            error?: boolean;
          } = {};
          if (isStepFailed(index)) {
            stepProps.error = error;
          }
          return (
            <Step key={label} >
              <StepLabel {...stepProps}>{label}</StepLabel>
            </Step>
          );
        })}
      </Stepper>
      {activeStep === steps.length ? (
        <>
          <Typography sx={{ mt: 2, mb: 1 }}>
            All steps completed - you&apos;re finished
          </Typography>
          <Box sx={{ display: "flex", flexDirection: "row", pt: 2 }}>
            <Box sx={{ flex: "1 1 auto" }} />
          </Box>
        </>
      ) : (
        <>
          {InstallStep(activeStep + 1)}
          <Box sx={{ display: "flex", flexDirection: "row", pt: 2 }}>
            <Button
              color="inherit"
              disabled={activeStep === 0}
              onClick={() =>
                setActiveStep((prevActiveStep) => prevActiveStep - 1)
              }
              sx={{ mr: 1 }}
            >
              Back
            </Button>
            <Box sx={{ flex: "1 1 auto" }} />
            <Button
              disabled={error}
              onClick={() =>
                setActiveStep((prevActiveStep) => prevActiveStep + 1)
              }
            >
              {activeStep === steps.length - 1 ? "Finish" : "Next"}
            </Button>
          </Box>
        </>
      )}
    </Box>
  );
}
