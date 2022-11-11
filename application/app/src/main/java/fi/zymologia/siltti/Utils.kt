package fi.zymologia.siltti

import android.app.Activity
import android.content.Context
import android.content.ContextWrapper
import androidx.core.app.ActivityCompat
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
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

class PackagesSent : ViewModel() {
    private val _count = MutableLiveData<Int?>(null)
    val count: LiveData<Int?> = _count

    fun reset() {
        _count.value = 0
    }

    fun inc() {
        _count.value = (_count.value ?: 0) + 1
    }

    fun disable() {
        _count.value = null
    }
}