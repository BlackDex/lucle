import { useState, useContext, useEffect } from "react";
import { useNavigate } from "react-router-dom";

// MUI
import Box from "@mui/material/Box";
import Stepper from "@mui/material/Stepper";
import Step from "@mui/material/Step";
import StepLabel from "@mui/material/StepLabel";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";

// Components
import CreateDB from "views/Install/createDB";
import CreateDefaultUser from "views/Install/createUser";
import { createUser, createDB } from "utils/rpc";

// Context
import { LucleRPC } from "context";

const steps = ["Create Database", "Create default user"];

function InstallStep(
  step: number,
  handleDBtype: (DBType: number) => void,
  selectedDB: number,
  setUsername: (user: string) => void,
  setPassword: (pass: string) => void,
  setConfirmPassword: (confirmPass: string) => void,
  setPasswordStrengh: (strengh: number) => void,
  setEmail: (email: string) => void,
) {
  switch (step) {
    case 1:
      return <CreateDB setSelectedDB={handleDBtype} selectedDB={selectedDB} />;
    case 2:
      return (
        <CreateDefaultUser
          user={setUsername}
          password={setPassword}
          confirmPassword={setConfirmPassword}
          passwordStrengh={setPasswordStrengh}
          email={setEmail}
        />
      );
    default:
      break;
  }
}

export default function Install() {
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [confirmPassword, setConfirmPassword] = useState<string>("");
  const [passwordStrengh, setPasswordStrengh] = useState(0);
  const [email, setEmail] = useState<string>("");
  const [error, setError] = useState<string>("");
  const [activeStep, setActiveStep] = useState<number>(0);
  const [selectedDB, setSelectedDB] = useState<number>(2);
  const navigate = useNavigate();
  const client = useContext(LucleRPC);

  const handleDBtype = (DBType = 2) => {
    setSelectedDB(DBType);
  };

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
              <StepLabel
                completed={stepProps.completed}
                error={stepProps.error}
              >
                {label}
              </StepLabel>
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
          <Box sx={{ display: "flex", flexDirection: "roqw", pt: 2 }}>
            <Box sx={{ flex: "1 1 auto" }} />
          </Box>
        </>
      ) : (
        <>
          {InstallStep(
            activeStep + 1,
            handleDBtype,
            selectedDB,
            setUsername,
            setPassword,
            setConfirmPassword,
            setPasswordStrengh,
            setEmail,
          )}
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
              disabled={activeStep == 1 && passwordStrengh < 3}
              onClick={() => {
                switch (activeStep) {
                  case 0:
                    createDB(client, selectedDB)
                      .then(() =>
                        setActiveStep((prevActiveStep) => prevActiveStep + 1),
                      )
                      .catch((err) => console.log(err)); //setError(err.rawMessage));
                    break;
                  case steps.length - 1:
                    if (password == confirmPassword && password) {
                      createUser(
                        client,
                        username,
                        password,
                        email,
                        "admin",
                      ).catch((err) => setError(err.rawMessage));
                      setTimeout(() => navigate("/"), 10000);
                      setActiveStep((prevActiveStep) => prevActiveStep + 1);
                    } else {
                      setError("Password doesn't match");
                    }
                    break;
                  default:
                    break;
                }
              }}
            >
              {activeStep === steps.length - 1 ? "Finish" : "Next"}
            </Button>
          </Box>
          {error}
        </>
      )}
    </Box>
  );
}
