import * as React from "react";
import { useState } from "react";
import Switch from "@mui/material/Switch";
import FormControlLabel from "@mui/material/FormControlLabel";


export default function ThemeSwitcherComponent(props) {
  const expandedProps = {
    ...props,
    useOs: props.useOs || false,
    useDark: props.useDark || false,
    darkPrompt: props.darkPrompt || "Use dark mode",
    osPrompt: props.osPrompt || "Use OS preference",
    tooltipText: props.tooltipText || "OS preference: "
  };

  const [state, setState] = useState(expandedProps);


  const handleSwitch = (_e, checked) => {
    setState({ ...state, useDark: checked });
    state.themeChanger(checked);
    console.log(state);
  };

  return (
    <>
      <FormControlLabel
        labelPlacement="end"
        label={state.darkPrompt}
        control={
          <Switch
            checked={state.useDark}
            onChange={handleSwitch}
          />
        }
      />
    </>
  );
}
