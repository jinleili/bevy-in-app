package name.jinleili.bevy

import android.view.Surface

class RustBridge {
    init {
        System.loadLibrary("bevy_in_app")
    }

    external fun create_bevy_app(surface: Surface, scale_factor: Float): Long
    external fun enter_frame(bevy_app: Long)
    external fun device_motion(bevy_app: Long, x: Float, y: Float, z: Float)
    external fun release_bevy_app(bevy_app: Long)
}