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
@@ -81,6 +80,8 @@ where
                        * at the location specified by l_whence and l_start through to the end of
                        * file, no matter how large the file grows. */
             l_pid: 0, /* "By contrast with traditional record locks, the l_pid field of that structure must be set to zero when using the commands described below." */
+            #[cfg(target_os = "freebsd")]
+            l_sysid: 0,
         };
         let ret_val = unsafe { libc::fcntl(fd, F_SETLK, &mut flock) };
         log::debug!("dropped unix fd lock for mbox fd {}, got {}", fd, ret_val);
