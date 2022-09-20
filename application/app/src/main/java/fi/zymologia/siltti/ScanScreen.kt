package fi.zymologia.siltti

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.content.Context
import android.content.ContextWrapper
import android.content.pm.PackageManager
import android.util.Log
import android.widget.Toast
import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.ImageProxy
import androidx.camera.core.Preview
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.platform.LocalView
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import com.google.mlkit.vision.barcode.BarcodeScanner
import com.google.mlkit.vision.barcode.BarcodeScannerOptions
import com.google.mlkit.vision.barcode.BarcodeScanning
import com.google.mlkit.vision.barcode.common.Barcode
import com.google.mlkit.vision.common.InputImage
import fi.zymologia.siltti.uniffi.Collection
import fi.zymologia.siltti.uniffi.Frames
import fi.zymologia.siltti.uniffi.Payload

val REQUIRED_PERMISSIONS = arrayOf(Manifest.permission.CAMERA)
val REQUEST_CODE_PERMISSIONS = 10

@Composable
fun KeepScreenOn() {
    val currentView = LocalView.current
    DisposableEffect(Unit) {
        currentView.keepScreenOn = true
        onDispose {
            currentView.keepScreenOn = false
        }
    }
}

/**
 * Main scanner screen. One of navigation roots.
 */
@Composable
fun ScanScreen() {
    var collection = remember { Collection() }
    val frames: MutableState<Frames?> = remember { mutableStateOf(null) }
    val lifecycleOwner = LocalLifecycleOwner.current
    val context = LocalContext.current
    val cameraProviderFuture =
        remember { ProcessCameraProvider.getInstance(context) }
    var appState by remember { mutableStateOf(Mode.Scan) }

    if (frames.value != null) {
        KeepScreenOn()
    }

    Column(
        Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
    ) {
        Box(
            Modifier.padding(8.dp)
        ) {
            // TODO: use all the cores needed to make this smooth
            when (appState) {
                Mode.Address -> {
                    Surface(
                        Modifier
                            .padding(bottom = 24.dp)
                            .border(
                                BorderStroke(1.dp, MaterialTheme.colors.primary),
                                RoundedCornerShape(8.dp)
                            )
                            .clip(RoundedCornerShape(8.dp))
                    ) {
                        Text("Type derivation")
                        Button(
                            onClick = { appState = Mode.TX }
                        ) {
                            Text("crach the app")
                        }
                    }
                }
                Mode.Scan -> {
                    AndroidView(
                        factory = { context ->
                            val executor = ContextCompat.getMainExecutor(context)
                            val previewView = PreviewView(context)
                            // mlkit docs: The default option is not recommended because it tries
                            // to scan all barcode formats, which is slow.
                            //
                            // In fact, it is not slow at all, no measurable difference was
                            // observed, but this avoids extra barcodes being accidentally scanned
                            // during multiframes.
                            val options = BarcodeScannerOptions.Builder()
                                .setBarcodeFormats(Barcode.FORMAT_QR_CODE).build()

                            val barcodeScanner = BarcodeScanning.getClient(options)

                            // This might be done more elegantly, if needed.
                            // But it's pretty obvious that the app needs camera
                            // and why; also it just works so far and code is tiny.
                            handleCameraPermissions(context.getActivity())

                            cameraProviderFuture.addListener({
                                val cameraProvider = cameraProviderFuture.get()

                                val preview = Preview.Builder().build().also {
                                    it.setSurfaceProvider(previewView.surfaceProvider)
                                }

                                val cameraSelector = CameraSelector.Builder()
                                    .requireLensFacing(CameraSelector.LENS_FACING_BACK)
                                    .build()

                                val imageAnalysis = ImageAnalysis.Builder()
                                    .setBackpressureStrategy(ImageAnalysis.STRATEGY_KEEP_ONLY_LATEST)
                                    .build()
                                    .apply {
                                        setAnalyzer(executor) { imageProxy ->
                                            processFrame(
                                                context,
                                                barcodeScanner,
                                                imageProxy,
                                                { appState = Mode.TX },
                                                collection::processFrame
                                            ) {
                                                try {
                                                    frames.value = collection.frames()
                                                } catch (e: fi.zymologia.siltti.uniffi.ErrorQr) {
                                                    Toast.makeText(
                                                        context,
                                                        "QR scanner error: " + e.message,
                                                        Toast.LENGTH_LONG
                                                    ).show()
                                                }
                                            }
                                        }
                                    }

                                cameraProvider.unbindAll()
                                cameraProvider.bindToLifecycle(
                                    lifecycleOwner,
                                    cameraSelector,
                                    imageAnalysis,
                                    preview
                                )
                            }, executor)
                            previewView
                        },
                        Modifier
                            .padding(bottom = 24.dp)
                            .border(
                                BorderStroke(1.dp, MaterialTheme.colors.primary),
                                RoundedCornerShape(8.dp)
                            )
                            .clip(RoundedCornerShape(8.dp))
                    )
                }
                Mode.TX -> {
                    Button(
                        onClick = {
                            collection.clean()
                            appState = Mode.Scan
                        }
                    ) {
                        Text("Stop transmission")
                    }
                }
            }
        }

        Column(
            verticalArrangement = Arrangement.Bottom,
            modifier = Modifier.fillMaxSize()
        ) {
            ScanProgressBar(
                frames = frames,
                resetScan = {
                    try {
                        collection.clean()
                        frames.value = collection.frames()
                    } catch (e: fi.zymologia.siltti.uniffi.ErrorQr) {
                        Toast
                            .makeText(
                                context,
                                "QR scanner reset error: " + e.message,
                                Toast.LENGTH_LONG
                            ).show()
                    }
                }
            )
            Button(
                onClick = { appState = Mode.Address }
            ) {
                Text("Create address")
            }
        }
    }
}

/**
 * Barcode detecting function.
 * This uses experimental features
 */
@OptIn(ExperimentalUnsignedTypes::class)
@SuppressLint("UnsafeOptInUsageError")
fun processFrame(
    context: Context,
    barcodeScanner: BarcodeScanner,
    imageProxy: ImageProxy,
    startTransmission: () -> Unit,
    submitFrame: (List<UByte>) -> Payload,
    refreshFrames: () -> Unit
) {
    if (imageProxy.image == null) return
    val inputImage = InputImage.fromMediaImage(
        imageProxy.image!!,
        imageProxy.imageInfo.rotationDegrees
    )

    barcodeScanner.process(inputImage)
        .addOnSuccessListener { barcodes ->
            barcodes.forEach {
                it?.rawBytes?.toUByteArray()?.toList()?.let { payload ->
                    try {
                        submitFrame(payload)
                    } catch (e: fi.zymologia.siltti.uniffi.ErrorQr) {
                        Toast.makeText(
                            context,
                            "QR parser error: " + e.message,
                            Toast.LENGTH_SHORT
                        ).show()
                        null
                    }
                }?.payload?.let { payload ->
                    // This is pressed only once, that's checked in rust backend
                    // by sending complete payload only once
                    Toast.makeText(
                        context,
                        "success!",
                        Toast.LENGTH_SHORT
                    ).show()
                    startTransmission()
                }
                refreshFrames()
            }
        }
        .addOnFailureListener {
            Log.e("Scan failed", it.message.toString())
        }
        .addOnCompleteListener {
            imageProxy.close()
        }
}

fun allPermissionsGranted(activity: Activity) = REQUIRED_PERMISSIONS.all {
    ContextCompat.checkSelfPermission(
        activity,
        it
    ) == PackageManager.PERMISSION_GRANTED
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

fun Context.getActivity(): Activity = when (this) {
    is Activity -> this
    is ContextWrapper -> baseContext.getActivity()
    else -> TODO()
}

enum class Mode {
    Scan,
    Address,
    TX,
}
