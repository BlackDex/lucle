import { useState, useEffect } from "react";
import libsodium from 'libsodium-wrappers-sumo';

// MUI
import Box from "@mui/material/Box";
import Stepper from "@mui/material/Stepper";
import Step from "@mui/material/Step";
import StepLabel from "@mui/material/StepLabel";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";

// RPC Connect
import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { createPromiseClient } from "@bufbuild/connect";


// Components
import CreateDB from "views/Install/createDB";
import CreateDefaultUser from "views/Install/createUsers";
import { create_user } from "utils/rpc";

import { Lucle } from "gen/lucle_connect";

const steps = ["Create Database", "Create default user"];

export default function Install() {
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [client, setClient] = useState<any>();
  const [error, setError] = useState<boolean>(false);
  const [activeStep, setActiveStep] = useState<number>(0);

  const InstallStep = (step: number) => {
    switch (step) {
      case 1:
        return <CreateDB InstallError={() => setError(true)} />;
      case 2:
        return (
          <CreateDefaultUser
            user={(user: string) => setUsername(user)}
            passwd={(pass: string) => setPassword(pass)}
          />
        );
      default:
        break;
    }
  };

  const hash_password = async (plain_password : string) => {
    await libsodium.ready;
    const sodium = libsodium;

    var hashed_password =
    sodium.crypto_pwhash_str(plain_password,
    sodium.crypto_pwhash_OPSLIMIT_INTERACTIVE,
    sodium.crypto_pwhash_MEMLIMIT_INTERACTIVE);
    
    return hashed_password;
  }

  useEffect(() => {
    //const newclient = connect("127.0.0.1", "3000");

    const transport = createGrpcWebTransport({
      baseUrl: `http://127.0.0.1:50051`,
    });
    const client = createPromiseClient(Lucle, transport);
    setClient(client);
  }, []);

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
            <Step key={label}>
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
              onClick={() => {
                setActiveStep((prevActiveStep) => prevActiveStep + 1);
                activeStep === steps.length - 1
                  ? hash_password(password).then((passwd: string) => create_user(client, username, passwd))
                  : null;
              }}
            >
              {activeStep === steps.length - 1 ? "Finish" : "Next"}
            </Button>
          </Box>
        </>
      )}
    </Box>
  );
}
