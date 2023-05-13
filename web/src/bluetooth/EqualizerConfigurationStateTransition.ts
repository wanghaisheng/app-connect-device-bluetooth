import { isEqual } from "lodash-es";
import { SetEqualizerPacket } from "../../wasm/pkg/openscq30_web_wasm";
import { SoundcoreDeviceConnection } from "./SoundcoreDeviceConnection";
import { SoundcoreDeviceState } from "./SoundcoreDeviceState";

export async function transitionEqualizerState(
  connection: SoundcoreDeviceConnection,
  previousState: SoundcoreDeviceState,
  newState: SoundcoreDeviceState,
) {
  if (
    previousState.equalizerConfiguration.presetProfile ==
      newState.equalizerConfiguration.presetProfile &&
    isEqual(
      previousState.equalizerConfiguration.volumeAdjustments.adjustments,
      newState.equalizerConfiguration.volumeAdjustments.adjustments,
    )
  ) {
    return;
  }

  await connection.write(
    new SetEqualizerPacket(newState.equalizerConfiguration).bytes(),
  );
}
