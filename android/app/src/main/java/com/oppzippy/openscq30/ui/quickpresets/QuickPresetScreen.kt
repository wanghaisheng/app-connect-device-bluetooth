package com.oppzippy.openscq30.ui.quickpresets

import android.Manifest
import android.content.Intent
import android.os.Build
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.hilt.navigation.compose.hiltViewModel
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.features.soundcoredevice.service.SoundcoreDeviceNotification
import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling
import com.oppzippy.openscq30.lib.bindings.DeviceFeatureFlags
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import com.oppzippy.openscq30.ui.quickpresets.composables.QuickPresetConfiguration
import com.oppzippy.openscq30.ui.quickpresets.composables.QuickPresetSelection
import com.oppzippy.openscq30.ui.quickpresets.models.QuickPresetEqualizerConfiguration
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme
import com.oppzippy.openscq30.ui.utils.Loading
import com.oppzippy.openscq30.ui.utils.PermissionCheck
import java.util.UUID

@Composable
fun QuickPresetScreen(
    featureFlags: DeviceFeatureFlags,
    deviceBleServiceUuid: UUID,
    viewModel: QuickPresetViewModel = hiltViewModel(),
) {
    DisposableEffect(deviceBleServiceUuid) {
        viewModel.selectQuickPreset(deviceBleServiceUuid, 0)
        onDispose {
            viewModel.clearSelection()
        }
    }

    val preset = viewModel.quickPreset.collectAsState().value
    val customEqualizerProfiles by viewModel.customEqualizerProfiles.collectAsState()
    val context = LocalContext.current

    // We can't nest the content inside the permission check since we need to ensure the permission
    // check doesn't run on versions of android that don't require permission for foreground service
    // notifications.
    val isTiramisuOrNewer = Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU
    var permissionCheckPassed by remember { mutableStateOf(!isTiramisuOrNewer) }
    // Redundant check to fix lint error since permissionCheckPassed will always be true if
    // isTiramisuOrNewer is false.
    if (!permissionCheckPassed && isTiramisuOrNewer) {
        PermissionCheck(
            permission = Manifest.permission.POST_NOTIFICATIONS,
            prompt = stringResource(R.string.notification_permission_is_required),
        ) {
            permissionCheckPassed = true
            // Since we may have not had notification permission before this point, we need to
            // resend the notification to ensure it is visible.
            Intent().apply {
                action = SoundcoreDeviceNotification.ACTION_SEND_NOTIFICATION
                context.sendBroadcast(this)
            }
        }
    }

    if (permissionCheckPassed) {
        if (preset != null) {
            QuickPresetScreen(
                featureFlags = featureFlags,
                preset = preset,
                customEqualizerProfiles = customEqualizerProfiles,
                onSelectedIndexChange = { viewModel.selectQuickPreset(deviceBleServiceUuid, it) },
                onAmbientSoundModeChange = {
                    viewModel.upsertQuickPreset(
                        preset.copy(ambientSoundMode = it),
                    )
                },
                onNoiseCancelingModeChange = {
                    viewModel.upsertQuickPreset(
                        preset.copy(noiseCancelingMode = it),
                    )
                },
                onTransparencyModeChange = {
                    viewModel.upsertQuickPreset(preset.copy(transparencyMode = it))
                },
                onCustomNoiseCancelingChange = {
                    viewModel.upsertQuickPreset(preset.copy(customNoiseCanceling = it))
                },
                onEqualizerChange = {
                    val presetEqualizerProfile =
                        if (it is QuickPresetEqualizerConfiguration.PresetProfile) {
                            it.profile
                        } else {
                            null
                        }
                    val customEqualizerProfile =
                        if (it is QuickPresetEqualizerConfiguration.CustomProfile) {
                            it.name
                        } else {
                            null
                        }
                    viewModel.upsertQuickPreset(
                        preset.copy(
                            presetEqualizerProfile = presetEqualizerProfile,
                            customEqualizerProfileName = customEqualizerProfile,
                        ),
                    )
                },
                onNameChange = { viewModel.upsertQuickPreset(preset.copy(name = it)) },
            )
        } else {
            Loading()
        }
    }
}

@Composable
private fun QuickPresetScreen(
    featureFlags: DeviceFeatureFlags,
    preset: QuickPreset,
    customEqualizerProfiles: List<CustomProfile>,
    onSelectedIndexChange: (index: Int) -> Unit = {},
    onAmbientSoundModeChange: (ambientSoundMode: AmbientSoundMode?) -> Unit = {},
    onNoiseCancelingModeChange: (noiseCancelingMode: NoiseCancelingMode?) -> Unit = {},
    onTransparencyModeChange: (transparencyMode: TransparencyMode?) -> Unit = {},
    onCustomNoiseCancelingChange: (customNoiseCanceling: CustomNoiseCanceling?) -> Unit = {},
    onEqualizerChange: (config: QuickPresetEqualizerConfiguration?) -> Unit = {},
    onNameChange: (name: String?) -> Unit = {},
) {
    Column {
        QuickPresetSelection(
            selectedIndex = preset.index,
            onSelectedIndexChange = onSelectedIndexChange,
        )
        QuickPresetConfiguration(
            featureFlags = featureFlags,
            name = preset.name,
            defaultName = stringResource(R.string.quick_preset_number, preset.index + 1),
            ambientSoundMode = preset.ambientSoundMode,
            noiseCancelingMode = preset.noiseCancelingMode,
            transparencyMode = preset.transparencyMode,
            customNoiseCanceling = preset.customNoiseCanceling,
            equalizerConfiguration = if (preset.presetEqualizerProfile != null) {
                QuickPresetEqualizerConfiguration.PresetProfile(preset.presetEqualizerProfile)
            } else if (preset.customEqualizerProfileName != null) {
                QuickPresetEqualizerConfiguration.CustomProfile(preset.customEqualizerProfileName)
            } else {
                null
            },
            customEqualizerProfiles = customEqualizerProfiles,
            onAmbientSoundModeChange = onAmbientSoundModeChange,
            onNoiseCancelingModeChange = onNoiseCancelingModeChange,
            onTransparencyModeChange = onTransparencyModeChange,
            onCustomNoiseCancelingChange = onCustomNoiseCancelingChange,
            onEqualizerChange = onEqualizerChange,
            onNameChange = onNameChange,
        )
    }
}

@Preview(showBackground = true)
@Composable
fun PreviewQuickPresetScreenWithAllOptionsChecked() {
    OpenSCQ30Theme {
        QuickPresetScreen(
            featureFlags = DeviceFeatureFlags.all(),
            preset = QuickPreset(
                deviceBleServiceUuid = UUID(0, 0),
                index = 0,
                ambientSoundMode = AmbientSoundMode.Normal,
                noiseCancelingMode = NoiseCancelingMode.Transport,
                customEqualizerProfileName = "Test EQ Profile",
            ),
            customEqualizerProfiles = emptyList(),
        )
    }
}

@Preview(showBackground = true)
@Composable
fun PreviewQuickPresetScreenWithNoOptionsChecked() {
    OpenSCQ30Theme {
        QuickPresetScreen(
            featureFlags = DeviceFeatureFlags.all(),
            preset = QuickPreset(UUID(0, 0), 0),
            customEqualizerProfiles = emptyList(),
        )
    }
}
