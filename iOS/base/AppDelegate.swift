//
//  AppDelegate.swift
//  bevy_in_iOS
//
//  Created by Jinlei Li on 2022/12/20.
//

import UIKit

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?
        
    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        window = UIWindow(frame: UIScreen.main.bounds)
        let mainStroryBoard = UIStoryboard(name: "Main", bundle: nil)
        window?.rootViewController = mainStroryBoard.instantiateInitialViewController()

        window?.makeKeyAndVisible()
        return true
    }

}

