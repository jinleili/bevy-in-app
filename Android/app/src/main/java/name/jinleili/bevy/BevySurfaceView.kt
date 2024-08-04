package name.jinleili.bevy

import android.content.Context
import android.graphics.Canvas
import android.hardware.*
import android.util.AttributeSet
import android.view.SurfaceHolder
import android.view.SurfaceView

class BevySurfaceView : SurfaceView, SurfaceHolder.Callback2 {
    private var rustBrige = RustBridge()
    private var bevy_app: Long = Long.MAX_VALUE
    private var appPreparationCompleted: Int = 0;
    private var ndk_inited = false
    private var idx: Int = 0
    private var sensorManager: SensorManager? = null
    private var mSensor: Sensor? = null
    private var sensorValues: FloatArray = FloatArray(3)

    constructor(context: Context) : super(context) {
        sensorManager = context.getSystemService(Context.SENSOR_SERVICE) as SensorManager
        mSensor = sensorManager?.getDefaultSensor(Sensor.TYPE_GRAVITY)
    }
    constructor(context: Context, attrs: AttributeSet) : super(context, attrs) {
    }
    constructor(context: Context, attrs: AttributeSet, defStyle: Int) : super(
        context,
        attrs,
        defStyle
    ) {
    }

    init {
        // 将当前类设置为 SurfaceHolder 的回调接口代理
        holder.addCallback(this)
    }

    override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
    }

    // 绘制表面被创建后，创建/重新创建 Bevy App
    override fun surfaceCreated(holder: SurfaceHolder) {
        holder.let { h ->
            if (!ndk_inited) {
                ndk_inited = true
                rustBrige.init_ndk_context(this.context)
            }

            if (bevy_app == Long.MAX_VALUE) {
                // Get the screen's density scale
                val scaleFactor: Float = resources.displayMetrics.density
                bevy_app = rustBrige.create_bevy_app(this.context.assets, h.surface, scaleFactor)
            }

            // SurfaceView 默认不会自动开始绘制，setWillNotDraw(false) 用于通知 App 已经准备好开始绘制了。
            setWillNotDraw(false)

            var sensorEventListener = object : SensorEventListener {
                override fun onSensorChanged(event: SensorEvent?) {
                    if (event != null) {
                        sensorValues = event.values
                    }
                }

                override fun onAccuracyChanged(sensor: Sensor?, accuracy: Int) {
                }
            }
            mSensor?.also { sensor ->
                sensorManager?.registerListener(sensorEventListener, sensor, SensorManager.SENSOR_DELAY_GAME)
            }
        }
    }

    // 绘制表面被销毁后，也销毁 Bevy 中的 Android window
    override fun surfaceDestroyed(holder: SurfaceHolder) {
        if (bevy_app != Long.MAX_VALUE) {
            rustBrige.release_bevy_app(bevy_app)
            bevy_app = Long.MAX_VALUE
            appPreparationCompleted = 0;
        }
    }

    override fun surfaceRedrawNeeded(holder: SurfaceHolder) {
    }

    // API Level 26+
//    override fun surfaceRedrawNeededAsync(holder: SurfaceHolder, drawingFinished: Runnable) {
//        super.surfaceRedrawNeededAsync(holder, drawingFinished)
//    }

    override fun draw(canvas: Canvas?) {
        super.draw(canvas)
        if (bevy_app == Long.MAX_VALUE) {
           return
        }
        if (appPreparationCompleted == 0) {
            appPreparationCompleted = rustBrige.is_preparation_completed(bevy_app);
        } else {
            rustBrige.device_motion(bevy_app, sensorValues[0], sensorValues[1], sensorValues[2])
            rustBrige.enter_frame(bevy_app)
        }
        // invalidate() 函数通知通知 App，在下一个 UI 刷新周期重新调用 draw() 函数
        invalidate()
    }

    fun changeExample(index: Int) {
        if (bevy_app != Long.MAX_VALUE && this.idx != index) {
            this.idx = index
        }
    }
}