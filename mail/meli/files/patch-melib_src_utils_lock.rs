--- melib/src/utils/lock.rs.orig	2025-05-09 22:02:20 UTC
+++ melib/src/utils/lock.rs
@@ -50,7 +50,6 @@ cfg_if::cfg_if! {
 cfg_if::cfg_if! {
     if #[cfg(any(
             target_os = "android",
-            target_os = "freebsd",
             target_os = "illumos",
             target_os = "ios",
             target_os = "linux",
