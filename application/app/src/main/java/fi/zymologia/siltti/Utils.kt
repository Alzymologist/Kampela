package fi.zymologia.siltti

import android.app.Activity
import android.content.Context
import android.content.ContextWrapper
import androidx.core.app.ActivityCompat
import fi.zymologia.siltti.screens.allPermissionsGranted

fun Context.getActivity(): Activity = when (this) {
    is Activity -> this
    is ContextWrapper -> baseContext.getActivity()
    else -> TODO()
}

fun handleCameraPermissions(activity: Activity) {
    if (!allPermissionsGranted(activity)) {
        ActivityCompat.requestPermissions(
            activity,
            REQUIRED_PERMISSIONS,
            REQUEST_CODE_PERMISSIONS
        )
    }
}