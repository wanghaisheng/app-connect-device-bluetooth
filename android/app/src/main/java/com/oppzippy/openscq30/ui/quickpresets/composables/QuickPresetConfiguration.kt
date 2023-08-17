package com.oppzippy.openscq30.ui.quickpresets.composables

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Divider
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.PresetEqualizerProfile
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import com.oppzippy.openscq30.lib.extensions.resources.toStringResource
import com.oppzippy.openscq30.ui.equalizer.composables.CustomProfileSelection
import com.oppzippy.openscq30.ui.quickpresets.models.QuickPresetEqualizerConfiguration
import com.oppzippy.openscq30.ui.soundmode.AmbientSoundModeSelection
import com.oppzippy.openscq30.ui.soundmode.CustomNoiseCancelingSelection
import com.oppzippy.openscq30.ui.soundmode.NoiseCancelingModeSelection
import com.oppzippy.openscq30.ui.soundmode.TransparencyModeSelection
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.CheckboxWithLabel
import com.oppzippy.openscq30.ui.utils.Dropdown
import com.oppzippy.openscq30.ui.utils.DropdownOption

@Composable
fun QuickPresetConfiguration(
    name: String?,
    defaultName: String,
    ambientSoundMode: AmbientSoundMode?,
    noiseCancelingMode: NoiseCancelingMode?,
    transparencyMode: TransparencyMode?,
    customNoiseCanceling: CustomNoiseCanceling?,
    equalizerConfiguration: QuickPresetEqualizerConfiguration?,
    customEqualizerProfiles: List<CustomProfile>,
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode?) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode?) -> Unit = {},
    onTransparencyModeChange: (transparencyMode: TransparencyMode?) -> Unit = {},
    onCustomNoiseCancelingChange: (customNoiseCanceling: CustomNoiseCanceling?) -> Unit = {},
    onEqualizerChange: (config: QuickPresetEqualizerConfiguration?) -> Unit = {},
    onNameChange: (name: String?) -> Unit = {},
) {
    Column(Modifier.verticalScroll(rememberScrollState())) {
        TextField(
            value = name ?: "",
            onValueChange = {
                onNameChange(it.ifBlank { null })
            },
            placeholder = { Text(defaultName) },
            label = { Text(stringResource(R.string.name)) },
            singleLine = true,
            modifier = Modifier
                .fillMaxWidth()
                .testTag("quickPresetNameInput"),
        )
        CheckboxWithLabel(
            text = stringResource(R.string.ambient_sound_mode),
            isChecked = ambientSoundMode != null,
            onCheckedChange = {
                onAmbientSoundModeChange(if (it) AmbientSoundMode.Normal else null)
            },
        )
        if (ambientSoundMode != null) {
            AmbientSoundModeSelection(
                ambientSoundMode = ambientSoundMode,
                onAmbientSoundModeChange = onAmbientSoundModeChange,
                hasNoiseCanceling = true,
            )
        }
        Divider()

        CheckboxWithLabel(
            text = stringResource(R.string.transparency_mode),
            isChecked = transparencyMode != null,
            onCheckedChange = {
                onTransparencyModeChange(if (it) TransparencyMode.VocalMode else null)
            },
        )
        if (transparencyMode != null) {
            TransparencyModeSelection(
                transparencyMode = transparencyMode,
                onTransparencyModeChange = onTransparencyModeChange,
            )
        }
        Divider()

        CheckboxWithLabel(
            text = stringResource(R.string.noise_canceling_mode),
            isChecked = noiseCancelingMode != null,
            onCheckedChange = {
                onNoiseCancelingModeChange(if (it) NoiseCancelingMode.Transport else null)
            },
        )
        if (noiseCancelingMode != null) {
            NoiseCancelingModeSelection(
                noiseCancelingMode = noiseCancelingMode,
                onNoiseCancelingModeChange = onNoiseCancelingModeChange,
                hasCustomNoiseCanceling = true,
            )
        }
        Divider()

        CheckboxWithLabel(
            text = stringResource(R.string.custom_noise_canceling),
            isChecked = customNoiseCanceling != null,
            onCheckedChange = {
                onCustomNoiseCancelingChange(if (it) CustomNoiseCanceling(0) else null)
            },
        )
        if (customNoiseCanceling != null) {
            CustomNoiseCancelingSelection(
                customNoiseCanceling = customNoiseCanceling,
                onCustomNoiseCancelingChange = onCustomNoiseCancelingChange,
            )
        }
        Divider()

        var isEqualizerChecked by remember { mutableStateOf(equalizerConfiguration != null) }
        // We want isEqualizerChecked to reset when moving between tabs. defaultName is a key unique
        // to each tab.
        LaunchedEffect(defaultName) {
            isEqualizerChecked = equalizerConfiguration != null
        }
        CheckboxWithLabel(
            text = stringResource(R.string.equalizer),
            isChecked = equalizerConfiguration != null || isEqualizerChecked,
            onCheckedChange = {
                isEqualizerChecked = it
                if (!isEqualizerChecked) {
                    onEqualizerChange(null)
                }
            },
        )
        if (equalizerConfiguration != null || isEqualizerChecked) {
            Dropdown(
                value = (equalizerConfiguration as? QuickPresetEqualizerConfiguration.PresetProfile)?.profile,
                options = PresetEqualizerProfile.values().map {
                    val presetName = stringResource(it.toStringResource())
                    DropdownOption(
                        value = it,
                        label = { Text(presetName) },
                        name = presetName,
                    )
                },
                label = stringResource(R.string.preset_profile),
                onItemSelected = {
                    onEqualizerChange(
                        QuickPresetEqualizerConfiguration.PresetProfile(
                            it,
                        ),
                    )
                },
                modifier = Modifier.testTag("quickPresetPresetEqualizerProfile"),
            )

            CustomProfileSelection(
                selectedProfile = if (equalizerConfiguration is QuickPresetEqualizerConfiguration.CustomProfile) {
                    customEqualizerProfiles.find { it.name == equalizerConfiguration.name }
                } else {
                    null
                },
                profiles = customEqualizerProfiles,
                onProfileSelected = {
                    onEqualizerChange(QuickPresetEqualizerConfiguration.CustomProfile(it.name))
                },
                modifier = Modifier.testTag("quickPresetCustomEqualizerProfile"),
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewQuickPresetConfiguration() {
    OpenSCQ30Theme {
        QuickPresetConfiguration(
            name = null,
            defaultName = "Quick Preset 1",
            ambientSoundMode = AmbientSoundMode.NoiseCanceling,
            noiseCancelingMode = NoiseCancelingMode.Transport,
            transparencyMode = TransparencyMode.VocalMode,
            customNoiseCanceling = CustomNoiseCanceling(5),
            customEqualizerProfiles = emptyList(),
            equalizerConfiguration = QuickPresetEqualizerConfiguration.PresetProfile(
                PresetEqualizerProfile.SoundcoreSignature,
            ),
        )
    }
}
