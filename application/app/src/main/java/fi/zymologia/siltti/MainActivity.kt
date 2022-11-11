package fi.zymologia.siltti

import android.app.PendingIntent
import android.app.PendingIntent.FLAG_MUTABLE
import android.content.Intent
import android.content.IntentFilter
import android.nfc.NfcAdapter
import android.nfc.NfcAdapter.*
import android.nfc.Tag
import android.nfc.tech.*
import android.os.Bundle
import android.util.Log
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import fi.zymologia.siltti.ui.theme.SilttiTheme

class MainActivity : ComponentActivity() {
    private var nfcAdapter: NfcAdapter? = null
    private var pendingIntent: PendingIntent? = null
    private var transmitData: List<ByteArray> = emptyList()
    private val packagesSent by viewModels<PackagesSent>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        System.loadLibrary("siltti")
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
            FLAG_MUTABLE
        )

        val dbName = this.filesDir.toString()
        setContent {
            SilttiTheme {
                // TODO: this does not work as TX is in main thread
                val count = packagesSent.count.observeAsState()
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colors.background
                ) {
                    ScreenScaffold(
                        dbName,
                        count
                    ) { newData: List<ByteArray> ->
                        transmitData = newData
                    }
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
            packagesSent.reset()
            val tag = intent.getParcelableExtra(EXTRA_TAG, Tag::class.java)
            Log.d("NFC tag", tag.toString())
            val tech = IsoDep.get(tag)
            tech.connect()
            Log.d("max length", tech.maxTransceiveLength.toString())
            try {
                while (tech.isConnected) {
                    tech.transceive(transmitData.random())
                    packagesSent.inc()
                    Log.d("sent: ", packagesSent.count.value.toString())
                }
            } catch (e: java.lang.Exception) {
                Log.d("NFC link crashed", e.message ?: "unknown")
            }
            Log.d("NFC TX", "done")
            tech.close()
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
