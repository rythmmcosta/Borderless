# Flutter internals reference Play Core classes not bundled in direct APK distributions.
# These dontwarn rules tell R8 to suppress the missing-class error.
-dontwarn com.google.android.play.core.**
-dontwarn io.flutter.app.FlutterPlayStoreSplitApplication

# Keep Flutter entry points
-keep class io.flutter.app.** { *; }
-keep class io.flutter.plugin.** { *; }
-keep class io.flutter.util.** { *; }
-keep class io.flutter.view.** { *; }
-keep class io.flutter.** { *; }
-keep class io.flutter.plugins.** { *; }
