package fi.zymologia.siltti

import android.Manifest
import android.app.Activity
import android.content.pm.PackageManager
import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.Preview
import androidx.camera.core.impl.utils.ContextUtil.getApplicationContext
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
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
import com.google.mlkit.vision.barcode.BarcodeScannerOptions
import com.google.mlkit.vision.barcode.BarcodeScanning
import com.google.mlkit.vision.barcode.common.Barcode

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
fun ScanScreen(activity: Activity) {
    // var collection = remember { Collection() }
    // val frames: MutableState<Frames?> = remember { mutableStateOf(null)}
    val lifecycleOwner = LocalLifecycleOwner.current
    val context = LocalContext.current
    val cameraProviderFuture =
        remember { ProcessCameraProvider.getInstance(context) }

	/*
	if (frames.value != null) {
		KeepScreenOn()
	}
	*/

    Column(
        Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
    ) {
        Box(
            Modifier.padding(8.dp)
        ) {
            // TODO: use all the cores needed to make this smooth
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
                    handleCameraPermissions(activity)

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
									/* TODO
									processFrame(
										barcodeScanner,
										imageProxy,
										collection::processFrame
									) {
										try {
											frames.value = collection.frames()
										} catch (e: io.parity.signer.uniffi.ErrorQr) {
											Toast.makeText(
												context,
												"QR scanner error: " + e.message,
												Toast.LENGTH_LONG
											).show()
										}
									}*/
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

		/* TODO
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
					} catch (e: io.parity.signer.uniffi.ErrorQr) {
						Toast
							.makeText(
								context,
								"QR scanner reset error: " + e.message,
								Toast.LENGTH_LONG
							).show()
					}
				}
			)
		}*/
    }
}

/* TODO
/**
 * Barcode detecting function.
 * This uses experimental features
 */
@OptIn(ExperimentalUnsignedTypes::class)
@SuppressLint("UnsafeOptInUsageError")
fun processFrame(
	barcodeScanner: BarcodeScanner,
	imageProxy: ImageProxy,
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
					} catch (e: java.lang.Exception) { // TODO: specify error correctly
						Toast.makeText(
							MainActivity().applicationContext,
							"QR parser error: " + e.message,
							Toast.LENGTH_SHORT
						).show()
						null
					}
				}?.payload?.let { payload ->
					// This is pressed only once, that's checked in rust backend
					// by sending complete payload only once
					TODO()
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
*/
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
