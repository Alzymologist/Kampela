package fi.zymologia.siltti

import android.Manifest
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.unit.dp
import fi.zymologia.siltti.screens.KeepScreenOn
import fi.zymologia.siltti.screens.ScanScreen
import fi.zymologia.siltti.screens.TXScreen
import fi.zymologia.siltti.uniffi.*
import fi.zymologia.siltti.uniffi.Collection
import java.security.KeyPairGenerator
import java.security.Signature

val REQUIRED_PERMISSIONS = arrayOf(Manifest.permission.CAMERA)
val REQUEST_CODE_PERMISSIONS = 10

/**
 * Main scanner screen. One of navigation roots.
 */
@Composable
fun ScreenScaffold(
    dbName: String,
    transmitCallback: (List<ByteArray>) -> Unit
) {
    val collection = remember { Collection() }
    val frames: MutableState<Frames?> = remember { mutableStateOf(null) }
    val lifecycleOwner = LocalLifecycleOwner.current
    val context = LocalContext.current
    val cameraProviderFuture =
        remember { ProcessCameraProvider.getInstance(context) }
    var appState by remember { mutableStateOf(Mode.Scan) }

    val setAppState = { mode: Mode -> appState = mode }

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
                    fi.zymologia.siltti.screens.NewAddress(setAppState)
                }
                Mode.Scan -> {
                    ScanScreen(
                        cameraProviderFuture,
                        dbName,
                        transmitCallback,
                        setAppState,
                        collection,
                        frames,
                        lifecycleOwner
                    )
                }
                Mode.TX -> {
                    TXScreen(collection, transmitCallback, setAppState)
                }
            }
        }
    }
}

enum class Mode {
    Scan,
    Address,
    TX;
}

class Signer : SignByCompanion {
    override fun makeSignature(data: List<UByte>): List<UByte> {
        val kpg = KeyPairGenerator.getInstance(
            KeyProperties.KEY_ALGORITHM_EC,
            "AndroidKeyStore"
        )
        val parameterSpec: KeyGenParameterSpec = KeyGenParameterSpec.Builder(
            "AndroidKeyStore",
            KeyProperties.PURPOSE_SIGN
        ).run {
            setDigests(KeyProperties.DIGEST_SHA256, KeyProperties.DIGEST_SHA512)
            build()
        }

        kpg.initialize(parameterSpec)

        val kp = kpg.generateKeyPair()
        val s = Signature.getInstance("SHA256withECDSA").apply {
            initSign(kp.private)
            update(data.toUByteArray().toByteArray())
        }
        val signature: ByteArray = s.sign()
        return signature.toUByteArray().toList() + kp.public.encoded.toUByteArray() // TODO
    }
}
