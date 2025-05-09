--- melib/src/utils/connections.rs.orig	2025-05-09 22:21:45 UTC
+++ melib/src/utils/connections.rs
@@ -433,6 +433,7 @@ impl Connection {
                 #[cfg(any(
                     target_os = "openbsd",
                     target_os = "netbsd",
+                    target_os = "freebsd",
                     target_os = "haiku",
                     target_os = "macos",
                     target_os = "ios"
@@ -447,6 +448,7 @@ impl Connection {
                 #[cfg(not(any(
                     target_os = "openbsd",
                     target_os = "netbsd",
+                    target_os = "freebsd",
                     target_os = "haiku",
                     target_os = "macos",
                     target_os = "ios"
