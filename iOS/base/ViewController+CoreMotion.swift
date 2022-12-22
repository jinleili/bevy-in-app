//
//  ViewController+CoreMotion.swift
//  bevy_in_iOS
//
//  Created by 李金磊 on 2022/12/22.
//

import CoreMotion

extension ViewController {
    func startGyroUpdates() {
        if !motionManager.isGyroAvailable {
            print("陀螺仪不可用")
            return
        }
        if motionManager.isGyroActive {
            return
        }
                
        motionManager.startGyroUpdates(to: OperationQueue.init()) { gyroData, error in
            guard let gyroData = gyroData else {
                print("startGyroUpdates error: \(error!)")
                return
            }
            // 获取陀螺仪数据
            self.rotationRate = gyroData.rotationRate
        }
    }
    
    func stopGyroUpdates() {
        motionManager.stopGyroUpdates()
    }
    
    func startAccelerometerUpdates() {
        if !motionManager.isAccelerometerAvailable {
            print("加速计不可用")
            return
        }
        if motionManager.isAccelerometerActive {
            return
        }
        motionManager.startAccelerometerUpdates(to: OperationQueue.init()) { accelerometerData, error in
            guard let accelerometerData = accelerometerData else {
                print("startAccelerometerUpdates error: \(error!)")
                return
            }
            // 获取加速计数据
            print("\(accelerometerData)")
        }
    }
    
    func stopAccelerometerUpdates() {
        motionManager.stopAccelerometerUpdates()
    }
    
    func startDeviceMotionUpdates() {
        motionManager.startDeviceMotionUpdates(to: OperationQueue.init()) { deviceMotion, error in
            guard let deviceMotion = deviceMotion else {
                print("startDeviceMotionUpdates error: \(error!)")
                return
            }
            // Gravity 获取重力值在各个方向上的分量，根据这个就可以获得手机的空间位置，倾斜角度等
            self.gravity = deviceMotion.gravity
        }
    }
    
    func stopDeviceMotionUpdates() {
        motionManager.stopDeviceMotionUpdates()
    }
}
