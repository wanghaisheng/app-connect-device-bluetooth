package com.oppzippy.openscq30.ui.devicesettings

import androidx.compose.runtime.*
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.R

@Composable
fun PresetProfileSelection(
    value: EqualizerProfile, onProfileSelected: (profile: EqualizerProfile) -> Unit
) {
    Dropdown(
        value = value,
        values = EqualizerProfile.values().asList(),
        label = stringResource(id = R.string.profile),
        onItemSelected = onProfileSelected,
    )
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        var profile by remember { mutableStateOf(EqualizerProfile.Classical) }
        PresetProfileSelection(value = profile, onProfileSelected = {
            profile = it
        })
    }
}
