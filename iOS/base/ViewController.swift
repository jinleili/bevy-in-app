//
//  ViewController.swift
//  bevy_in_iOS
//
//  Created by Jinlei Li on 2022/12/20.
//

import UIKit
import CoreMotion

class ViewController: UIViewController {
    @IBOutlet var metalV: MetalView!
    var bevyApp: OpaquePointer?
    
    var rotationRate: CMRotationRate?
    var gravity: CMAcceleration?
    lazy var motionManager: CMMotionManager = {
        let manager = CMMotionManager.init()
        manager.gyroUpdateInterval = 0.032
        manager.accelerometerUpdateInterval = 0.032
        manager.deviceMotionUpdateInterval = 0.032
        return manager
    }()
    
    lazy var displayLink: CADisplayLink = {
        CADisplayLink.init(target: self, selector: #selector(enterFrame))
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
       
        self.displayLink.add(to: .current, forMode: .default)
        self.displayLink.isPaused = true
    }
    
    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        self.view.backgroundColor = .white
        if bevyApp == nil {
            self.createBevyApp()
        }
        self.displayLink.isPaused = false
        self.startDeviceMotionUpdates()
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        displayLink.isPaused = true
        self.stopDeviceMotionUpdates()
    }
    
    func createBevyApp() {
        let viewPointer = Unmanaged.passUnretained(self.metalV).toOpaque()
        let maximumFrames = Int32(UIScreen.main.maximumFramesPerSecond)
        
        bevyApp = create_bevy_app(viewPointer, maximumFrames, Float(UIScreen.main.nativeScale))
    }
    
    @IBAction func recreateBevyApp() {
        if let bevy = bevyApp {
            displayLink.isPaused = true
            release_bevy_app(bevy)
        }
        // NOTE: bevy 0.17 recreate app will cause panic:
        // thread '<unnamed>' panicked at /Users/lijinlei/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/
        // bevy_render-0.17.1/src/render_resource/bind_group_layout.rs:73:10:
        // init_empty_bind_group_layout was called more than once: BindGroupLayout { id: BindGroupLayoutId(513), 
        // value: WgpuWrapper(BindGroupLayout { inner: Core(CoreBindGroupLayout { context: ContextWgpuCore { type: "Native" }, id: Id(241,1) }) }) }
        
        createBevyApp()
        displayLink.isPaused = false
    }
    
    @objc func enterFrame() {
        guard let bevy = self.bevyApp else {
            return
        }
        // call rust
        if let gravity = gravity {
            device_motion(bevy, Float(gravity.x), Float(gravity.y), Float(gravity.z))
        }
        enter_frame(bevy)
    }
    
    // MARK: touch
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let bevy = self.bevyApp, let touch: UITouch = touches.first {
            let location = touch.location(in: self.metalV);
            touch_started(bevy, Float(location.x), Float(location.y));
        }
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let bevy = self.bevyApp, let touch: UITouch = touches.first {
            let location = touch.location(in: self.metalV);
            touch_moved(bevy, Float(location.x), Float(location.y));
        }
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let bevy = self.bevyApp, let touch: UITouch = touches.first {
            let location = touch.location(in: self.metalV);
            touch_ended(bevy, Float(location.x), Float(location.y));
        }
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        if let bevy = self.bevyApp, let touch: UITouch = touches.first {
            let location = touch.location(in: self.metalV);
            touch_cancelled(bevy, Float(location.x), Float(location.y));
        }
    }

    deinit {
        if let bevy = bevyApp {
            release_bevy_app(bevy)
        }
    }
}
