// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkSnapshot")]
    pub struct Snapshot(Object<ffi::GdkSnapshot, ffi::GdkSnapshotClass>);

    match fn {
        type_ => || ffi::gdk_snapshot_get_type(),
    }
}

impl Snapshot {
    pub const NONE: Option<&'static Snapshot> = None;
}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Snapshot")
    }
}
