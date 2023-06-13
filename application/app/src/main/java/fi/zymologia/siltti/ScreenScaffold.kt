package fi.zymologia.siltti

import android.Manifest
import android.util.Log
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import fi.zymologia.siltti.screens.ScanScreen
import fi.zymologia.siltti.screens.TXScreen
import fi.zymologia.siltti.uniffi.*
import java.security.KeyStore
import java.security.Signature

val REQUIRED_PERMISSIONS = arrayOf(Manifest.permission.CAMERA)
val REQUEST_CODE_PERMISSIONS = 10

/**
 * Main scanner screen. One of navigation roots.
 */
@Composable
fun ScreenScaffold(
    dbName: String,
    count: State<Int?>,
    counterReset: () -> Unit,
    transmitCallback: (Action?) -> Unit
) {
    var appState by remember { mutableStateOf(Mode.Scan) }

    val setAppState = { mode: Mode -> appState = mode }

    Column(
        Modifier
            .fillMaxSize()
    ) {
        Box(
            Modifier.padding(8.dp)
        ) {
            // TODO: use all the cores needed to make this smooth
            when (appState) {
                Mode.Address -> {
                    fi.zymologia.siltti.screens.NewAddress(
                        setAppState,
                        transmitCallback,
                        dbName
                    )
                }
                Mode.Scan -> {
                    ScanScreen(
                        dbName,
                        transmitCallback,
                        setAppState
                    )
                }
                Mode.TX -> {
                    TXScreen(transmitCallback, setAppState, count, counterReset)
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
    @OptIn(ExperimentalUnsignedTypes::class)
    override fun makeSignature(data: List<UByte>): List<UByte> {
        val ks = KeyStore.getInstance("AndroidKeyStore").apply {
            load(null)
        }

        val ke = ks.getEntry("AndroidKeyStore", null)

        if (ke !is KeyStore.PrivateKeyEntry) {
            Log.w("", "Not an instance of a PrivateKeyEntry")
            return emptyList()
        }

        val s = Signature.getInstance("SHA256withECDSA").apply {
            initSign(ke.privateKey)
            update(data.toUByteArray().toByteArray())
        }

        val signature: ByteArray = s.sign()
        return signature.toUByteArray().toList()
    }

    override fun exportPublicKey(): List<UByte> {
        val ks = KeyStore.getInstance("AndroidKeyStore").apply {
            load(null)
        }

        val ke = ks.getEntry("AndroidKeyStore", null)

        if (ke !is KeyStore.PrivateKeyEntry) {
            Log.w("", "Not an instance of a PrivateKeyEntry")
            return emptyList()
        }
        return ke.certificate.publicKey.encoded.toUByteArray().toList()
    }
}
