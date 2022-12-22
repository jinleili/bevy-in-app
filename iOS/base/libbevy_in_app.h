//
//  libbevy_in_app.h
//  bevy_in_iOS
//
//  Created by Jinlei Li on 2022/12/20.
//

#ifndef libbevy_in_app_h
#define libbevy_in_app_h

#include <stdint.h>

// 这个不透明结构体用来指代 Rust 端的 Bevy App 对象
struct bevy_app;

struct bevy_app *create_bevy_app(void *view, int maximum_frames, float scale_factor);

void enter_frame(struct bevy_app *app);
void release_bevy_app(struct bevy_app *app);

void touch_started(struct bevy_app *app, float x, float y);
void touch_moved(struct bevy_app *app, float x, float y);
void touch_ended(struct bevy_app *app, float x, float y);
void touch_cancelled(struct bevy_app *app, float x, float y);

// Gyroscope, Accelerometer, DeviceMotion
void gyroscope_motion(struct bevy_app *app, float x, float y, float z);
void accelerometer_motion(struct bevy_app *app, float x, float y, float z);
void device_motion(struct bevy_app *app, float x, float y, float z);

#endif /* libbevy_in_app_h */
