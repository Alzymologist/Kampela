package fi.zymologia.siltti

import android.app.PendingIntent
import android.app.PendingIntent.FLAG_MUTABLE
import android.content.Intent
import android.content.IntentFilter
import android.nfc.NfcAdapter
import android.nfc.NfcAdapter.*
import android.nfc.Tag
import android.nfc.tech.*
import android.os.Build
import android.os.Bundle
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Log
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import fi.zymologia.siltti.ui.theme.SilttiTheme
import java.security.KeyPairGenerator
import java.security.KeyStore

class MainActivity : ComponentActivity() {
    private var nfcAdapter: NfcAdapter? = null
    private var pendingIntent: PendingIntent? = null
    private var transmitData: List<ByteArray> = emptyList()
    private val packagesSent by viewModels<PackagesSent>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        System.loadLibrary("siltti")

        val ks = KeyStore.getInstance("AndroidKeyStore").apply {
            load(null)
        }

        if (!ks.aliases().toList().contains("AndroidKeyStore")) {
            val kpg = KeyPairGenerator.getInstance(
                KeyProperties.KEY_ALGORITHM_EC,
                "AndroidKeyStore",
            )
            val parameterSpec: KeyGenParameterSpec = KeyGenParameterSpec.Builder(
                "AndroidKeyStore",
                KeyProperties.PURPOSE_SIGN,
            ).run {
                setDigests(KeyProperties.DIGEST_SHA256, KeyProperties.DIGEST_SHA512)
                build()
            }
            kpg.initialize(parameterSpec)
            kpg.generateKeyPair()
        }

        /* thing to view signature
        // START
        val data = listOf<UByte>(1u, 2u, 3u, 4u)

        val s = if (ks.aliases().toList().contains("AndroidKeyStore")) {
            val ke = ks.getEntry("AndroidKeyStore", null)

            if (ke !is KeyStore.PrivateKeyEntry) {
                Log.w("", "Not an instance of a PrivateKeyEntry")
                return
            }
            Log.d("test", ke.certificate.publicKey.encoded.toUByteArray().toList().toString())
            Signature.getInstance("SHA256withECDSA").apply {
                initSign(ke.privateKey)
                update(data.toUByteArray().toByteArray())
            }
        } else {
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
            Log.d("test", kp.public.encoded.toUByteArray().toList().toString())
            Signature.getInstance("SHA256withECDSA").apply {
                initSign(kp.private)
                update(data.toUByteArray().toByteArray())
            }
        }
        val signature: ByteArray = s.sign()
        Log.d("testsign", signature.toUByteArray().toList().toString())

        // END
        */

        nfcAdapter = getDefaultAdapter(this)
        Log.d("blem start" , "999");
        if (nfcAdapter == null) {
            Toast.makeText(this, "NFC is not available", Toast.LENGTH_LONG).show()
            finish()
            return
        } else {
            Log.d("NFC support status", nfcAdapter!!.isEnabled.toString())
        }
        Log.d("NFC enabled", nfcAdapter?.isEnabled.toString())
        /*
        val intent = Intent(this, javaClass).apply {
            addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP)
        }
        pendingIntent = PendingIntent.getActivity(
            this,
            0,
            intent,
            PendingIntent.FLAG_MUTABLE,
        )
*/
        val dbName = this.filesDir.toString()
        setContent {
            SilttiTheme {
                // TODO: this does not work as TX is in main thread
                val count = packagesSent.count.observeAsState()
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colors.background,
                ) {
                    ScreenScaffold(
                        dbName,
                        count,
                    ) { newData: List<ByteArray> ->
                        transmitData = newData
                    }
                }
            }
        }
    }

    override fun onResume() {
        super.onResume()
        Log.d("resume blem", "1")
        val intent = Intent(this, javaClass).apply {
            addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP)
        }
        pendingIntent = PendingIntent.getActivity(
            this,
            0,
            intent,
            PendingIntent.FLAG_MUTABLE,
        )
        // An Intent to start your current Activity. Flag to singleTop
        // to imply that it should only be delivered to the current
        // instance rather than starting a new instance of the Activity.
        // Define your filters and desired technology types
        val filters = arrayOf(IntentFilter(NfcAdapter.ACTION_TAG_DISCOVERED))
        val techTypes = arrayOf(arrayOf<String>(NfcA::class.java.name, Ndef::class.java.name, IsoDep::class.java.name))

        // And enable your Activity to receive NFC events. Note that there
        // is no need to manually disable dispatch in onPause() as the system
        // very strictly performs this for you. You only need to disable
        // dispatch if you don't want to receive tags while resumed.
        nfcAdapter!!.enableForegroundDispatch(
            this,
            pendingIntent,
            filters,
            techTypes,
        )
    }

    override fun onPause() {
        super.onPause()
        Log.d("pause blem", "1")
        nfcAdapter!!.disableForegroundDispatch(this)
    }

    // TODO: move to bg thread
    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        Log.d("NFC blem", "1")
        if (ACTION_TAG_DISCOVERED == intent.action) {
            packagesSent.reset()
            val tag = if (Build.VERSION.SDK_INT >= 33) {
                intent.getParcelableExtra(EXTRA_TAG, Tag::class.java)
            } else {
                intent.getParcelableExtra(EXTRA_TAG)
            }
            Log.d("NFC tag", tag.toString())

            IsoDep.get(tag)?.let { tech ->
                Log.d("max length", tech.maxTransceiveLength.toString())
                try {
                    Log.d("blem status", "connecting")
                    while (true) {
                        if (transmitData.size < (packagesSent.count.value ?: 0)) {
                            packagesSent.reset()
                        }
                        Log.d("sending:", transmitData.getOrNull(packagesSent.count.value ?: 0)?.contentToString() ?: "empty")
                        tech.connect()
                        tech.transceive(transmitData.getOrNull(packagesSent.count.value ?: 0))
                        tech.close()
                        packagesSent.inc()
                        Log.d("sent: ", packagesSent.count.value.toString())
                    }
                } catch (e: java.lang.Exception) {
                    Log.d("NFC link crashed", e.message ?: "unknown")
                }
                Log.d("NFC TX", "done")
            }
            packagesSent.disable()
        }

        Log.d("NFC", "intent processed")
    }
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    val count = PackagesSent().count.observeAsState()
    SilttiTheme {
        ScreenScaffold("stub", count, {})
    }
}
