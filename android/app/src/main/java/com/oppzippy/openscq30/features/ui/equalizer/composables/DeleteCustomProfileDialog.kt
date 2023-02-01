package com.oppzippy.openscq30.features.ui.equalizer.composables

import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.oppzippy.openscq30.R

@Composable
fun DeleteCustomProfileDialog(
    isOpen: Boolean,
    profileName: String,
    onDismiss: () -> Unit,
    onDelete: () -> Unit,
) {
    if (isOpen) {
        AlertDialog(
            onDismissRequest = onDismiss,
            title = {
                Text(stringResource(R.string.delete_custom_profile))
            },
            text = {
                Text(stringResource(R.string.custom_profile_delete_confirm, profileName))
            },
            confirmButton = {
                Button(onClick = {
                    onDelete()
                    onDismiss()
                }) {
                    Text(stringResource(R.string.delete))
                }
            },
            dismissButton = {
                Button(onClick = onDismiss) {
                    Text(stringResource(R.string.cancel))
                }
            },
        )
    }
}