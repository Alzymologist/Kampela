package fi.zymologia.nfctester

import android.app.PendingIntent
import android.content.Intent
import android.content.IntentFilter
import android.nfc.NdefMessage
import android.nfc.NdefRecord
import android.nfc.NfcAdapter
import android.nfc.NfcAdapter.ACTION_TAG_DISCOVERED
import android.nfc.NfcAdapter.EXTRA_TAG
import android.nfc.Tag
import android.nfc.tech.Ndef
import android.os.Build
import android.os.Bundle
import android.util.Log
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import fi.zymologia.nfctester.ui.theme.NFCTesterTheme

class MainActivity : ComponentActivity() {
    private var nfcAdapter: NfcAdapter? = null
    private var pendingIntent: PendingIntent? = null
    private val payload by viewModels<Paload>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        nfcAdapter = NfcAdapter.getDefaultAdapter(this)
        if (nfcAdapter == null) {
            Toast.makeText(this, "NFC is not available", Toast.LENGTH_LONG).show()
            finish()
            return
        } else {
            Log.d("NFC support status", nfcAdapter!!.isEnabled.toString())
        }
        Log.d("NFC enabled", nfcAdapter?.isEnabled.toString())
        val intent = Intent(this, javaClass).apply {
            addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP)
        }
        pendingIntent = PendingIntent.getActivity(
            this,
            0,
            intent,
            PendingIntent.FLAG_MUTABLE
        )

        setContent {
            NFCTesterTheme {
                val tnf = payload.tnf.observeAsState()
                val type = payload.recordType.observeAsState()
                val id = payload.recordId.observeAsState()
                val pl = payload.paload.observeAsState()
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colors.background
                ) {
                    Greeting(
                        tnf,
                        type,
                        id,
                        pl,
                        payload::setTNF,
                        payload::setType,
                        payload::setId,
                        payload::setPayload
                    )
                }
            }
        }
    }

    override fun onResume() {
        super.onResume()
        // An Intent to start your current Activity. Flag to singleTop
        // to imply that it should only be delivered to the current
        // instance rather than starting a new instance of the Activity.
        // Define your filters and desired technology types
        val filters = arrayOf(IntentFilter(ACTION_TAG_DISCOVERED))
        // val techTypes = arrayOf(arrayOf(NfcA::class.java.name, Ndef::class.java.name))

        // And enable your Activity to receive NFC events. Note that there
        // is no need to manually disable dispatch in onPause() as the system
        // very strictly performs this for you. You only need to disable
        // dispatch if you don't want to receive tags while resumed.
        nfcAdapter!!.enableForegroundDispatch(
            this,
            pendingIntent,
            filters,
            null
        )
    }

    override fun onPause() {
        super.onPause()
        nfcAdapter!!.disableForegroundDispatch(this)
    }

    // TODO: move to bg thread
    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        if (NfcAdapter.ACTION_TAG_DISCOVERED == intent.action) {
            val tag = if (Build.VERSION.SDK_INT >= 33) {
                intent.getParcelableExtra(EXTRA_TAG, Tag::class.java)
            } else {
                intent.getParcelableExtra(EXTRA_TAG)
            }
            Log.d("NFC tag", tag.toString())

            Ndef.get(tag)?.let { ndef ->
                ndef.connect()
                Log.d("max length", ndef.maxSize.toString())
                try {
                    val ndefRecord = NdefRecord(
                        payload.tnf.value ?: 0,
                        payload.recordType.value,
                        payload.recordId.value,
                        payload.paload.value
                    )
                    Log.d("Record formed", "1")
                    val ndefMessage = NdefMessage(ndefRecord)
                    Log.d("Message formed", "1")
                    ndef.writeNdefMessage(ndefMessage)
                    Log.d("Message sent", "1")
                } catch (e: java.lang.Exception) {
                    Log.d("NFC link crashed", e.message ?: "unknown")
                }
                Log.d("NFC TX", "done")
                ndef.close()
            }
        }
        Log.d("NFC", "intent processed")
    }
}

@Composable
fun Greeting(
    tnf: State<Short?>,
    type: State<ByteArray?>,
    id: State<ByteArray?>,
    payload: State<ByteArray?>,
    setTnf: (String) -> Unit,
    setType: (String) -> Unit,
    setId: (String) -> Unit,
    setPayload: (String) -> Unit
) {
    Column() {
        Text(text = "Terve!")

        TextField(
            value = tnf.value.toString(),
            onValueChange = setTnf,
            label = { Text("tnf") }
        )
        TextField(
            value = type.value?.encodeHex() ?: "",
            onValueChange = setType,
            label = { Text("type") }
        )
        TextField(
            value = id.value?.encodeHex() ?: "",
            onValueChange = setId,
            label = { Text("value") }
        )
        TextField(
            value = payload.value?.encodeHex() ?: "",
            onValueChange = setPayload,
            label = { Text("payload") }
        )
    }
}

class Paload : ViewModel() {
    private var _tnf = MutableLiveData<Short>(5)
    private var _recordType = MutableLiveData<ByteArray>(null)
    private var _recordId = MutableLiveData<ByteArray>(null)
    private var _paload = MutableLiveData<ByteArray>(byteArrayOf(1, 2, 3, 4))
    val tnf: LiveData<Short?> = _tnf
    val recordType: LiveData<ByteArray> = _recordType
    val recordId: LiveData<ByteArray> = _recordId
    val paload: LiveData<ByteArray> = _paload

    fun setTNF(a: String) {
        try {
            _tnf.value = a.toShort()
        } catch (_: java.lang.Exception) {
            Log.d("payload manager", "tnf not parsed")
        }
    }

    fun setType(a: String) {
        try {
            _recordType.value = a.decodeHex()
        } catch (_: java.lang.Exception) {
            Log.d("payload manager", "type not parsed")
        }
    }

    fun setId(a: String) {
        try {
            _recordId.value = a.decodeHex()
        } catch (_: java.lang.Exception) {
            Log.d("payload manager", "id not parsed")
        }
    }

    fun setPayload(a: String) {
        Log.d("input:", a)
        try {
            _paload.value = a.decodeHex()
        } catch (_: java.lang.Exception) {
            Log.d("payload manager", "payload not parsed")
        }
    }
}

/**
 * Decodes from hex string into number array
 */
fun String.decodeHex(): ByteArray {
    return chunked(2).map { it.toInt(16).toByte() }.toByteArray()
}

/**
 * Encodes number array into string
 */
fun ByteArray.encodeHex(): String {
    return this.joinToString(separator = "") { byte -> "%02x".format(byte) }
}
