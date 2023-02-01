package com.oppzippy.openscq30.features.ui.soundmode

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.selectable
import androidx.compose.foundation.selection.selectableGroup
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.RadioButton
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun GeneralSettings(
    modifier: Modifier = Modifier,
    viewModel: GeneralSettingsViewModel = hiltViewModel(),
) {
    val ambientSoundMode = viewModel.ambientSoundMode?.collectAsState()
    val noiseCancelingMode = viewModel.noiseCancelingMode?.collectAsState()

    if (ambientSoundMode != null && noiseCancelingMode != null) {
        Column(modifier = modifier) {
            GroupHeader(stringResource(R.string.ambient_sound_mode))
            LabeledRadioButtonGroup(
                selectedValue = ambientSoundMode.value,
                values = linkedMapOf(
                    Pair(AmbientSoundMode.Normal, stringResource(R.string.normal)),
                    Pair(AmbientSoundMode.Transparency, stringResource(R.string.transparency)),
                    Pair(AmbientSoundMode.NoiseCanceling, stringResource(R.string.noise_canceling)),
                ),
                onValueChange = { viewModel.setAmbientSoundMode(it) },
            )
            Spacer(modifier = Modifier.padding(vertical = 8.dp))
            GroupHeader(stringResource(R.string.noise_canceling_mode))
            LabeledRadioButtonGroup(
                selectedValue = noiseCancelingMode.value,
                values = linkedMapOf(
                    Pair(NoiseCancelingMode.Transport, stringResource(R.string.transport)),
                    Pair(NoiseCancelingMode.Indoor, stringResource(R.string.indoor)),
                    Pair(NoiseCancelingMode.Outdoor, stringResource(R.string.outdoor)),
                ),
                onValueChange = { viewModel.setNoiseCancelingMode(it) },
            )
        }
    }
}

@Composable
private fun GroupHeader(text: String) {
    Text(
        text = text,
        style = MaterialTheme.typography.titleMedium,
        modifier = Modifier.padding(horizontal = 2.dp, vertical = 2.dp),
    )
}

@Composable
private fun <T> LabeledRadioButtonGroup(
    selectedValue: T, values: LinkedHashMap<T, String>, onValueChange: (value: T) -> Unit
) {
    Column(Modifier.selectableGroup()) {
        values.forEach { (value, text) ->
            LabeledRadioButton(text = text, selected = selectedValue == value, onClick = {
                onValueChange(value)
            })
        }
    }
}

@Composable
private fun LabeledRadioButton(text: String, selected: Boolean, onClick: () -> Unit) {
    Row(
        Modifier
            .fillMaxWidth()
            .selectable(selected = selected, onClick = onClick, role = Role.RadioButton)
            .padding(horizontal = 2.dp, vertical = 2.dp)
    ) {
        RadioButton(selected = selected, onClick = null)
        Text(
            text = text,
            modifier = Modifier.padding(start = 8.dp),
        )
    }
}

@Preview(showBackground = true)
@Composable
private fun DefaultPreview() {
    OpenSCQ30Theme {
        GeneralSettings()
    }
}