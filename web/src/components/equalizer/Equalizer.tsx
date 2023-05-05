import Grid2 from "@mui/material/Unstable_Grid2/Grid2";
import { VolumeSlider } from "./VolumeSlider";

type Props = {
  disabled?: boolean;
  values: number[];
  onValueChange: (index: number, newValue: number) => void;
};

export function Equalizer(props: Props) {
  return (
    <Grid2
      container
      rowSpacing={2}
      columnSpacing={2}
      sx={{
        // 3 columns per row, so to have no bottom border on the last row, exclude the last three children
        "& > :not(:nth-last-of-type(1), :nth-last-of-type(2), :nth-last-of-type(3))":
          {
            borderBottom: "1px solid",
            borderColor: "divider",
          },
      }}
    >
      {props.values.map((value, index) => {
        return (
          <VolumeSlider
            disabled={props.disabled}
            key={index}
            // Starts at 100hz
            hz={100 * Math.pow(2, index)}
            value={value}
            onValueChange={(newValue) => props.onValueChange(index, newValue)}
          />
        );
      })}
    </Grid2>
  );
}
