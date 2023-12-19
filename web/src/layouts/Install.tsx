import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

// MUI
import Box from "@mui/material/Box";
import Stepper from "@mui/material/Stepper";
import Step from "@mui/material/Step";
import StepLabel from "@mui/material/StepLabel";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";

// RPC Connect
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { createPromiseClient } from "@connectrpc/connect";
import { Lucle } from "gen/lucle_connect";

// Components
import CreateDB from "views/Install/createDB";
import CreateDefaultUser from "views/Install/createUsers";
import { createUser, dbConnection } from "utils/rpc";

const steps = ["Create Database", "Create default user"];

export default function Install() {
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [email, setEmail] = useState<string>("");
  const [client, setClient] = useState<any>();
  const [error, setError] = useState<boolean>(false);
  const [activeStep, setActiveStep] = useState<number>(0);
  const [selectedDB, setSelectedDB] = useState<number>(2);
  const navigate = useNavigate();

  const handleDBtype = (DBType = 2) => {
    setSelectedDB(DBType);
  };

  const InstallStep = (step: number) => {
    switch (step) {
      case 1:
        return (
          <CreateDB setSelectedDB={handleDBtype} selectedDB={selectedDB} />
        );
      case 2:
        return (
          <CreateDefaultUser
            user={(user: string) => setUsername(user)}
            passwd={(pass: string) => setPassword(pass)}
            email={(mail: string) => setEmail(mail)}
          />
        );
      default:
        break;
    }
  };

  useEffect(() => {
    const transport = createGrpcWebTransport({
      baseUrl: `http://127.0.0.1:50051`,
    });
    const newclient = createPromiseClient(Lucle, transport);
    setClient(newclient);
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
            <p>All steps completed - you&apos;re finished</p>
            <p>You will be redirect to home page into 10 secondes</p>
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
                activeStep === 0
                  ? dbConnection(client, selectedDB).catch((err) =>
                      setError(err),
                    )
                  : null;
                activeStep === steps.length - 1
                  ? (createUser(client, username, password, email),
                    setTimeout(() => navigate("/"), 10000))
                  : null;
              }}
            >
              {activeStep === steps.length - 1 ? "Finish" : "Next"}
            </Button>
            {error}
          </Box>
        </>
      )}
    </Box>
  );
}
